// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import {UniConsumer} from "./UniConsumer.sol";
import {IPoolManager} from "v4-core/src/interfaces/IPoolManager.sol";
import {Asset} from "../types/Asset.sol";
import {Globals} from "../types/Globals.sol";
import {AssetIndex} from "../types/PriceGraph.sol";
import {tuint256} from "transient-goodies/TransientPrimitives.sol";
import {BalanceDelta} from "v4-core/src/types/BalanceDelta.sol";

import {SafeTransferLib} from "solady/src/utils/SafeTransferLib.sol";
import {ConversionLib} from "src/libraries/ConversionLib.sol";

import {console2 as console} from "forge-std/console2.sol";

struct PoolSwap {
    AssetIndex asset0Index;
    AssetIndex asset1Index;
    bool zeroForOne;
    uint256 amountIn;
}

/// @author philogy <https://github.com/philogy>
abstract contract Accounter is UniConsumer {
    using SafeTransferLib for address;
    using ConversionLib for *;

    /// @dev Uniswap's `MIN_SQRT_RATIO + 1` to pass the limit check.
    uint160 internal constant MIN_SQRT_RATIO = 4295128740;
    /// @dev Uniswap's `MAX_SQRT_RATIO - 1` to pass the limit check.
    uint160 internal constant MAX_SQRT_RATIO = 1461446703485210103287273052203988822378723970341;

    mapping(address => uint256) internal savedFees;
    mapping(address => tuint256) internal freeBalance;

    function _borrowAssets(Asset[] calldata assets) internal {
        for (uint256 i = 0; i < assets.length; i++) {
            Asset calldata asset = assets[i];
            uint256 amount = asset.borrow;
            address addr = asset.addr;
            UNI_V4.take(addr.intoC(), address(this), amount);
            freeBalance[addr].inc(amount);
        }
    }

    function _execPoolSwaps(Globals memory g, PoolSwap[] calldata swaps) internal {
        for (uint256 i = 0; i < swaps.length; i++) {
            PoolSwap calldata swap = swaps[i];
            address asset0 = g.get(swap.asset0Index);
            address asset1 = g.get(swap.asset1Index);
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

    function _saveAndSettle(Asset[] calldata assets) internal {
        for (uint256 i = 0; i < assets.length; i++) {
            Asset calldata asset = assets[i];
            address addr = asset.addr;
            uint256 saving = asset.save;
            uint256 settle = asset.settle;

            freeBalance[addr].dec(saving + settle);
            savedFees[addr] += saving;
            if (settle > 0) {
                UNI_V4.sync(addr.intoC());
                addr.safeTransfer(address(UNI_V4), settle);
                UNI_V4.settle(addr.intoC());
            }

            console.log("final excess [%s]: %s", addr, freeBalance[addr].get());
        }
    }

    function _accountIn(address from, address asset, uint256 amount) internal {
        freeBalance[asset].inc(amount);
        asset.safeTransferFrom(from, address(this), amount);
    }

    function _accountOut(address to, address asset, uint256 amount) internal {
        freeBalance[asset].dec(amount);
        asset.safeTransfer(to, amount);
    }
}
