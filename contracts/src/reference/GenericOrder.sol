// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {
    PartialStandingOrder,
    ExactStandingOrder,
    PartialFlashOrder,
    ExactFlashOrder,
    TopOfBlockOrder
} from "./OrderTypes.sol";

type GenericOrder is uint256;

using GenericOrderLib for GenericOrder global;

/// @author philogy <https://github.com/philogy>
library GenericOrderLib {
    enum OrderVariant {
        PartialStandingOrder,
        ExactStandingOrder,
        PartialFlashOrder,
        ExactFlashOrder,
        TopOfBlockOrder
    }

    function from(PartialStandingOrder memory spOrder) internal pure returns (GenericOrder order) {
        OrderVariant variant = OrderVariant.PartialStandingOrder;
        assembly ("memory-safe") {
            order := or(shl(8, spOrder), variant)
        }
    }

    function from(ExactStandingOrder memory spOrder) internal pure returns (GenericOrder order) {
        OrderVariant variant = OrderVariant.ExactStandingOrder;
        assembly ("memory-safe") {
            order := or(shl(8, spOrder), variant)
        }
    }

    function from(PartialFlashOrder memory spOrder) internal pure returns (GenericOrder order) {
        OrderVariant variant = OrderVariant.PartialFlashOrder;
        assembly ("memory-safe") {
            order := or(shl(8, spOrder), variant)
        }
    }

    function from(ExactFlashOrder memory spOrder) internal pure returns (GenericOrder order) {
        OrderVariant variant = OrderVariant.ExactFlashOrder;
        assembly ("memory-safe") {
            order := or(shl(8, spOrder), variant)
        }
    }

    function from(TopOfBlockOrder memory spOrder) internal pure returns (GenericOrder order) {
        OrderVariant variant = OrderVariant.TopOfBlockOrder;
        assembly ("memory-safe") {
            order := or(shl(8, spOrder), variant)
        }
    }

    function init() internal pure returns (GenericOrder) {
        PartialStandingOrder memory empty;
        return from(empty);
    }

    function set(
        GenericOrder order,
        uint256 minPrice,
        address assetIn,
        address assetOut,
        address recipient,
        bytes memory hookData
    ) internal pure returns (GenericOrder) {
        PartialStandingOrder memory inner = _toPartialStandingFn(_toMemPtr)(order);
        inner.minPrice = minPrice;
        inner.assetIn = assetIn;
        inner.assetOut = assetOut;
        inner.recipient = recipient;
        inner.hookData = hookData;
        return order;
    }

    function setExact(GenericOrder order, bool exactIn, uint256 amount) internal pure returns (GenericOrder) {
        ExactFlashOrder memory inner = _toExactFlashFn(_toMemPtr)(order);
        inner.exactIn = exactIn;

        inner.amount = amount;
        return order;
    }

    function setPartial(GenericOrder order, uint256 minAmountIn, uint256 maxAmountOut)
        internal
        pure
        returns (GenericOrder)
    {
        PartialStandingOrder memory inner = _toPartialStandingFn(_toMemPtr)(order);
        inner.minAmountIn = minAmountIn;
        inner.maxAmountIn = maxAmountOut;
        return order;
    }

    function setStanding(GenericOrder order, uint64 nonce, uint40 deadline) internal pure returns (GenericOrder) {
        PartialStandingOrder memory inner = _toPartialStandingFn(_toMemPtr)(order);
        inner.nonce = nonce;
        inner.deadline = deadline;
        return order;
    }

    function setFlash(GenericOrder order, uint64 validForBlock) internal pure returns (GenericOrder) {
        ExactFlashOrder memory inner = _toExactFlashFn(_toMemPtr)(order);
        inner.validForBlock = validForBlock;
        return order;
    }

    function setVariant(GenericOrder order, OrderVariant variant) internal pure returns (GenericOrder) {
        assembly {
            order := or(and(order, not(0xff)), variant)
        }
        return order;
    }

    function getVariant(GenericOrder order) internal pure returns (OrderVariant variant) {
        assembly {
            variant := and(order, 0xff)
        }
    }

    function _toExactFlashFn(function(GenericOrder) internal pure returns (bytes32) fnIn)
        private
        pure
        returns (function(GenericOrder) internal pure returns (ExactFlashOrder memory) fnOut)
    {
        assembly {
            fnIn := fnOut
        }
    }

    function _toPartialStandingFn(function(GenericOrder) internal pure returns (bytes32) fnIn)
        private
        pure
        returns (function(GenericOrder) internal pure returns (PartialStandingOrder memory) fnOut)
    {
        assembly {
            fnIn := fnOut
        }
    }

    function _toMemPtr(GenericOrder order) private pure returns (bytes32 ptr) {
        assembly ("memory-safe") {
            ptr := shr(8, order)
        }
    }
}
