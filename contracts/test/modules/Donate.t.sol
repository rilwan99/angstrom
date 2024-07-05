// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {BaseTest} from "../_helpers/BaseTest.sol";
import {Donate} from "../../src/modules/Donate.sol";
import {TickRewards} from "../../src/types/TickRewards.sol";
import {ConversionLib} from "../../src/libraries/ConversionLib.sol";

import {PoolId, PoolIdLibrary} from "v4-core/src/types/PoolId.sol";
import {TICK_SPACING} from "../../src/Constants.sol";
import {UniV4Inspector} from "../_mocks/UniV4Inspector.sol";
import {MockERC20} from "../_mocks/MockERC20.sol";
import {TickMath} from "v4-core/src/libraries/TickMath.sol";

import {PoolGate} from "../_helpers/PoolGate.sol";
import {MockConversionLib} from "../_helpers/MockConversionLib.sol";
import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";

import {PRNG} from "../_helpers/PRNG.sol";
import {FormatLib} from "../_helpers/FormatLib.sol";
import {LibString} from "solady/src/utils/LibString.sol";
import {console2 as console} from "forge-std/console2.sol";

/// @author philogy <https://github.com/philogy>
contract DonateTest is BaseTest, Donate {
    using TickMath for int24;

    using MockConversionLib for *;
    using FormatLib for *;
    using LibString for *;
    using FixedPointMathLib for *;

    uint160 constant ONE_SQRTX96 = 1 << 96;

    address owner = makeAddr("owner");
    UniV4Inspector uniV4;
    PoolGate gate;
    address asset0 = address(new MockERC20());
    address asset1 = address(new MockERC20());

    TickRewards rewards;
    PoolId mainId;

    function setUp() public {
        if (asset1 < asset0) (asset0, asset1) = (asset1, asset0);
        mainId = PoolIdLibrary.toId(ConversionLib.toPoolKey(address(0), asset0, asset1));

        vm.prank(owner);
        uniV4 = new UniV4Inspector();
        gate = new PoolGate(address(uniV4));
    }

    struct Position {
        uint128 liq;
        int24 lowerTick;
        int24 upperTick;
    }

    function test_donate(int256 input_startCompressedTick, uint256 seed) public {
        // Setup pool.
        int24 poolTick = int24(bound(input_startCompressedTick, -2500, 2500)) * TICK_SPACING;
        gate.initializePool(asset0, asset1, poolTick.getSqrtPriceAtTick());

        PRNG memory rng = PRNG(seed);

        // Select donate range.
        int8 ticksOff = int8(rng.randbool() ? rng.randint(-64, -1) : rng.randint(1, 64));
        uint256 totalTicks;
        unchecked {
            totalTicks = ticksOff < 0 ? uint8(-ticksOff) : uint8(ticksOff);
        }

        uint256 SPREAD_POSITIONS = 4;

        // Generate random positions and compute total liquidity for later.
        Position[] memory positions = new Position[](SPREAD_POSITIONS);
        uint256[] memory totalLiqAtIndex = new uint256[](totalTicks);
        int24 minTick = ticksOff < 0 ? poolTick + TICK_SPACING * ticksOff : poolTick + TICK_SPACING;

        for (uint256 i = 0; i < positions.length; i++) {
            Position memory p = positions[i];
            // `lo` a bit biased towards the middle
            uint256 lo = rng.randuint(0, totalTicks);
            uint256 hi = rng.randuint(lo + 1, totalTicks + 1);
            p.liq = u128(rng.randuint(0.1e21, 1.0e21));

            for (uint256 j = lo; j < hi - 1; j++) {
                totalLiqAtIndex[j] += p.liq;
            }

            p.lowerTick = minTick + TICK_SPACING * i24(lo);
            p.upperTick = minTick + TICK_SPACING * i24(hi);

            gate.addLiquidity(asset0, asset1, p.lowerTick, p.upperTick, p.liq);
        }

        {
            // Sanity check that final tick is left empty.
            uint256 currentLiq = _getCurrentLiquidity(mainId);
            assertEq(currentLiq, 0);
        }

        uint256 donateTicks;
        {
            uint256 totalInitializedTicks = 0;
            for (uint256 i = 0; i < totalLiqAtIndex.length; i++) {
                if (totalLiqAtIndex[i] > 0) totalInitializedTicks++;
            }
            assertGe(totalInitializedTicks, 1, "Not even one tick was initialized.");
            donateTicks = rng.randuint(1, totalInitializedTicks);
        }

        // Generate random amounts to be donated.
        uint256[] memory amounts = new uint256[](donateTicks);
        for (uint256 i = 0; i < amounts.length; i++) {
            amounts[i] = rng.randchoice(0.2e18, 0, rng.randuint(10.0e18));
        }

        console.log("totalLiqAtIndex:");

        // Perform donate
        uint256 total;
        if (ticksOff < 0) {
            int24 startTick = poolTick - i24(donateTicks - 1) * TICK_SPACING;
            uint128 startLiquidity;
            uint256 j = donateTicks;
            uint256 k = totalTicks;
            while (k > 0) {
                if (totalLiqAtIndex[--k] != 0) j--;
                if (j == 0) break;
            }

            for (uint256 i = 0; i < totalLiqAtIndex.length; i++) {
                uint256 liq = totalLiqAtIndex[i];
                int24 tick = minTick + i24(i) * TICK_SPACING;
                if (i == k) {
                    console.log("  %s: %s <", tick.toString(), liq);
                } else {
                    console.log("  %s: %s", tick.toString(), liq);
                }
            }

            console.log("amounts.length (%s): %s", donateTicks, amounts.length);
            total = _donate(rewards, mainId, minTick + i24(k) * TICK_SPACING, u128(totalLiqAtIndex[k]), amounts);
        } else {}
    }

    function test_oneOff_donate() public {
        gate.initializePool(asset0, asset1, ONE_SQRTX96);
        int24 current = _getCurrentTick(mainId);
        uint256 fixedLiquidity = 1e21;
        int24 startTick = current - TICK_SPACING * 4;

        int24 tick = startTick;
        for (uint256 i = 0; i < 9; i++) {
            gate.addLiquidity(asset0, asset1, tick, (tick += TICK_SPACING), fixedLiquidity);
        }

        tick = startTick;
        for (uint256 i = 0; i < 9; i++) {
            uint256 growthInside = rewards.getGrowthInside(current, tick, (tick + TICK_SPACING));
            uint256 fees = growthInside.mulWad(fixedLiquidity);
            console.log("%s: %s", tick.toString(), fees.formatDecimals());
            tick += TICK_SPACING;
        }

        uint256[] memory amounts = new uint256[](4);
        amounts[0] = 1e18;
        amounts[1] = 1e18;
        amounts[2] = 1.3e18;
        amounts[3] = 1e18;

        console.log("\n  DONATE");
        uint256 total = _donate(rewards, mainId, current + 4 * TICK_SPACING, uint128(fixedLiquidity), amounts);
        console.log("\n  total: %s\n", total.formatDecimals());

        tick = startTick;
        for (uint256 i = 0; i < 9; i++) {
            uint256 growthInside = rewards.getGrowthInside(current, tick, (tick + TICK_SPACING));
            uint256 fees = growthInside.mulWad(fixedLiquidity);
            console.log(
                "%s: %s [%s]\n",
                tick.toString(),
                fees.formatDecimals(),
                rewards.tickGrowthOutside[tick].formatDecimals()
            );
            tick += TICK_SPACING;
        }

        uint256 growthInside = rewards.getGrowthInside(current, startTick, current);
        uint256 fees = growthInside.mulWad(fixedLiquidity);
        console.log("\n  [%s, %s): %s", startTick.toString(), current.toString(), fees.formatDecimals());
    }

    function _getPoolBitmapInfo(PoolId id, int16 wordPos) internal view override returns (uint256) {
        return uniV4.getBitmapWord(id, wordPos);
    }

    function _getNetTickLiquidity(PoolId id, int24 tick) internal view override returns (int128 liquidityNet) {
        (, liquidityNet,,) = uniV4.getTick(id, tick);
    }

    function _getCurrentLiquidity(PoolId id) internal view override returns (uint128 liquidity) {
        (,,,,,, liquidity) = uniV4.getPool(id);
    }

    function _getCurrentTick(PoolId id) internal view override returns (int24 tick) {
        (, tick,,,,,) = uniV4.getPool(id);
    }
}
