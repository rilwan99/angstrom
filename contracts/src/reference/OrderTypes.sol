// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {AssetIndex} from "../types/PriceGraph.sol";
import {GenericOrder, OrderMode, OrderType} from "../types/OrderTypes.sol";

import {FormatLib} from "super-sol/libraries/FormatLib.sol";

struct OrderMeta {
    address from;
    bytes signature;
}

struct PartialStandingOrder {
    uint256 minAmountIn;
    uint256 maxAmountIn;
    uint256 minPrice;
    bool useInternal;
    address assetIn;
    address assetOut;
    address recipient;
    bytes hookData;
    uint64 nonce;
    uint40 deadline;
    uint256 amountFilled;
    OrderMeta meta;
}

struct ExactStandingOrder {
    bool exactIn;
    uint256 amount;
    uint256 minPrice;
    bool useInternal;
    address assetIn;
    address assetOut;
    address recipient;
    bytes hookData;
    uint64 nonce;
    uint40 deadline;
    OrderMeta meta;
}

struct PartialFlashOrder {
    uint256 minAmountIn;
    uint256 maxAmountIn;
    uint256 minPrice;
    bool useInternal;
    address assetIn;
    address assetOut;
    address recipient;
    bytes hookData;
    uint64 validForBlock;
    uint256 amountFilled;
    OrderMeta meta;
}

struct ExactFlashOrder {
    bool exactIn;
    uint256 amount;
    uint256 minPrice;
    bool useInternal;
    address assetIn;
    address assetOut;
    address recipient;
    bytes hookData;
    uint64 validForBlock;
    OrderMeta meta;
}

struct TopOfBlockOrder {
    uint256 amountIn;
    uint256 amountOut;
    bool useInternal;
    address assetIn;
    address assetOut;
    address recipient;
    bytes hookData;
    uint64 validForBlock;
    OrderMeta meta;
}

using OrdersLib for OrderMeta global;
using OrdersLib for PartialStandingOrder global;
using OrdersLib for ExactStandingOrder global;
using OrdersLib for PartialFlashOrder global;
using OrdersLib for ExactFlashOrder global;
using OrdersLib for TopOfBlockOrder global;

