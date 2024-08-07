// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Donate} from "./Donate.sol";
import {UniConsumer} from "./UniConsumer.sol";
import {ILiqChangeHooks} from "../interfaces/ILiqChangeHooks.sol";

import {tuint256} from "transient-goodies/TransientPrimitives.sol";
import {TickRewards} from "../types/TickRewards.sol";
import {AssetArray} from "../types/Asset.sol";

import {IUniV4} from "../interfaces/IUniV4.sol";
import {Hooks} from "v4-core/src/libraries/Hooks.sol";
import {IPoolManager} from "../interfaces/IUniV4.sol";
import {PoolId, PoolIdLibrary} from "v4-core/src/types/PoolId.sol";
import {PoolKey} from "v4-core/src/types/PoolKey.sol";
import {BalanceDelta} from "v4-core/src/types/BalanceDelta.sol";
import {MixedSignLib} from "../libraries/MixedSignLib.sol";

import {ConversionLib} from "../libraries/ConversionLib.sol";

struct PoolRewardsUpdate {
    uint16 asset0Index;
    uint16 asset1Index;
    int24 startTick;
    uint128 startLiquidity;
    uint256 currentDonate;
    uint256[] amounts;
}

/// @author philogy <https://github.com/philogy>
abstract contract PoolRewardsManager is Donate, ILiqChangeHooks, UniConsumer {
    using PoolIdLibrary for PoolKey;
    using IUniV4 for IPoolManager;
    using MixedSignLib for uint128;

    struct Position {
        uint256 rewardDebt;
    }

    struct Pool {
        TickRewards rewards;
        mapping(address owner => mapping(int24 tickLower => mapping(int24 tickUpper => Position))) positions;
    }

    mapping(PoolId id => Pool pool) internal pools;

    constructor() {
        _checkHookPermissions(Hooks.BEFORE_ADD_LIQUIDITY_FLAG | Hooks.AFTER_REMOVE_LIQUIDITY_FLAG);
    }

    // function beforeAddLiquidity(
    //     address sender,
    //     PoolKey calldata key,
    //     IPoolManager.ModifyLiquidityParams calldata params,
    //     bytes calldata hookData
    // ) external override onlyUniV4 returns (bytes4) {
    //     assert(false); // TODO
    //     PoolId id = key.toId();
    //     Position storage position = pools[id].positions[sender][params.tickLower][params.tickUpper];
    //     return this.beforeAddLiquidity.selector;
    // }
    function beforeAddLiquidity(address, PoolKey calldata, IPoolManager.ModifyLiquidityParams calldata, bytes calldata)
        external
        view
        override
        onlyUniV4
        returns (bytes4)
    {
        return this.beforeAddLiquidity.selector;
    }

    // function afterRemoveLiquidity(
    //     address sender,
    //     PoolKey calldata key,
    //     IPoolManager.ModifyLiquidityParams calldata params,
    //     BalanceDelta delta,
    //     bytes calldata hookData
    // ) external returns (bytes4, BalanceDelta) {
    //     assert(false); // TODO
    // }
    function afterRemoveLiquidity(
        address,
        PoolKey calldata,
        IPoolManager.ModifyLiquidityParams calldata,
        BalanceDelta,
        bytes calldata
    ) external pure returns (bytes4, BalanceDelta) {
        return (bytes4(0), BalanceDelta.wrap(0));
    }

    function _rewardPools(
        AssetArray assets,
        PoolRewardsUpdate[] calldata updates,
        mapping(address => tuint256) storage freeBalance
    ) internal {
        for (uint256 i = 0; i < updates.length; i++) {
            PoolRewardsUpdate calldata update = updates[i];
            address asset0 = assets.get(update.asset0Index).addr();
            address asset1 = assets.get(update.asset1Index).addr();
            PoolId id = ConversionLib.toPoolKey(address(this), asset0, asset1).toId();
            uint256 total = _donate(
                pools[id].rewards, id, update.startTick, update.startLiquidity, update.currentDonate, update.amounts
            );
            freeBalance[asset0].dec(total);
        }
    }

    function _getPoolBitmapInfo(PoolId id, int16 wordPos) internal view override returns (uint256) {
        return UNI_V4.getPoolBitmapInfo(id, wordPos);
    }

    function _getNetTickLiquidity(PoolId id, int24 tick) internal view override returns (int128 liquidityNet) {
        (, liquidityNet) = UNI_V4.getTickLiquidity(id, tick);
    }

    function _getCurrentLiquidity(PoolId id) internal view override returns (uint128 liquidity) {
        liquidity = UNI_V4.getPoolLiquidity(id);
    }

    function _getCurrentTick(PoolId id) internal view override returns (int24 tick) {
        tick = UNI_V4.getSlot0(id).tick();
    }
}
