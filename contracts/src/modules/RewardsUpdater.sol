// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.24;

import {PoolId} from "v4-core/src/types/PoolId.sol";
import {PoolRewards, REWARD_GROWTH_SIZE} from "../types/PoolRewards.sol";
import {CalldataReader} from "../types/CalldataReader.sol";

import {TickLib} from "../libraries/TickLib.sol";
import {MixedSignLib} from "../libraries/MixedSignLib.sol";
import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";

import {console} from "forge-std/console.sol";
import {FormatLib} from "super-sol/libraries/FormatLib.sol";

/// @author philogy <https://github.com/philogy>
abstract contract RewardsUpdater {
    using FixedPointMathLib for uint256;
    using TickLib for uint256;
    using FormatLib for *;

    error WrongEndLiquidity(uint128 endLiquidity, uint128 actualCurrentLiquidity);

    struct PoolInterface {
        PoolId id;
        int24 tickSpacing;
        int24 currentTick;
    }

    function _decodeAndReward(
        CalldataReader reader,
        PoolRewards storage poolRewards,
        PoolId id,
        int24 tickSpacing
    ) internal returns (CalldataReader, uint256) {
        {
            bool onlyCurrent;
            (reader, onlyCurrent) = reader.readBool();

            if (onlyCurrent) {
                uint128 amount;
                (reader, amount) = reader.readU128();
                poolRewards.globalGrowth += flatDivWad(amount, _getCurrentLiquidity(id));

                return (reader, amount);
            }
        }

        uint256 cumulativeGrowth;
        uint128 endLiquidity;

        int24 startTick;
        (reader, startTick) = reader.readI24();
        uint128 liquidity;
        (reader, liquidity) = reader.readU128();
        (CalldataReader newReader, CalldataReader amountsEnd) = reader.readU24End();

        uint256 total;
        PoolInterface memory pool = PoolInterface(id, tickSpacing, _getCurrentTick(id));
        (newReader, total, cumulativeGrowth, endLiquidity) = startTick <= pool.currentTick
            ? _rewardBelow(poolRewards.rewardGrowthOutside, startTick, newReader, liquidity, pool)
            : _rewardAbove(poolRewards.rewardGrowthOutside, startTick, newReader, liquidity, pool);

        uint128 donateToCurrent;
        (newReader, donateToCurrent) = newReader.readU128();
        cumulativeGrowth += flatDivWad(donateToCurrent, endLiquidity);
        total += donateToCurrent;

        newReader.requireAtEndOf(amountsEnd);

        uint128 currentLiquidity = _getCurrentLiquidity(id);
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
        PoolInterface memory pool
    ) internal returns (CalldataReader, uint256, uint256, uint128) {
        bool initialized = true;
        uint256 total = 0;
        uint256 cumulativeGrowth = 0;

        while (rewardTick <= pool.currentTick) {
            if (initialized) {
                uint256 amount;
                (reader, amount) = reader.readU128();

                total += amount;
                cumulativeGrowth += flatDivWad(amount, liquidity);
                rewardGrowthOutside[uint24(rewardTick)] += cumulativeGrowth;

                liquidity = MixedSignLib.add(liquidity, _getNetTickLiquidity(pool.id, rewardTick));
            }
            (initialized, rewardTick) = _findNextTickUp(pool.id, rewardTick, pool.tickSpacing);
        }

        return (reader, total, cumulativeGrowth, liquidity);
    }

    function _rewardAbove(
        uint256[REWARD_GROWTH_SIZE] storage rewardGrowthOutside,
        int24 rewardTick,
        CalldataReader reader,
        uint128 liquidity,
        PoolInterface memory pool
    ) internal returns (CalldataReader, uint256, uint256, uint128) {
        bool initialized = true;
        uint256 total = 0;
        uint256 cumulativeGrowth = 0;

        while (rewardTick > pool.currentTick) {
            if (initialized) {
                uint256 amount;
                (reader, amount) = reader.readU128();

                total += amount;
                cumulativeGrowth += flatDivWad(amount, liquidity);
                rewardGrowthOutside[uint24(rewardTick)] += cumulativeGrowth;

                liquidity = MixedSignLib.sub(liquidity, _getNetTickLiquidity(pool.id, rewardTick));
            }
            (initialized, rewardTick) = _findNextTickDown(pool.id, rewardTick, pool.tickSpacing);
        }

        return (reader, total, cumulativeGrowth, liquidity);
    }

    function _findNextTickUp(PoolId id, int24 tick, int24 tickSpacing)
        internal
        view
        returns (bool initialized, int24 newTick)
    {
        (int16 wordPos, uint8 bitPos) = TickLib.position(TickLib.compress(tick, tickSpacing) + 1);
        (initialized, bitPos) = _getPoolBitmapInfo(id, wordPos).nextBitPosGte(bitPos);
        newTick = TickLib.toTick(wordPos, bitPos, tickSpacing);
    }

    function _findNextTickDown(PoolId id, int24 tick, int24 tickSpacing)
        internal
        view
        returns (bool initialized, int24 newTick)
    {
        (int16 wordPos, uint8 bitPos) = TickLib.position(TickLib.compress(tick - 1, tickSpacing));
        (initialized, bitPos) = _getPoolBitmapInfo(id, wordPos).nextBitPosLte(bitPos);
        newTick = TickLib.toTick(wordPos, bitPos, tickSpacing);
    }

    /**
     * @dev Overflow-safe fixed-point division of `x / y` resulting in `0` if `y` is zero.
     */
    function flatDivWad(uint256 x, uint256 y) internal pure returns (uint256) {
        return (x * FixedPointMathLib.WAD).rawDiv(y);
    }

    ////////////////////////////////////////////////////////////////
    //                       POOL INTERFACE                       //
    ////////////////////////////////////////////////////////////////

    function _getPoolBitmapInfo(PoolId id, int16 wordPos) internal view virtual returns (uint256);
    function _getNetTickLiquidity(PoolId id, int24 tick) internal view virtual returns (int128);
    function _getCurrentLiquidity(PoolId id) internal view virtual returns (uint128);
    function _getCurrentTick(PoolId id) internal view virtual returns (int24);
}
