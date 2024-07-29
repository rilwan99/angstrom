// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {GenericOrder, OrderType, OrderMode} from "src/reference/GenericOrder.sol";
import {
    PartialStandingOrder,
    ExactStandingOrder,
    PartialFlashOrder,
    ExactFlashOrder,
    OrderMeta
} from "../reference/OrderTypes.sol";

import {console2 as console} from "forge-std/console2.sol";

/// @author philogy <https://github.com/philogy>
library RefAdapaterLib {
    bool internal constant VERBOSE = false;

    function adaptHash(GenericOrder memory order, address assetIn, address assetOut) internal view returns (bytes32) {
        OrderMeta memory empty;
        if (order.otype == OrderType.Standing) {
            // Standing orders.
            if (order.mode == OrderMode.Partial) {
                PartialStandingOrder memory outOrder = PartialStandingOrder({
                    minAmountIn: order.minAmountIn,
                    maxAmountIn: order.amountSpecified,
                    minPrice: order.minPrice,
                    useInternal: order.useInternal,
                    assetIn: assetIn,
                    assetOut: assetOut,
                    recipient: order.recipient,
                    hookData: order.hook == address(0) ? new bytes(0) : abi.encodePacked(order.hook, order.hookPayload),
                    nonce: order.nonce,
                    deadline: order.deadline,
                    amountFilled: order.amountFilled,
                    meta: empty
                });
                if (VERBOSE) console.log("order:\n%s", outOrder.toStr());
                return outOrder.hash();
            } else {
                assert(order.mode == OrderMode.ExactIn || order.mode == OrderMode.ExactOut);
                ExactStandingOrder memory outOrder = ExactStandingOrder({
                    exactIn: order.mode == OrderMode.ExactIn,
                    amount: order.amountSpecified,
                    minPrice: order.minPrice,
                    useInternal: order.useInternal,
                    assetIn: assetIn,
                    assetOut: assetOut,
                    recipient: order.recipient,
                    hookData: order.hook == address(0) ? new bytes(0) : abi.encodePacked(order.hook, order.hookPayload),
                    nonce: order.nonce,
                    deadline: order.deadline,
                    meta: empty
                });
                if (VERBOSE) console.log("order:\n%s", outOrder.toStr());
                return outOrder.hash();
            }
        } else {
            // Flash orders.
            if (order.mode == OrderMode.Partial) {
                PartialFlashOrder memory outOrder = PartialFlashOrder({
                    minAmountIn: order.minAmountIn,
                    maxAmountIn: order.amountSpecified,
                    minPrice: order.minPrice,
                    useInternal: order.useInternal,
                    assetIn: assetIn,
                    assetOut: assetOut,
                    recipient: order.recipient,
                    hookData: order.hook == address(0) ? new bytes(0) : abi.encodePacked(order.hook, order.hookPayload),
                    validForBlock: uint64(block.number),
                    amountFilled: order.amountFilled,
                    meta: empty
                });
                if (VERBOSE) console.log("order:\n%s", outOrder.toStr());
                return outOrder.hash();
            } else {
                assert(order.mode == OrderMode.ExactIn || order.mode == OrderMode.ExactOut);
                ExactFlashOrder memory outOrder = ExactFlashOrder({
                    exactIn: order.mode == OrderMode.ExactIn,
                    amount: order.amountSpecified,
                    minPrice: order.minPrice,
                    useInternal: order.useInternal,
                    assetIn: assetIn,
                    assetOut: assetOut,
                    recipient: order.recipient,
                    hookData: order.hook == address(0) ? new bytes(0) : abi.encodePacked(order.hook, order.hookPayload),
                    validForBlock: uint64(block.number),
                    meta: empty
                });
                if (VERBOSE) console.log("order:\n%s", outOrder.toStr());
                return outOrder.hash();
            }
        }
    }
}
