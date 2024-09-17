// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {BaseTest} from "test/_helpers/BaseTest.sol";
import {TICK_SPACING} from "src/Constants.sol";
import {PoolRewardsHandler} from "./PoolRewardsHandler.sol";

import {PoolGate} from "test/_helpers/PoolGate.sol";
import {TickMath} from "v4-core/src/libraries/TickMath.sol";

import {PoolId} from "v4-core/src/types/PoolId.sol";
import {PRNG} from "super-sol/collections/PRNG.sol";
import {MockERC20} from "super-sol/mocks/MockERC20.sol";
import {ExtAngstrom} from "../../_view-ext/ExtAngstrom.sol";
import {UniV4Inspector} from "test/_view-ext/UniV4Inspector.sol";
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

/// @author philogy <https://github.com/philogy>
contract PoolRewardsInvariantTest is BaseTest, HookDeployer {
    using TickMath for int24;

    UniV4Inspector public uniV4;
    ExtAngstrom public angstrom;
    PoolGate public gate;
    PoolId public id;

    MockERC20 public asset0 = new MockERC20();
    MockERC20 public asset1 = new MockERC20();
    address public owner = makeAddr("owner");
    address public gov = makeAddr("gov");

    PoolRewardsHandler handler;
    bytes4[] handlerSelectors;

    function setUp() public {
        if (asset1 < asset0) (asset0, asset1) = (asset1, asset0);

        vm.prank(owner);
        uniV4 = new UniV4Inspector();
        gate = new PoolGate(address(uniV4));

        (bool success, address angstromAddr,) = deployHook(
            abi.encodePacked(type(ExtAngstrom).creationCode, abi.encode(uniV4, gov)), _angstromFlags(), CREATE2_FACTORY
        );
        gate.setHook(angstromAddr);

        assertTrue(success, "Failed to deploy angstrom");
        angstrom = ExtAngstrom(angstromAddr);
        id = PoolIdLibrary.toId(poolKey());

        int24 startTick = 0;
        gate.initializePool(address(asset0), address(asset1), startTick.getSqrtPriceAtTick());

        handler = new PoolRewardsHandler(uniV4, angstrom, gate, id, asset0, asset1, gov);

        handlerSelectors.push(PoolRewardsHandler.rewardLiquidity.selector);

        targetSelector(FuzzSelector({addr: address(handler), selectors: handlerSelectors}));
        targetContract(address(handler));

        handler.addLiquidity(-2 * TICK_SPACING, startTick, 1e21);
        handler.addLiquidity(startTick, 2 * TICK_SPACING, 1e21);
    }

    function invariant_thereIsNoWarInBaSingSe() public view {
        assertEq(handler.ghost_totalInititalizedTicks(), 3);
    }

    function poolKey() internal view returns (PoolKey memory) {
        return ConversionLib.toPoolKey(address(angstrom), address(asset0), address(asset1));
    }
}
