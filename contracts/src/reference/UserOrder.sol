// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {
    PartialStandingOrder,
    ExactStandingOrder,
    PartialFlashOrder,
    ExactFlashOrder,
    OrderMeta,
    OrdersLib
} from "./OrderTypes.sol";
import {GenericOrder} from "../types/OrderTypes.sol";
import {TypedDataHasher} from "../types/TypedDataHasher.sol";

type UserOrder is uint256;

enum OrderVariant {
    PartialStandingOrder,
    ExactStandingOrder,
    PartialFlashOrder,
    ExactFlashOrder
}

using UserOrderLib for UserOrder global;
using UserOrderLib for OrderVariant global;

/// @author philogy <https://github.com/philogy>
library UserOrderLib {
    using UserOrderLib for function(UserOrder) internal pure returns (PartialFlashOrder memory);

    function from(PartialStandingOrder memory spOrder) internal pure returns (UserOrder order) {
        OrderVariant variant = OrderVariant.PartialStandingOrder;
        assembly ("memory-safe") {
            order := or(shl(8, spOrder), variant)
        }
    }

    function from(ExactStandingOrder memory spOrder) internal pure returns (UserOrder order) {
        OrderVariant variant = OrderVariant.ExactStandingOrder;
        assembly ("memory-safe") {
            order := or(shl(8, spOrder), variant)
        }
    }

    function from(PartialFlashOrder memory spOrder) internal pure returns (UserOrder order) {
        OrderVariant variant = OrderVariant.PartialFlashOrder;
        assembly ("memory-safe") {
            order := or(shl(8, spOrder), variant)
        }
    }

    function from(ExactFlashOrder memory spOrder) internal pure returns (UserOrder order) {
        OrderVariant variant = OrderVariant.ExactFlashOrder;
        assembly ("memory-safe") {
            order := or(shl(8, spOrder), variant)
        }
    }

    function getVariant(UserOrder order) internal pure returns (OrderVariant variant) {
        assembly {
            variant := and(order, 0xff)
        }
    }

    function setMeta(UserOrder order, OrderMeta memory meta) internal pure {
        OrderVariant variant = order.getVariant();
        if (variant == OrderVariant.PartialStandingOrder) {
            _toPartialStandingFn(_toMemPtr)(order).meta = meta;
        } else if (variant == OrderVariant.ExactStandingOrder) {
            _toExactStandingFn(_toMemPtr)(order).meta = meta;
        } else if (variant == OrderVariant.PartialFlashOrder) {
            _toPartialFlashFn(_toMemPtr)(order).meta = meta;
        } else if (variant == OrderVariant.ExactFlashOrder) {
            _toExactFlashFn(_toMemPtr)(order).meta = meta;
        } else {
            revert("Unimplemented variant");
        }
    }

    function hash712(UserOrder order, TypedDataHasher hasher) internal pure returns (bytes32) {
        return hasher.hashTypedData(order.hash());
    }

    function hash(UserOrder order) internal pure returns (bytes32) {
        OrderVariant variant = order.getVariant();
        if (variant == OrderVariant.PartialStandingOrder) {
            return _toPartialStandingFn(_toMemPtr)(order).hash();
        } else if (variant == OrderVariant.ExactStandingOrder) {
            return _toExactStandingFn(_toMemPtr)(order).hash();
        } else if (variant == OrderVariant.PartialFlashOrder) {
            return _toPartialFlashFn(_toMemPtr)(order).hash();
        } else if (variant == OrderVariant.ExactFlashOrder) {
            return _toExactFlashFn(_toMemPtr)(order).hash();
        } else {
            revert("Unimplemented variant");
        }
    }

    function into(UserOrder order, address[] memory assets) internal pure returns (GenericOrder memory g) {
        OrderVariant variant = order.getVariant();
        if (variant == OrderVariant.PartialStandingOrder) {
            _toPartialStandingFn(_toMemPtr)(order).setGeneric(g, assets);
        } else if (variant == OrderVariant.ExactStandingOrder) {
            _toExactStandingFn(_toMemPtr)(order).setGeneric(g, assets);
        } else if (variant == OrderVariant.PartialFlashOrder) {
            _toPartialFlashFn(_toMemPtr)(order).setGeneric(g, assets);
        } else if (variant == OrderVariant.ExactFlashOrder) {
            _toExactFlashFn(_toMemPtr)(order).setGeneric(g, assets);
        } else {
            revert("Unimplemented variant");
        }
    }

    function toStr(UserOrder order) internal pure returns (string memory) {
        OrderVariant variant = order.getVariant();
        if (variant == OrderVariant.PartialStandingOrder) {
            return _toPartialStandingFn(_toMemPtr)(order).toStr();
        } else if (variant == OrderVariant.ExactStandingOrder) {
            return _toExactStandingFn(_toMemPtr)(order).toStr();
        } else if (variant == OrderVariant.PartialFlashOrder) {
            return _toPartialFlashFn(_toMemPtr)(order).toStr();
        } else if (variant == OrderVariant.ExactFlashOrder) {
            return _toExactFlashFn(_toMemPtr)(order).toStr();
        } else {
            revert("Unimplemented variant");
        }
    }

    function _toPartialStandingFn(function(UserOrder) internal pure returns (bytes32) fnIn)
        private
        pure
        returns (function(UserOrder) internal pure returns (PartialStandingOrder memory) fnOut)
    {
        assembly {
            fnOut := fnIn
        }
    }

    function _toExactStandingFn(function(UserOrder) internal pure returns (bytes32) fnIn)
        private
        pure
        returns (function(UserOrder) internal pure returns (ExactStandingOrder memory) fnOut)
    {
        assembly {
            fnOut := fnIn
        }
    }

    function _toPartialFlashFn(function(UserOrder) internal pure returns (bytes32) fnIn)
        private
        pure
        returns (function(UserOrder) internal pure returns (PartialFlashOrder memory) fnOut)
    {
        assembly {
            fnOut := fnIn
        }
    }

    function _toExactFlashFn(function(UserOrder) internal pure returns (bytes32) fnIn)
        private
        pure
        returns (function(UserOrder) internal pure returns (ExactFlashOrder memory) fnOut)
    {
        assembly {
            fnOut := fnIn
        }
    }

    function _toMemPtr(UserOrder order) private pure returns (bytes32 ptr) {
        assembly ("memory-safe") {
            ptr := shr(8, order)
        }
    }

    function toStr(OrderVariant variant) internal pure returns (string memory) {
        if (variant == OrderVariant.PartialStandingOrder) {
            return "OrderVariant::PartialStandingOrder";
        } else if (variant == OrderVariant.ExactStandingOrder) {
            return "OrderVariant::ExactStandingOrder";
        } else if (variant == OrderVariant.PartialFlashOrder) {
            return "OrderVariant::PartialFlashOrder";
        } else if (variant == OrderVariant.ExactFlashOrder) {
            return "OrderVariant::ExactFlashOrder";
        } else {
            revert("Unimplemented variant");
        }
    }
}
