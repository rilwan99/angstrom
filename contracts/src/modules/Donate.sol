// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {PoolId} from "v4-core/src/types/PoolId.sol";
import {TickRewards} from "../types/TickRewards.sol";
import {TickLib} from "../libraries/TickLib.sol";
import {MixedSignLib} from "../libraries/MixedSignLib.sol";
import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";

import {FormatLib} from "test/_helpers/FormatLib.sol";
import {LibString} from "solady/src/utils/LibString.sol";
import {console2 as console} from "forge-std/console2.sol";

/// @author philogy <https://github.com/philogy>
abstract contract Donate {
    using FormatLib for *;
    using LibString for *;

    using FixedPointMathLib for uint256;
    using TickLib for uint256;
    using MixedSignLib for uint128;

    error InvalidEndTick(int24, int24);
    error WrongEndLiquidity(uint128, uint128);

    /**
     * @dev Entry point to multi-tick donate logic, *cannot* donate to current tick.
     * @param rewards The rewards struct to update
     * @param id The pool id to use to retrieve pool data
     * @param tick The start tick from which to donate
     * @param startLiquidity The assumed startLiquidity
     */
    function _donate(
        TickRewards storage rewards,
        PoolId id,
        int24 tick,
        uint128 startLiquidity,
        uint256[] memory amounts
    ) internal returns (uint256 total) {
        int24 currentTick = _getCurrentTick(id);
        uint256 cumulativeGrowth;
        uint128 endLiquidity;
        (total, cumulativeGrowth, endLiquidity, tick) = tick <= currentTick
            ? _donateBelow(rewards, id, tick, startLiquidity, amounts)
            : _donateAbove(rewards, id, tick, startLiquidity, amounts);
        if (tick != currentTick) revert InvalidEndTick(tick, currentTick);
        uint128 currentLiquidity = _getCurrentLiquidity(id);
        if (endLiquidity != currentLiquidity) revert WrongEndLiquidity(endLiquidity, currentLiquidity);
        rewards.globalGrowth += cumulativeGrowth;
    }

    function _donateBelow(
        TickRewards storage rewards,
        PoolId id,
        int24 tick,
        uint128 liquidity,
        uint256[] memory amounts
    ) internal returns (uint256 totalDonated, uint256 cumulativeGrowth, uint128 endLiquidity, int24 endTick) {
        // To not waste a loop iteration we assume the given tick is a valid initialized tick.
        // TODO(security): Check that this doesn't allow breaking invariants if invoked with an
        // unitialized tick.
        bool initialized = true;
        uint256 amountIndex = 0;

        while (true) {
            if (initialized) {
                uint256 amount = amounts[amountIndex++];
                totalDonated += amount;
                console.log("%s.divWad(%s)", amount, liquidity);
                cumulativeGrowth += amount.divWad(liquidity);

                liquidity = liquidity.add(_getNetTickLiquidity(id, tick));
                rewards.tickGrowthOutside[tick] += cumulativeGrowth;

                if (amountIndex >= amounts.length) break;
            }

            (initialized, tick) = _findNextTickUp(id, tick);
        }

        endLiquidity = liquidity;
        endTick = tick;
    }

    function _donateAbove(
        TickRewards storage rewards,
        PoolId id,
        int24 tick,
        uint128 liquidity,
        uint256[] memory amounts
    ) internal returns (uint256 totalDonated, uint256 cumulativeGrowth, uint128 endLiquidity, int24 endTick) {
        // To not waste a loop iteration we assume the given tick is a valid initialized tick.
        // TODO(security): Check that this doesn't allow breaking invariants if invoked with an
        // unitialized tick.
        bool initialized = true;
        uint256 amountIndex = 0;

        while (true) {
            if (initialized) {
                uint256 amount = amounts[amountIndex++];
                totalDonated += amount;
                cumulativeGrowth += amount.divWad(liquidity);
                rewards.tickGrowthOutside[tick] += cumulativeGrowth;
                liquidity = liquidity.sub(_getNetTickLiquidity(id, tick));
                if (amountIndex >= amounts.length) break;
            }

            (initialized, tick) = _findNextTickDown(id, tick);
        }

        endLiquidity = liquidity;
        // Need to go down one final time because when donating above we *do not* want to update the current.
        (, endTick) = _findNextTickDown(id, tick);
    }

    function _findNextTickUp(PoolId id, int24 tick) internal view returns (bool initialized, int24 newTick) {
        (int16 wordPos, uint8 bitPos) = TickLib.position(TickLib.compress(tick) + 1);
        (initialized, bitPos) = _getPoolBitmapInfo(id, wordPos).nextBitPosGte(bitPos);
        newTick = TickLib.toTick(wordPos, bitPos);
        console.log("next up: %s -> %s", tick.toString(), newTick.toString());
    }

    function _findNextTickDown(PoolId id, int24 tick) internal view returns (bool initialized, int24 newTick) {
        (int16 wordPos, uint8 bitPos) = TickLib.position(TickLib.compress(tick - 1));
        (initialized, bitPos) = _getPoolBitmapInfo(id, wordPos).nextBitPosLte(bitPos);
        newTick = TickLib.toTick(wordPos, bitPos);
    }

    ////////////////////////////////////////////////////////////////
    //                       POOL INTERFACE                       //
    ////////////////////////////////////////////////////////////////

    function _getPoolBitmapInfo(PoolId id, int16 wordPos) internal view virtual returns (uint256);
    function _getNetTickLiquidity(PoolId id, int24 tick) internal view virtual returns (int128);
    function _getCurrentLiquidity(PoolId id) internal view virtual returns (uint128);
    function _getCurrentTick(PoolId id) internal view virtual returns (int24);
}
