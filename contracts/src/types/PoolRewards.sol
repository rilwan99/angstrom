// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IPoolManager} from "v4-core/src/interfaces/IPoolManager.sol";
import {PoolId} from "v4-core/src/types/PoolId.sol";
import {IUniV4} from "../interfaces/IUniV4.sol";
import {TickLib} from "../libraries/TickLib.sol";

/// @dev Should accomodate all possible tick values.
uint256 constant REWARD_GROWTH_SIZE = 16777216;

struct PoolRewards {
    uint256[REWARD_GROWTH_SIZE] rewardGrowthOutside;
    uint256 globalGrowth;
}

using PoolRewardsLib for PoolRewards global;

/// @author philogy <https://github.com/philogy>
library PoolRewardsLib {
    using IUniV4 for IPoolManager;
    using TickLib for uint256;

    function getGrowthInside(PoolRewards storage self, int24 current, int24 lower, int24 upper)
        internal
        view
        returns (uint256)
    {
        uint256 lowerGrowth = self.rewardGrowthOutside[uint24(lower)];
        uint256 upperGrowth = self.rewardGrowthOutside[uint24(upper)];
        if (current < lower) {
            return lowerGrowth - upperGrowth;
        } else if (current >= upper) {
            return upperGrowth - lowerGrowth;
        } else {
            return self.globalGrowth - lowerGrowth - upperGrowth;
        }
    }

    function updateAfterTickMove(PoolRewards storage self, PoolId id, IPoolManager uniV4, int24 lastTick, int24 newTick)
        internal
    {
        if (newTick > lastTick) {
            _updateTickMoveUp(self, id, uniV4, lastTick, newTick);
        } else if (newTick < lastTick) {
            _updateTickMoveDown(self, id, uniV4, lastTick, newTick);
        }
    }

    function _updateTickMoveUp(PoolRewards storage self, PoolId id, IPoolManager uniV4, int24 tick, int24 newTick)
        private
    {
        uint256 globalGrowth = self.globalGrowth;
        while (tick < newTick) {
            (int16 wordPos, uint8 bitPos) = TickLib.position(TickLib.compress(tick) + 1);
            bool initialized;
            (initialized, bitPos) = uniV4.getPoolBitmapInfo(id, wordPos).nextBitPosGte(bitPos);
            tick = TickLib.toTick(wordPos, bitPos);

            if (initialized) {
                self.rewardGrowthOutside[uint24(tick)] = globalGrowth - self.rewardGrowthOutside[uint24(tick)];
            }
        }
    }

    function _updateTickMoveDown(PoolRewards storage self, PoolId id, IPoolManager uniV4, int24 tick, int24 newTick)
        private
    {
        uint256 globalGrowth = self.globalGrowth;
        while (tick > newTick) {
            (int16 wordPos, uint8 bitPos) = TickLib.position(TickLib.compress(tick) - 1);
            bool initialized;
            (initialized, bitPos) = uniV4.getPoolBitmapInfo(id, wordPos).nextBitPosLte(bitPos);
            tick = TickLib.toTick(wordPos, bitPos);

            if (initialized) {
                self.rewardGrowthOutside[uint24(tick)] = globalGrowth - self.rewardGrowthOutside[uint24(tick)];
            }
        }
    }
}
