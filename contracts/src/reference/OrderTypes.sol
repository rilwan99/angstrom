// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {AssetIndex} from "../types/PriceGraph.sol";
import {OrderBufferLib} from "../types/OrderBuffer.sol";
import {SafeCastLib} from "solady/src/utils/SafeCastLib.sol";

import {FormatLib} from "super-sol/libraries/FormatLib.sol";

import {console} from "forge-std/console.sol";

struct OrderMeta {
    bool isEcdsa;
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
    address hook;
    bytes hookPayload;
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
    address hook;
    bytes hookPayload;
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
    address hook;
    bytes hookPayload;
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
    address hook;
    bytes hookPayload;
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
    address hook;
    bytes hookPayload;
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
    using SafeCastLib for *;

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
            _logB(
                abi.encode(
                    PARTIAL_STANDING_ORDER_TYPEHASH,
                    order.minAmountIn,
                    order.maxAmountIn,
                    order.minPrice,
                    order.useInternal,
                    order.assetIn,
                    order.assetOut,
                    order.recipient,
                    keccak256(_toHookData(order.hook, order.hookPayload)),
                    order.nonce,
                    order.deadline
                )
            )
        );
    }

    function hash(ExactStandingOrder memory order) internal pure returns (bytes32) {
        return keccak256(
            _logB(
                abi.encode(
                    EXACT_STANDING_ORDER_TYPEHASH,
                    order.exactIn,
                    order.amount,
                    order.minPrice,
                    order.useInternal,
                    order.assetIn,
                    order.assetOut,
                    order.recipient,
                    keccak256(_toHookData(order.hook, order.hookPayload)),
                    order.nonce,
                    order.deadline
                )
            )
        );
    }

    function hash(PartialFlashOrder memory order) internal pure returns (bytes32) {
        return keccak256(
            _logB(
                abi.encode(
                    PARTIAL_FLASH_ORDER_TYPEHASH,
                    order.minAmountIn,
                    order.maxAmountIn,
                    order.minPrice,
                    order.useInternal,
                    order.assetIn,
                    order.assetOut,
                    order.recipient,
                    keccak256(_toHookData(order.hook, order.hookPayload)),
                    order.validForBlock
                )
            )
        );
    }

    function hash(ExactFlashOrder memory order) internal pure returns (bytes32) {
        return keccak256(
            _logB(
                abi.encode(
                    EXACT_FLASH_ORDER_TYPEHASH,
                    order.exactIn,
                    order.amount,
                    order.minPrice,
                    order.useInternal,
                    order.assetIn,
                    order.assetOut,
                    order.recipient,
                    keccak256(_toHookData(order.hook, order.hookPayload)),
                    order.validForBlock
                )
            )
        );
    }

    function _logB(bytes memory b) internal pure returns (bytes memory) {
        console.logBytes(b);
        return b;
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
                keccak256(_toHookData(order.hook, order.hookPayload)),
                order.validForBlock
            )
        );
    }

    function encode(PartialStandingOrder memory order, address[] memory assets) internal pure returns (bytes memory) {
        // forgefmt: disable-next-item
        uint256 variantMap =
            OrderBufferLib.VARIANT_IS_EXACT_BIT * 0 |
            OrderBufferLib.VARIANT_IS_FLASH_BIT * 0 |
            OrderBufferLib.VARIANT_IS_OUT_BIT * 0 |
            OrderBufferLib.VARIANT_NO_HOOK_BIT * (order.hook == address(0) ? 1 : 0) |
            OrderBufferLib.VARIANT_USE_INTERNAL_BIT * (order.useInternal ? 1 : 0) |
            OrderBufferLib.VARIANT_HAS_RECIPIENT * (order.recipient != address(0) ? 1 : 0) |
            OrderBufferLib.VARIANT_ECDSA_SIG * (order.meta.isEcdsa ? 1 : 0);
        AssetIndex assetInIndex = _toAssetIndex(assets, order.assetIn);
        AssetIndex assetOutIndex = _toAssetIndex(assets, order.assetOut);

        return bytes.concat(
            // Stack too deep
            bytes.concat(
                bytes1(uint8(variantMap)),
                bytes2(AssetIndex.unwrap(assetInIndex)),
                bytes2(AssetIndex.unwrap(assetOutIndex)),
                bytes32(order.minPrice),
                _encodeHookData(order.hook, order.hookPayload),
                bytes8(order.nonce)
            ),
            bytes5(order.deadline),
            bytes16(order.minAmountIn.toUint128()),
            bytes16(order.maxAmountIn.toUint128()),
            bytes16(order.amountFilled.toUint128()),
            order.recipient == address(0) ? new bytes(0) : bytes.concat(bytes20(order.recipient)),
            _encodeSig(order.meta)
        );
    }

    function encode(ExactStandingOrder memory order, address[] memory assets) internal pure returns (bytes memory) {
        // forgefmt: disable-next-item
        uint256 variantMap =
            OrderBufferLib.VARIANT_IS_EXACT_BIT * 1 |
            OrderBufferLib.VARIANT_IS_FLASH_BIT * 0 |
            OrderBufferLib.VARIANT_IS_OUT_BIT * (!order.exactIn ? 1 : 0) |
            OrderBufferLib.VARIANT_NO_HOOK_BIT * (order.hook == address(0) ? 1 : 0) |
            OrderBufferLib.VARIANT_USE_INTERNAL_BIT * (order.useInternal ? 1 : 0) |
            OrderBufferLib.VARIANT_HAS_RECIPIENT * (order.recipient != address(0) ? 1 : 0) |
            OrderBufferLib.VARIANT_ECDSA_SIG * (order.meta.isEcdsa ? 1 : 0);
        AssetIndex assetInIndex = _toAssetIndex(assets, order.assetIn);
        AssetIndex assetOutIndex = _toAssetIndex(assets, order.assetOut);

        return bytes.concat(
            bytes1(uint8(variantMap)),
            bytes2(AssetIndex.unwrap(assetInIndex)),
            bytes2(AssetIndex.unwrap(assetOutIndex)),
            bytes32(order.minPrice),
            _encodeHookData(order.hook, order.hookPayload),
            bytes8(order.nonce),
            bytes5(order.deadline),
            bytes16(order.amount.toUint128()),
            order.recipient == address(0) ? new bytes(0) : bytes.concat(bytes20(order.recipient)),
            _encodeSig(order.meta)
        );
    }

    function encode(PartialFlashOrder memory order, address[] memory assets) internal pure returns (bytes memory) {
        // forgefmt: disable-next-item
        uint256 variantMap =
            OrderBufferLib.VARIANT_IS_EXACT_BIT * 0 |
            OrderBufferLib.VARIANT_IS_FLASH_BIT * 1 |
            OrderBufferLib.VARIANT_IS_OUT_BIT * 0 |
            OrderBufferLib.VARIANT_NO_HOOK_BIT * (order.hook == address(0) ? 1 : 0) |
            OrderBufferLib.VARIANT_USE_INTERNAL_BIT * (order.useInternal ? 1 : 0) |
            OrderBufferLib.VARIANT_HAS_RECIPIENT * (order.recipient != address(0) ? 1 : 0) |
            OrderBufferLib.VARIANT_ECDSA_SIG * (order.meta.isEcdsa ? 1 : 0);
        AssetIndex assetInIndex = _toAssetIndex(assets, order.assetIn);
        AssetIndex assetOutIndex = _toAssetIndex(assets, order.assetOut);

        return bytes.concat(
            bytes1(uint8(variantMap)),
            bytes2(AssetIndex.unwrap(assetInIndex)),
            bytes2(AssetIndex.unwrap(assetOutIndex)),
            bytes32(order.minPrice),
            _encodeHookData(order.hook, order.hookPayload),
            bytes16(order.minAmountIn.toUint128()),
            bytes16(order.maxAmountIn.toUint128()),
            bytes16(order.amountFilled.toUint128()),
            order.recipient == address(0) ? new bytes(0) : bytes.concat(bytes20(order.recipient)),
            _encodeSig(order.meta)
        );
    }

    function encode(ExactFlashOrder memory order, address[] memory assets) internal pure returns (bytes memory) {
        // forgefmt: disable-next-item
        uint256 variantMap =
            OrderBufferLib.VARIANT_IS_EXACT_BIT * 1 |
            OrderBufferLib.VARIANT_IS_FLASH_BIT * 1 |
            OrderBufferLib.VARIANT_IS_OUT_BIT * (!order.exactIn ? 1 : 0) |
            OrderBufferLib.VARIANT_NO_HOOK_BIT * (order.hook == address(0) ? 1 : 0) |
            OrderBufferLib.VARIANT_USE_INTERNAL_BIT * (order.useInternal ? 1 : 0) |
            OrderBufferLib.VARIANT_HAS_RECIPIENT * (order.recipient != address(0) ? 1 : 0) |
            OrderBufferLib.VARIANT_ECDSA_SIG * (order.meta.isEcdsa ? 1 : 0);
        AssetIndex assetInIndex = _toAssetIndex(assets, order.assetIn);
        AssetIndex assetOutIndex = _toAssetIndex(assets, order.assetOut);

        return bytes.concat(
            bytes1(uint8(variantMap)),
            bytes2(AssetIndex.unwrap(assetInIndex)),
            bytes2(AssetIndex.unwrap(assetOutIndex)),
            bytes32(order.minPrice),
            _encodeHookData(order.hook, order.hookPayload),
            bytes16(order.amount.toUint128()),
            order.recipient == address(0) ? new bytes(0) : bytes.concat(bytes20(order.recipient)),
            _encodeSig(order.meta)
        );
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
            ",\n  hook: ",
            o.hook.toStr(),
            ",\n  hookPayload: ",
            o.hookPayload.toStr(),
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
            ",\n  hook: ",
            o.hook.toStr(),
            ",\n  hookPayload: ",
            o.hookPayload.toStr(),
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
            ",\n  hook: ",
            o.hook.toStr(),
            ",\n  hookPayload: ",
            o.hookPayload.toStr(),
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
            ",\n  hook: ",
            o.hook.toStr(),
            ",\n  hookPayload: ",
            o.hookPayload.toStr(),
            ",\n  validForBlock: ",
            o.validForBlock.toStr(),
            ",\n  meta: ",
            o.meta.toStr(),
            "\n}"
        );
    }

    function toStr(OrderMeta memory meta) internal pure returns (string memory) {
        return string.concat(
            "OrderMeta { isEcdsa: ",
            meta.isEcdsa.toStr(),
            ", from: ",
            meta.from.toStr(),
            ", signature: ",
            meta.signature.toStr(),
            " }"
        );
    }

    function _encodeHookData(address hook, bytes memory hookPayload) internal pure returns (bytes memory) {
        if (hook == address(0)) {
            return new bytes(0);
        }
        return bytes.concat(bytes20(hook), bytes3(hookPayload.length.toUint24()), hookPayload);
    }

    function _toHookData(address hook, bytes memory hookPayload) internal pure returns (bytes memory) {
        if (hook == address(0)) {
            return new bytes(0);
        }
        return bytes.concat(bytes20(hook), hookPayload);
    }

    function _encodeSig(OrderMeta memory meta) internal pure returns (bytes memory) {
        if (meta.isEcdsa) {
            return meta.signature;
        } else {
            // ERC1271
            return bytes.concat(bytes20(meta.from), bytes3(meta.signature.length.toUint24()), meta.signature);
        }
    }

    function _toAssetIndex(address[] memory assets, address asset) internal pure returns (AssetIndex) {
        for (uint16 i = 0; i < assets.length; i++) {
            if (assets[i] == asset) return AssetIndex.wrap(i);
        }
        revert("Asset not found");
    }
}

