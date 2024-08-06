// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import {IPoolManager} from "v4-core/src/interfaces/IPoolManager.sol";
import {tuint256} from "transient-goodies/TransientPrimitives.sol";
import {BalanceDelta} from "v4-core/src/types/BalanceDelta.sol";
import {Assets, Asset} from "../types/Assets.sol";
import {UniConsumer} from "./UniConsumer.sol";

import {SafeTransferLib} from "solady/src/utils/SafeTransferLib.sol";
import {ConversionLib} from "src/libraries/ConversionLib.sol";

import {console2 as console} from "forge-std/console2.sol";
import {FormatLib} from "super-sol/libraries/FormatLib.sol";

struct PoolSwap {
    uint16 asset0Index;
    uint16 asset1Index;
    bool zeroForOne;
    uint128 amountIn;
}

/// @author philogy <https://github.com/philogy>
abstract contract Accounter is UniConsumer {
    using FormatLib for *;

    using SafeTransferLib for address;
    using ConversionLib for *;

    /// @dev Uniswap's `MIN_SQRT_RATIO + 1` to pass the limit check.
    uint160 internal constant MIN_SQRT_RATIO = 4295128740;
    /// @dev Uniswap's `MAX_SQRT_RATIO - 1` to pass the limit check.
    uint160 internal constant MAX_SQRT_RATIO = 1461446703485210103287273052203988822378723970341;

    mapping(address => uint256) internal savedFees;
    mapping(address => tuint256) internal freeBalance;

    mapping(address => mapping(address => uint256)) internal _angstromReserves;

    function _borrowAssets(Assets assets) internal {
        uint256 length = assets.len();
        for (uint256 i = 0; i < length; i++) {
            Asset asset = assets.get(i);
            uint256 amount = asset.borrow();
            address addr = asset.addr();
            UNI_V4.take(addr.intoC(), address(this), amount);
            freeBalance[addr].inc(amount);
        }
    }

    function _execPoolSwaps(Assets assets, PoolSwap[] calldata swaps) internal {
        for (uint256 i = 0; i < swaps.length; i++) {
            PoolSwap calldata swap = swaps[i];
            address asset0 = assets.get(swap.asset0Index).addr();
            address asset1 = assets.get(swap.asset1Index).addr();
            BalanceDelta delta = UNI_V4.swap(
                address(this).toPoolKey(asset0, asset1),
                IPoolManager.SwapParams({
                    zeroForOne: swap.zeroForOne,
                    amountSpecified: swap.amountIn.neg(),
                    sqrtPriceLimitX96: swap.zeroForOne ? MIN_SQRT_RATIO : MAX_SQRT_RATIO
                }),
                ""
            );
            freeBalance[asset0].dec(delta.amount0());
            freeBalance[asset1].dec(delta.amount1());
        }
    }

    function _saveAndSettle(Assets assets) internal {
        uint256 length = assets.len();
        for (uint256 i = 0; i < length; i++) {
            Asset asset = assets.get(i);
            address addr = asset.addr();
            uint256 saving = asset.save();
            uint256 settle = asset.settle();

            freeBalance[addr].dec(saving + settle);
            savedFees[addr] += saving;
            if (settle > 0) {
                UNI_V4.sync(addr.intoC());
                addr.safeTransfer(address(UNI_V4), settle);
                UNI_V4.settle();
            }
        }
    }

    function _accountIn(address from, address asset, uint256 amount, bool useInternal) internal {
        freeBalance[asset].inc(amount);
        if (useInternal) _angstromReserves[from][asset] -= amount;
        else asset.safeTransferFrom(from, address(this), amount);
    }

    function _accountOut(address to, address asset, uint256 amount, bool useInternal) internal {
        freeBalance[asset].dec(amount);
        if (useInternal) _angstromReserves[to][asset] += amount;
        else asset.safeTransfer(to, amount);
    }
}