library OrdersLib {
    using FormatLib for *;

    /// forgefmt: disable-next-item
    bytes32 internal constant PARTIAL_STANDING_ORDER_TYPEHASH = keccak256(
        "PartialStandingOrder("
           "uint256 min_amount_in,"
           "uint256 max_amount_in,"
           "uint256 min_price,"
           "bool use_internal,"
           "address asset_in,"
           "address asset_out,"
           "address recipient,"
           "bytes hook_data,"
           "uint64 nonce,"
           "uint40 deadline"
        ")"
    );

    /// forgefmt: disable-next-item
    bytes32 internal constant EXACT_STANDING_ORDER_TYPEHASH = keccak256(
        "ExactStandingOrder("
           "bool exact_in,"
           "uint256 amount,"
           "uint256 min_price,"
           "bool use_internal,"
           "address asset_in,"
           "address asset_out,"
           "address recipient,"
           "bytes hook_data,"
           "uint64 nonce,"
           "uint40 deadline"
        ")"
    );

    /// forgefmt: disable-next-item
    bytes32 internal constant PARTIAL_FLASH_ORDER_TYPEHASH = keccak256(
        "PartialFlashOrder("
           "uint256 min_amount_in,"
           "uint256 max_amount_in,"
           "uint256 min_price,"
           "bool use_internal,"
           "address asset_in,"
           "address asset_out,"
           "address recipient,"
           "bytes hook_data,"
           "uint64 valid_for_block"
        ")"
    );

    /// forgefmt: disable-next-item
    bytes32 internal constant EXACT_FLASH_ORDER_TYPEHASH = keccak256(
        "ExactFlashOrder("
           "bool exact_in,"
           "uint256 amount,"
           "uint256 min_price,"
           "bool use_internal,"
           "address asset_in,"
           "address asset_out,"
           "address recipient,"
           "bytes hook_data,"
           "uint64 valid_for_block"
        ")"
    );

    /// forgefmt: disable-next-item
    bytes32 internal constant TOP_OF_BLOCK_ORDER_TYPEHASH = keccak256(
        "TopOfBlockOrder("
           "uint256 amount_in,"
           "uint256 amount_out,"
           "bool use_internal,"
           "address asset_in,"
           "address asset_out,"
           "address recipient,"
           "bytes hook_data,"
           "uint256 valid_for_block"
        ")"
    );

    function hash(PartialStandingOrder memory order) internal pure returns (bytes32) {
        return keccak256(
            abi.encode(
                PARTIAL_STANDING_ORDER_TYPEHASH,
                order.minAmountIn,
                order.maxAmountIn,
                order.minPrice,
                order.useInternal,
                order.assetIn,
                order.assetOut,
                order.recipient,
                keccak256(order.hookData),
                order.nonce,
                order.deadline
            )
        );
    }

    function hash(ExactStandingOrder memory order) internal pure returns (bytes32) {
        return keccak256(
            abi.encode(
                EXACT_STANDING_ORDER_TYPEHASH,
                order.exactIn,
                order.amount,
                order.minPrice,
                order.useInternal,
                order.assetIn,
                order.assetOut,
                order.recipient,
                keccak256(order.hookData),
                order.nonce,
                order.deadline
            )
        );
    }

    function hash(PartialFlashOrder memory order) internal pure returns (bytes32) {
        return keccak256(
            abi.encode(
                PARTIAL_FLASH_ORDER_TYPEHASH,
                order.minAmountIn,
                order.maxAmountIn,
                order.minPrice,
                order.useInternal,
                order.assetIn,
                order.assetOut,
                order.recipient,
                keccak256(order.hookData),
                order.validForBlock
            )
        );
    }

    function hash(ExactFlashOrder memory order) internal pure returns (bytes32) {
        return keccak256(
            abi.encode(
                EXACT_FLASH_ORDER_TYPEHASH,
                order.exactIn,
                order.amount,
                order.minPrice,
                order.useInternal,
                order.assetIn,
                order.assetOut,
                order.recipient,
                keccak256(order.hookData),
                order.validForBlock
            )
        );
    }

    function hash(TopOfBlockOrder memory order) internal pure returns (bytes32) {
        return keccak256(
            abi.encode(
                TOP_OF_BLOCK_ORDER_TYPEHASH,
                order.amountIn,
                order.amountOut,
                order.useInternal,
                order.assetIn,
                order.assetOut,
                order.recipient,
                keccak256(order.hookData),
                order.validForBlock
            )
        );
    }

    function setGeneric(PartialStandingOrder memory o, GenericOrder memory g, address[] memory assets) internal pure {
        g.otype = OrderType.Standing;
        g.mode = OrderMode.Partial;
        g.minAmountIn = o.minAmountIn;
        g.amountSpecified = o.maxAmountIn;
        g.minPrice = o.minPrice;
        g.useInternal = o.useInternal;
        g.assetInIndex = _toAssetIndex(assets, o.assetIn);
        g.assetOutIndex = _toAssetIndex(assets, o.assetOut);
        g.nonce = o.nonce;
        g.deadline = o.deadline;
        g.recipient = o.recipient;
        _decodeHookData(g, o.hookData);
        g.amountFilled = o.amountFilled;
        g.from = o.meta.from;
        g.signature = o.meta.signature;
    }

    function setGeneric(ExactStandingOrder memory o, GenericOrder memory g, address[] memory assets) internal pure {
        g.otype = OrderType.Standing;
        g.mode = o.exactIn ? OrderMode.ExactIn : OrderMode.ExactOut;
        g.amountSpecified = o.amount;
        g.minPrice = o.minPrice;
        g.useInternal = o.useInternal;
        g.assetInIndex = _toAssetIndex(assets, o.assetIn);
        g.assetOutIndex = _toAssetIndex(assets, o.assetOut);
        g.nonce = o.nonce;
        g.deadline = o.deadline;
        g.recipient = o.recipient;
        _decodeHookData(g, o.hookData);
        g.from = o.meta.from;
        g.signature = o.meta.signature;
    }

    function setGeneric(PartialFlashOrder memory o, GenericOrder memory g, address[] memory assets) internal pure {
        g.otype = OrderType.Flash;
        g.mode = OrderMode.Partial;
        g.minAmountIn = o.minAmountIn;
        g.amountSpecified = o.maxAmountIn;
        g.minPrice = o.minPrice;
        g.useInternal = o.useInternal;
        g.assetInIndex = _toAssetIndex(assets, o.assetIn);
        g.assetOutIndex = _toAssetIndex(assets, o.assetOut);
        g.recipient = o.recipient;
        _decodeHookData(g, o.hookData);
        g.amountFilled = o.amountFilled;
        g.from = o.meta.from;
        g.signature = o.meta.signature;
    }

    function setGeneric(ExactFlashOrder memory o, GenericOrder memory g, address[] memory assets) internal pure {
        g.otype = OrderType.Flash;
        g.mode = o.exactIn ? OrderMode.ExactIn : OrderMode.ExactOut;
        g.amountSpecified = o.amount;
        g.minPrice = o.minPrice;
        g.useInternal = o.useInternal;
        g.assetInIndex = _toAssetIndex(assets, o.assetIn);
        g.assetOutIndex = _toAssetIndex(assets, o.assetOut);
        g.recipient = o.recipient;
        _decodeHookData(g, o.hookData);
        g.from = o.meta.from;
        g.signature = o.meta.signature;
    }

    function toStr(PartialStandingOrder memory o) internal pure returns (string memory str) {
        str = string.concat(
            "PartialStandingOrder {",
            "\n  minAmountIn: ",
            o.minAmountIn.toStr(),
            ",\n  maxAmountIn: ",
            o.maxAmountIn.toStr(),
            ",\n  minPrice: ",
            o.minPrice.toStr(),
            ",\n  useInternal: ",
            o.useInternal.toStr(),
            ",\n  assetIn: ",
            o.assetIn.toStr(),
            ",\n  assetOut: ",
            o.assetOut.toStr(),
            ",\n  recipient: "
        );
        str = string.concat(
            str,
            o.recipient.toStr(),
            ",\n  hookData: ",
            o.hookData.toStr(),
            ",\n  nonce: ",
            o.nonce.toStr(),
            ",\n  deadline: ",
            o.deadline.toStr(),
            ",\n  amountFilled: ",
            o.amountFilled.toStr(),
            ",\n  meta: ",
            o.meta.toStr(),
            "\n}"
        );
    }

    function toStr(ExactStandingOrder memory o) internal pure returns (string memory str) {
        str = string.concat(
            "ExactStandingOrder {",
            "\n  exactIn: ",
            o.exactIn.toStr(),
            ",\n  amount: ",
            o.amount.toStr(),
            ",\n  minPrice: ",
            o.minPrice.toStr(),
            ",\n  useInternal: ",
            o.useInternal.toStr(),
            ",\n  assetIn: ",
            o.assetIn.toStr(),
            ",\n  assetOut: ",
            o.assetOut.toStr()
        );
        str = string.concat(
            str,
            ",\n  recipient: ",
            o.recipient.toStr(),
            ",\n  hookData: ",
            o.hookData.toStr(),
            ",\n  nonce: ",
            o.nonce.toStr(),
            ",\n  deadline: ",
            o.deadline.toStr(),
            ",\n  meta: ",
            o.meta.toStr(),
            "\n}"
        );
    }

    function toStr(PartialFlashOrder memory o) internal pure returns (string memory str) {
        str = string.concat(
            "PartialFlashOrder {",
            "\n  minAmountIn: ",
            o.minAmountIn.toStr(),
            ",\n  maxAmountIn: ",
            o.maxAmountIn.toStr(),
            ",\n  minPrice: ",
            o.minPrice.toStr(),
            ",\n  useInternal: ",
            o.useInternal.toStr(),
            ",\n  assetIn: ",
            o.assetIn.toStr(),
            ",\n  assetOut: ",
            o.assetOut.toStr()
        );
        str = string.concat(
            str,
            ",\n  recipient: ",
            o.recipient.toStr(),
            ",\n  hookData: ",
            o.hookData.toStr(),
            ",\n  validForBlock: ",
            o.validForBlock.toStr(),
            ",\n  amountFilled: ",
            o.amountFilled.toStr(),
            ",\n  meta: ",
            o.meta.toStr(),
            "\n}"
        );
    }

    function toStr(ExactFlashOrder memory o) internal pure returns (string memory str) {
        str = string.concat(
            "ExactFlashOrder {",
            "\n  exactIn: ",
            o.exactIn.toStr(),
            ",\n  amount: ",
            o.amount.toStr(),
            ",\n  minPrice: ",
            o.minPrice.toStr(),
            ",\n  useInternal: ",
            o.useInternal.toStr(),
            ",\n  assetIn: ",
            o.assetIn.toStr(),
            ",\n  assetOut: "
        );
        str = string.concat(
            str,
            o.assetOut.toStr(),
            ",\n  recipient: ",
            o.recipient.toStr(),
            ",\n  hookData: ",
            o.hookData.toStr(),
            ",\n  validForBlock: ",
            o.validForBlock.toStr(),
            ",\n  meta: ",
            o.meta.toStr(),
            "\n}"
        );
    }

    function toStr(OrderMeta memory meta) internal pure returns (string memory) {
        return string.concat("OrderMeta {", " from: ", meta.from.toStr(), ", signature: ", meta.signature.toStr(), " }");
    }

    function _decodeHookData(GenericOrder memory g, bytes memory data) internal pure {
        uint256 dataLength = data.length;
        if (dataLength != 0) {
            require(dataLength >= 20, "Invalid hookData length");
            address hook;
            bytes memory hookPayload;
            assembly ("memory-safe") {
                hook := mload(add(data, 20))
                let len := sub(mload(data), 20)

                hookPayload := mload(0x40)
                let offset := add(hookPayload, 0x20)
                mstore(0x40, add(offset, len))

                mstore(hookPayload, len)
                mcopy(offset, add(data, 52), len)
            }
            g.hook = hook;
            g.hookPayload = hookPayload;
        }
    }

    function _toAssetIndex(address[] memory assets, address asset) internal pure returns (AssetIndex) {
        for (uint16 i = 0; i < assets.length; i++) {
            if (assets[i] == asset) return AssetIndex.wrap(i);
        }
        revert("Asset not found");
    }
}
