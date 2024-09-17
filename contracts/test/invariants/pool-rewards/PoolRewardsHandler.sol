// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {BaseTest} from "test/_helpers/BaseTest.sol";
import {PoolId} from "v4-core/src/types/PoolId.sol";
import {PRNG} from "super-sol/collections/PRNG.sol";
import {MockERC20} from "super-sol/mocks/MockERC20.sol";
import {TICK_SPACING} from "src/Constants.sol";
import {ExtAngstrom} from "../../_view-ext/ExtAngstrom.sol";
import {UniV4Inspector} from "test/_view-ext/UniV4Inspector.sol";
import {PoolGate} from "test/_helpers/PoolGate.sol";
import {TickLib} from "src/libraries/TickLib.sol";
import {TickReward, RewardLib} from "test/_helpers/RewardLib.sol";
import {UsedIndexMap} from "super-sol/collections/UsedIndexMap.sol";
import {PoolKey} from "v4-core/src/types/PoolKey.sol";
import {PoolId, PoolIdLibrary} from "v4-core/src/types/PoolId.sol";
import {ConversionLib} from "src/libraries/ConversionLib.sol";
import {HookDeployer} from "test/_helpers/HookDeployer.sol";

import {Bundle} from "src/reference/Bundle.sol";
import {Asset} from "src/reference/Asset.sol";
import {PoolUpdate, RewardsUpdate} from "src/reference/PoolUpdate.sol";
import {TopOfBlockOrder} from "src/reference/OrderTypes.sol";

import {EnumerableSetLib} from "solady/src/utils/EnumerableSetLib.sol";
import {console} from "forge-std/console.sol";
import {FormatLib} from "super-sol/libraries/FormatLib.sol";

/// @author philogy <https://github.com/philogy>
contract PoolRewardsHandler is BaseTest {
    using EnumerableSetLib for EnumerableSetLib.Int256Set;
    using FormatLib for *;

    using RewardLib for TickReward[];

    UniV4Inspector public immutable uniV4;
    ExtAngstrom public immutable angstrom;
    PoolGate public immutable gate;
    PoolId public immutable id;

    MockERC20 public immutable asset0;
    MockERC20 public immutable asset1;
    Account public rewarder = makeAccount("rewarder");

    EnumerableSetLib.Int256Set internal ghost_initializedTicks;

    constructor(
        UniV4Inspector uniV4_,
        ExtAngstrom angstrom_,
        PoolGate gate_,
        PoolId id_,
        MockERC20 asset0_,
        MockERC20 asset1_,
        address gov
    ) {
        uniV4 = uniV4_;
        angstrom = angstrom_;
        gate = gate_;
        id = id_;
        asset0 = asset0_;
        asset1 = asset1_;

        vm.prank(rewarder.addr);
        asset0.approve(address(angstrom), type(uint256).max);

        {
            address[] memory newNodes = new address[](1);
            newNodes[0] = rewarder.addr;
            vm.prank(gov);
            angstrom.govToggleNodes(newNodes);
        }
    }

    function addLiquidity(int24 lowerTick, int24 upperTick, uint256 liquidity) public {
        assertGt(liquidity, 0, "Liquidity zero");
        assertLt(lowerTick, upperTick, "Upper tick below or equal to lower tick");
        assertEq(lowerTick % TICK_SPACING, 0, "Lower tick incorrectly spaced");
        assertEq(upperTick % TICK_SPACING, 0, "Lower tick incorrectly spaced");
        gate.addLiquidity(address(asset0), address(asset1), lowerTick, upperTick, liquidity);
        ghost_initializedTicks.add(lowerTick);
        ghost_initializedTicks.add(upperTick);
    }

    function r() public view returns (address) {
        return rewarder.addr;
    }

    TickReward[] temp_tickRewards;

    function rewardLiquidity(uint256 ticksToReward, PRNG memory rng) public {
        uint256 totalTicks = ghost_initializedTicks.length();
        ticksToReward = bound(ticksToReward, 0, totalTicks);

        // Select ticks & amounts to reward with.
        uint128 total = 0;
        assembly {
            sstore(temp_tickRewards.slot, 0)
        }
        UsedIndexMap memory map;
        map.init(totalTicks, totalTicks / 4);
        for (uint256 i = 0; i < ticksToReward; i++) {
            int24 tick = int24(ghost_initializedTicks.at(rng.useRandIndex(map)));
            uint128 amount = u128(rng.randmag(0.8e18, 100.0e18));
            total += amount;
            temp_tickRewards.push(TickReward({tick: tick, amount: amount}));
        }

        vm.prank(msg.sender);
        asset0.mint(rewarder.addr, total);

        RewardsUpdate[] memory rewardUpdates = temp_tickRewards.toUpdates(uniV4, id);
        Bundle memory bundle;
        bundle.assets = new Asset[](2);
        bundle.assets[0].addr = address(asset0);
        bundle.assets[1].addr = address(asset1);
        bundle.poolUpdates = new PoolUpdate[](rewardUpdates.length);
        bundle.toBOrders = new TopOfBlockOrder[](1);
        TopOfBlockOrder memory tob = bundle.toBOrders[0];

        vm.roll(angstrom.lastBlockUpdated() + 1);

        for (uint256 i = 0; i < rewardUpdates.length; i++) {
            PoolUpdate memory poolUpdate = bundle.poolUpdates[i];
            poolUpdate.assetIn = address(asset0);
            poolUpdate.assetOut = address(asset1);
            poolUpdate.rewardUpdate = rewardUpdates[i];
            RewardsUpdate memory rewardUpdate = poolUpdate.rewardUpdate;

            for (uint256 j = 0; j < rewardUpdate.quantities.length; j++) {
                tob.quantityIn += rewardUpdate.quantities[j];
            }
        }

        tob.assetIn = address(asset0);
        tob.assetOut = address(asset1);
        tob.validForBlock = u64(block.number);
        sign(rewarder, tob.meta, angstrom.hashTyped(tob.hash()));

        vm.prank(rewarder.addr);
        bytes memory encoded = bundle.encode();
        angstrom.execute(encoded);
    }

    function ghost_totalInititalizedTicks() public view returns (uint256) {
        return ghost_initializedTicks.length();
    }

    function poolKey() internal view returns (PoolKey memory) {
        return ConversionLib.toPoolKey(address(angstrom), address(asset0), address(asset1));
    }
}