/*
0x
5b9d49cfed48f8c9d1a863f61c2479c579fa299c25a33211fc857390cecce6d4
0000000000000000000000000000000000000000000000000000000000000001
00000000000000000000000000000000000000000000000052c6824e16328345
0000000000000000000000000000000000000000006c2d47d8d2a960edd79e55
0000000000000000000000000000000000000000000000000000000000000001
0000000000000000000000005615deb798bb3e4dfa0139dfa1b3d433cc23b72f
0000000000000000000000002e234dae75c793f67a35089c9d99245e1c58470b
000000000000000000000000ad6c344738890956752bdc5f934619771a38ae63
c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470
0000000000000000000000000000000000000000000000000000000000000002



0x
5b9d49cfed48f8c9d1a863f61c2479c579fa299c25a33211fc857390cecce6d4
0000000000000000000000000000000000000000000000000000000000000001
00000000000000000000000000000000000000000000000052c6824e16328345
0000000000000000000000000000000000000000006c2d47d8d2a960edd79e55
0000000000000000000000000000000000000000000000000000000000000001
0000000000000000000000005615deb798bb3e4dfa0139dfa1b3d433cc23b72f
0000000000000000000000002e234dae75c793f67a35089c9d99245e1c58470b
000000000000000000000000ad6c344738890956752bdc5f934619771a38ae63
c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470
0000000000000000000000000000000000000000000000000000000000000002
*/
