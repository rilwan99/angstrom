// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {AssetIndex} from "../types/PriceGraph.sol";
import {GenericOrder} from "./GenericOrder.sol";

struct PartialStandingOrder {
    uint256 minAmountIn;
    uint256 maxAmountIn;
    uint256 minPrice;
    address assetIn;
    address assetOut;
    address recipient;
    bytes hookData;
    uint64 nonce;
    uint40 deadline;
}

struct ExactStandingOrder {
    bool exactIn;
    uint256 amount;
    uint256 minPrice;
    address assetIn;
    address assetOut;
    address recipient;
    bytes hookData;
    uint64 nonce;
    uint40 deadline;
}

struct PartialFlashOrder {
    uint256 minAmountIn;
    uint256 maxAmountIn;
    uint256 minPrice;
    address assetIn;
    address assetOut;
    address recipient;
    bytes hookData;
    uint64 validForBlock;
}

struct ExactFlashOrder {
    bool exactIn;
    uint256 amount;
    uint256 minPrice;
    address assetIn;
    address assetOut;
    address recipient;
    bytes hookData;
    uint64 validForBlock;
}

struct TopOfBlockOrder {
    uint256 amountIn;
    uint256 amountOut;
    address assetIn;
    address assetOut;
    address recipient;
    bytes hookData;
    uint64 validForBlock;
}

using OrdersLib for PartialStandingOrder global;
using OrdersLib for ExactStandingOrder global;
using OrdersLib for PartialFlashOrder global;
using OrdersLib for ExactFlashOrder global;
using OrdersLib for TopOfBlockOrder global;

library OrdersLib {
    /// forgefmt: disable-next-item
    bytes32 internal constant PARTIAL_STANDING_ORDER_TYPEHASH = keccak256(
        "PartialStandingOrder("
           "uint256 min_amount_in,"
           "uint256 max_amount_in,"
           "uint256 min_price,"
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
           "address asset_in,"
           "address asset_out,"
           "address recipient,"
           "bytes hook_data,"
           "uint256 valid_for_block"
        ")"
    );

    // function hash(GenericOrder memory order, address assetIn, address assetOut) internal view returns (bytes32) {
    //     if (order.otype == OrderType.Standing) {
    //         return keccak256(
    //             abi.encode(
    //                 STANDING_ORDER_TYPEHASH,
    //                 order.mode.hash(),
    //                 order.minAmountIn,
    //                 order.amountSpecified,
    //                 order.minPrice,
    //                 assetIn,
    //                 assetOut,
    //                 order.recipient,
    //                 _hashHookData(order.hook, order.hookPayload),
    //                 order.nonce,
    //                 order.deadline
    //             )
    //         );
    //     }
    //     if (order.otype == OrderType.Flash) {
    //         return keccak256(
    //             abi.encode(
    //                 FLASH_ORDER_TYPEHASH,
    //                 order.mode.hash(),
    //                 order.minAmountIn,
    //                 order.amountSpecified,
    //                 order.minPrice,
    //                 assetIn,
    //                 assetOut,
    //                 order.recipient,
    //                 _hashHookData(order.hook, order.hookPayload),
    //                 block.number
    //             )
    //         );
    //     }
    //     assert(false);
    //     return bytes32(0);
    // }

    // function hash(TopOfBlockOrderEnvelope memory order, address assetIn, address assetOut)
    //     internal
    //     view
    //     returns (bytes32)
    // {
    //     return keccak256(
    //         abi.encode(
    //             TOP_OF_BLOCK_ORDER_TYPEHASH,
    //             order.amountIn,
    //             order.amountOut,
    //             assetIn,
    //             assetOut,
    //             order.recipient,
    //             _hashHookData(order.hook, order.hookPayload),
    //             block.number
    //         )
    //     );
    // }

    function _hashHookData(address hook, bytes memory hookPayload) internal pure returns (bytes32) {
        if (hook == address(0)) {
            return keccak256("");
        }
        return keccak256(abi.encodePacked(hook, hookPayload));
    }
}
