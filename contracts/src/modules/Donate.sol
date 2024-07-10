// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.24;

import {PoolId} from "v4-core/src/types/PoolId.sol";
import {TickRewards} from "../types/TickRewards.sol";
import {TickLib} from "../libraries/TickLib.sol";
import {MixedSignLib} from "../libraries/MixedSignLib.sol";
import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";

/// @author philogy <https://github.com/philogy>
abstract contract Donate {
    using FixedPointMathLib for uint256;
    using TickLib for uint256;
    using MixedSignLib for uint128;

    error WrongEndLiquidity(uint128, uint128);

    /**
     * @dev Entry point to multi-tick donate logic, *cannot* donate to current tick.
     * @param rewards The rewards struct to update
     * @param id The pool id to use to retrieve pool data
     * @param tick The start tick from which to donate
     * @param startLiquidity The assumed startLiquidity
     * @param currentDonate Amount to donate to the current tick
     * @param amounts The amounts to donate across the ticks
     */
    function _donate(
        TickRewards storage rewards,
        PoolId id,
        int24 tick,
        uint128 startLiquidity,
        uint256 currentDonate,
        uint256[] memory amounts
    ) internal returns (uint256 total) {
        int24 currentTick = _getCurrentTick(id);
        uint256 cumulativeGrowth;
        uint128 endLiquidity;
        (total, cumulativeGrowth, endLiquidity) = tick <= currentTick
            ? _donateBelow(rewards, id, tick, currentTick, startLiquidity, amounts)
            : _donateAbove(rewards, id, tick, currentTick, startLiquidity, amounts);
        uint128 currentLiquidity = _getCurrentLiquidity(id);
        if (endLiquidity != currentLiquidity) revert WrongEndLiquidity(endLiquidity, currentLiquidity);
        total += currentDonate;
        rewards.globalGrowth += cumulativeGrowth + currentDonate.divWad(currentLiquidity);
    }

    function _donateBelow(
        TickRewards storage rewards,
        PoolId id,
        int24 tick,
        int24 currentTick,
        uint128 liquidity,
        uint256[] memory amounts
    ) internal returns (uint256, uint256, uint128) {
        // To not waste a loop iteration we assume the given tick is a valid initialized tick.
        // TODO(security): Check that this doesn't allow breaking invariants if invoked with an
        // unitialized tick.
        bool initialized = true;
        uint256 amountIndex = 0;
        uint256 total;
        uint256 cumulativeGrowth;

        do {
            if (initialized) {
                uint256 amount = amounts[amountIndex++];
                total += amount;

                cumulativeGrowth += flatDivWad(amount, liquidity);
                rewards.tickGrowthOutside[tick] += cumulativeGrowth;

                liquidity = liquidity.add(_getNetTickLiquidity(id, tick));
            }

            (initialized, tick) = _findNextTickUp(id, tick);
        } while (tick <= currentTick);

        return (total, cumulativeGrowth, liquidity);
    }

    function _donateAbove(
        TickRewards storage rewards,
        PoolId id,
        int24 tick,
        int24 currentTick,
        uint128 liquidity,
        uint256[] memory amounts
    ) internal returns (uint256, uint256, uint128) {
        // To not waste a loop iteration we assume the given tick is a valid initialized tick.
        // TODO(security): Check that this doesn't allow breaking invariants if invoked with an
        // unitialized tick.
        bool initialized = true;
        uint256 amountIndex = 0;
        uint256 total;
        uint256 cumulativeGrowth;

        do {
            if (initialized) {
                uint256 amount = amounts[amountIndex++];
                total += amount;

                cumulativeGrowth += flatDivWad(amount, liquidity);
                rewards.tickGrowthOutside[tick] += cumulativeGrowth;

                int128 netLiquidity = _getNetTickLiquidity(id, tick);
                liquidity = liquidity.sub(netLiquidity);
            }

            (initialized, tick) = _findNextTickDown(id, tick);
        } while (currentTick < tick);

        return (total, cumulativeGrowth, liquidity);
    }

    function _findNextTickUp(PoolId id, int24 tick) internal view returns (bool initialized, int24 newTick) {
        (int16 wordPos, uint8 bitPos) = TickLib.position(TickLib.compress(tick) + 1);
        (initialized, bitPos) = _getPoolBitmapInfo(id, wordPos).nextBitPosGte(bitPos);
        newTick = TickLib.toTick(wordPos, bitPos);
    }

    function _findNextTickDown(PoolId id, int24 tick) internal view returns (bool initialized, int24 newTick) {
        (int16 wordPos, uint8 bitPos) = TickLib.position(TickLib.compress(tick - 1));
        (initialized, bitPos) = _getPoolBitmapInfo(id, wordPos).nextBitPosLte(bitPos);
        newTick = TickLib.toTick(wordPos, bitPos);
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
