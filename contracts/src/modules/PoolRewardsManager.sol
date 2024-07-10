// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Donate} from "./Donate.sol";
import {UniConsumer} from "./UniConsumer.sol";
import {TickRewards} from "../types/TickRewards.sol";

import {IUniV4} from "../interfaces/IUniV4.sol";
import {IPoolManager} from "../interfaces/IUniV4.sol";
import {PoolId, PoolIdLibrary} from "v4-core/src/types/PoolId.sol";
import {AssetIndex} from "../types/PriceGraph.sol";
import {Globals} from "../types/Globals.sol";

import {ConversionLib} from "../libraries/ConversionLib.sol";

struct PoolRewardsUpdate {
    AssetIndex asset0;
    AssetIndex asset1;
    int24 startTick;
    uint128 startLiquidity;
    uint256 currentDonate;
    uint256[] amounts;
}

/// @author philogy <https://github.com/philogy>
abstract contract PoolRewardsManager is Donate, UniConsumer {
    using IUniV4 for IPoolManager;

    mapping(PoolId id => TickRewards rewards) internal rewards;

    function _rewardPools(Globals memory g, PoolRewardsUpdate[] memory updates) internal returns (uint256 total) {
        for (uint256 i = 0; i < updates.length; i++) {
            PoolRewardsUpdate memory update = updates[i];
            address asset0 = g.get(update.asset0);
            address asset1 = g.get(update.asset1);
            PoolId id = PoolIdLibrary.toId(ConversionLib.toPoolKey(address(this), asset0, asset1));
            total =
                _donate(rewards[id], id, update.startTick, update.startLiquidity, update.currentDonate, update.amounts);
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
