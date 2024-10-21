// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {BaseTest} from "test/_helpers/BaseTest.sol";
import {UniV4Inspector} from "test/_view-ext/UniV4Inspector.sol";
import {OpenAngstrom} from "test/_mocks/OpenAngstrom.sol";
import {PoolGate} from "test/_helpers/PoolGate.sol";
import {MockERC20} from "super-sol/mocks/MockERC20.sol";
import {EnumerableSetLib} from "solady/src/utils/EnumerableSetLib.sol";
import {MAX_FEE} from "src/libraries/PoolConfigStore.sol";

import {TickMath} from "v4-core/src/libraries/TickMath.sol";

struct Env {
    address owner;
    address controller;
    address feeMaster;
    UniV4Inspector uniV4;
    OpenAngstrom angstrom;
    PoolGate gate;
    MockERC20[] assets;
    MockERC20[] mirrors;
}

/// @author philogy <https://github.com/philogy>
contract AngstromHandler is BaseTest {
    using EnumerableSetLib for *;

    Env e;

    EnumerableSetLib.AddressSet internal _actors;
    EnumerableSetLib.AddressSet internal _enabledAssets;

    mapping(address asset => uint256 total) public ghost_totalHeldBalance;

    struct Pool {
        uint256 index0;
        uint256 index1;
    }

    Pool[] _ghost_pools;
    mapping(uint256 index0 => mapping(uint256 index1 => bool)) _ghost_poolInitialized;

    constructor(Env memory env) {
        e = env;
    }

    function initializePool(
        uint256 asset0Index,
        uint256 asset1Index,
        int24 tickSpacing,
        uint24 feeInE6,
        uint160 startSqrtPriceX96
    ) public {
        asset0Index = bound(asset0Index, 0, e.assets.length - 1);
        asset1Index = bound(asset1Index, 0, e.assets.length - 1);
        feeInE6 = uint24(bound(feeInE6, 0, MAX_FEE));
        startSqrtPriceX96 =
            uint160(bound(startSqrtPriceX96, TickMath.MIN_SQRT_PRICE, TickMath.MAX_SQRT_PRICE));
        if (asset0Index == asset1Index) {
            unchecked {
                asset1Index = (asset0Index + 1) % e.assets.length;
            }
        }
        if (asset0Index > asset1Index) (asset0Index, asset1Index) = (asset1Index, asset0Index);
        tickSpacing =
            int24(bound(tickSpacing, TickMath.MIN_TICK_SPACING, TickMath.MAX_TICK_SPACING));

        assertFalse(_ghost_poolInitialized[asset0Index][asset1Index]);
        _ghost_poolInitialized[asset0Index][asset1Index] = true;
        uint256 storeIndex = _ghost_pools.length;
        _ghost_pools.push(Pool(asset0Index, asset1Index));

        address asset0 = address(e.assets[asset0Index]);
        address mirror0 = address(e.mirrors[asset0Index]);
        address asset1 = address(e.assets[asset1Index]);
        address mirror1 = address(e.mirrors[asset1Index]);

        vm.prank(e.controller);
        e.angstrom.configurePool(asset0, asset1, uint16(uint24(tickSpacing)), feeInE6);

        e.angstrom.initializePool(asset0, asset1, storeIndex, startSqrtPriceX96);

        e.uniV4.initialize(poolKey(mirror0, mirror1, tickSpacing), startSqrtPriceX96);
    }

    function doNothing() public {}

    function enabledAssets() public view returns (address[] memory) {
        return _enabledAssets.values();
    }

    function actors() public view returns (address[] memory) {
        return _actors.values();
    }
}
