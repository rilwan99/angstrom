// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.24;

import {PoolId} from "v4-core/src/types/PoolId.sol";
import {PoolRewards, REWARD_GROWTH_SIZE} from "../types/PoolRewards.sol";
import {CalldataReader} from "../types/CalldataReader.sol";
import {IPoolManager, IUniV4} from "../interfaces/IUniV4.sol";
import {UniConsumer} from "./UniConsumer.sol";

import {TickLib} from "../libraries/TickLib.sol";
import {MixedSignLib} from "../libraries/MixedSignLib.sol";
import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";

import {console} from "forge-std/console.sol";
import {FormatLib} from "super-sol/libraries/FormatLib.sol";

/// @author philogy <https://github.com/philogy>
abstract contract RewardsUpdater is UniConsumer {
    using IUniV4 for IPoolManager;
    using FixedPointMathLib for uint256;
    using TickLib for uint256;
    // TODO: Remove
    using FormatLib for *;

    error WrongEndLiquidity(uint128 endLiquidity, uint128 actualCurrentLiquidity);

    // Stack too deep shenanigan.
    struct RewardParams {
        PoolId id;
        int24 tickSpacing;
        int24 currentTick;
    }

    function _decodeAndReward(
        bool currentOnly,
        CalldataReader reader,
        PoolRewards storage poolRewards_,
        PoolId id,
        int24 tickSpacing,
        int24 currentTick
    ) internal returns (CalldataReader, uint256) {
        if (currentOnly) {
            uint128 amount;
            (reader, amount) = reader.readU128();
            poolRewards_.globalGrowth += flatDivWad(amount, UNI_V4.getPoolLiquidity(id));

            return (reader, amount);
        }

        uint256 cumulativeGrowth;
        uint128 endLiquidity;

        int24 startTick;
        (reader, startTick) = reader.readI24();
        console.log("startTick: %s", startTick);
        uint128 liquidity;
        (reader, liquidity) = reader.readU128();
        console.log("liquidity: %s", liquidity);
        (CalldataReader newReader, CalldataReader amountsEnd) = reader.readU24End();

        // Stack too deep shenanigan.
        PoolRewards storage poolRewards = poolRewards_;

        uint256 total;
        RewardParams memory pool = RewardParams(id, tickSpacing, currentTick);
        (newReader, total, cumulativeGrowth, endLiquidity) = startTick <= pool.currentTick
            ? _rewardBelow(poolRewards.rewardGrowthOutside, startTick, newReader, liquidity, pool)
            : _rewardAbove(poolRewards.rewardGrowthOutside, startTick, newReader, liquidity, pool);

        console.log("Done rewarding, trying to get reward for current tick");
        uint128 donateToCurrent;
        (newReader, donateToCurrent) = newReader.readU128();
        console.log("donateToCurrent: %s", donateToCurrent);
        cumulativeGrowth += flatDivWad(donateToCurrent, endLiquidity);
        total += donateToCurrent;

        newReader.requireAtEndOf(amountsEnd);

        uint128 currentLiquidity = UNI_V4.getPoolLiquidity(pool.id);
        console.log("currentLiquidity: %s", currentLiquidity);
        console.log("endLiquidity: %s", endLiquidity);
        if (endLiquidity != currentLiquidity) {
            revert WrongEndLiquidity(endLiquidity, currentLiquidity);
        }

        poolRewards.globalGrowth += cumulativeGrowth;

        return (newReader, total);
    }

    function _rewardBelow(
        uint256[REWARD_GROWTH_SIZE] storage rewardGrowthOutside,
        int24 rewardTick,
        CalldataReader reader,
        uint128 liquidity,
        RewardParams memory pool
    ) internal returns (CalldataReader, uint256, uint256, uint128) {
        bool initialized = true;
        uint256 total = 0;
        uint256 cumulativeGrowth = 0;
        console.log("in _rewardBelow but we shouldn't be");
        do {
            if (initialized) {
                uint256 amount;
                (reader, amount) = reader.readU128();

                total += amount;
                cumulativeGrowth += flatDivWad(amount, liquidity);
                rewardGrowthOutside[uint24(rewardTick)] += cumulativeGrowth;

                (, int128 netLiquidity) = UNI_V4.getTickLiquidity(pool.id, rewardTick);
                liquidity = MixedSignLib.add(liquidity, netLiquidity);
            }
            (initialized, rewardTick) = UNI_V4.getNextTickGt(pool.id, rewardTick, pool.tickSpacing);
        } while (rewardTick <= pool.currentTick);

        return (reader, total, cumulativeGrowth, liquidity);
    }

    function _rewardAbove(
        uint256[REWARD_GROWTH_SIZE] storage rewardGrowthOutside,
        int24 rewardTick,
        CalldataReader reader,
        uint128 liquidity,
        RewardParams memory pool
    ) internal returns (CalldataReader, uint256, uint256, uint128) {
        bool initialized = true;
        uint256 total = 0;
        uint256 cumulativeGrowth = 0;
        console.log("in _rewardAbove");
        console.log("start liquidity: %s", liquidity);
        do {
            console.log("Trying to reward tick: %s", rewardTick);
            if (initialized) {
                console.log("Tick is initialized");
                uint256 amount;
                (reader, amount) = reader.readU128();
                console.log("Got reward amount: %s", amount);

                total += amount;
                cumulativeGrowth += flatDivWad(amount, liquidity);
                rewardGrowthOutside[uint24(rewardTick)] += cumulativeGrowth;

                (, int128 netLiquidity) = UNI_V4.getTickLiquidity(pool.id, rewardTick);
                console.log("netLiquidity: %s", netLiquidity);
                liquidity = MixedSignLib.sub(liquidity, netLiquidity);
                console.log("new liquidity: %s", liquidity);
            } else {
                console.log("Tick is not initialized");
            }
            (initialized, rewardTick) = UNI_V4.getNextTickLt(pool.id, rewardTick, pool.tickSpacing);
        } while (rewardTick > pool.currentTick);
        console.log("Done rewarding.  Total: %s", total);
        console.log("Done rewarding.  Final liquidity: %s", liquidity);
        return (reader, total, cumulativeGrowth, liquidity);
    }

    /**
     * @dev Overflow-safe fixed-point division of `x / y` resulting in `0` if `y` is zero.
     */
    function flatDivWad(uint256 x, uint256 y) internal pure returns (uint256) {
        return (x * FixedPointMathLib.WAD).rawDiv(y);
    }
}
