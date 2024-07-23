// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {AssetIndex} from "./PriceGraph.sol";
import {FormatLib} from "super-sol/libraries/FormatLib.sol";

enum OrderMode {
    ExactIn,
    ExactOut,
    Partial
}

using OrderLib for OrderMode global;

enum OrderType {
    Flash,
    Standing
}

using OrderLib for OrderType global;

struct GenericOrder {
    OrderType otype;
    OrderMode mode;
    uint256 minAmountIn;
    uint256 amountSpecified;
    uint256 minPrice;
    AssetIndex assetInIndex;
    AssetIndex assetOutIndex;
    uint64 nonce;
    uint40 deadline;
    address recipient;
    address hook;
    bytes hookPayload;
    uint256 amountFilled;
    address from;
    bytes signature;
}

using OrderLib for GenericOrder global;

struct TopOfBlockOrderEnvelope {
    uint256 amountIn;
    uint256 amountOut;
    AssetIndex assetInIndex;
    AssetIndex assetOutIndex;
    address recipient;
    address hook;
    bytes hookPayload;
    address from;
    bytes signature;
}

using OrderLib for TopOfBlockOrderEnvelope global;

library OrderLib {
    using FormatLib for *;

    error CannotFlashloanZero();

    /// forgefmt: disable-next-item
    bytes32 internal constant TOP_OF_BLOCK_ORDER_TYPEHASH = keccak256(
        "TopOfBlockOrder("
           "uint256 amountIn,"
           "uint256 amountOut,"
           "address assetIn,"
           "uint8 assetInForm,"
           "address assetOut,"
           "uint8 assetOutForm,"
           "address recipient,"
           "bytes hookData,"
           "uint256 validForBlock"
        ")"
    );

    function hash(TopOfBlockOrderEnvelope memory order, address assetIn, address assetOut)
        internal
        view
        returns (bytes32)
    {
        return keccak256(
            abi.encode(
                TOP_OF_BLOCK_ORDER_TYPEHASH,
                order.amountIn,
                order.amountOut,
                assetIn,
                assetOut,
                order.recipient,
                _hashHookData(order.hook, order.hookPayload),
                block.number
            )
        );
    }

    // function toStr(GenericOrder memory order) internal pure returns (string memory) {
    // }

    function toStr(OrderMode mode) internal pure returns (string memory) {
        if (mode == OrderMode.ExactIn) return "OrderMode::ExactIn";
        else if (mode == OrderMode.ExactOut) return "OrderMode::ExactOut";
        else if (mode == OrderMode.Partial) return "OrderMode::Partial";
        else revert("Unknown order mode variant");
    }

    function toStr(OrderType otype) internal pure returns (string memory) {
        if (otype == OrderType.Flash) return "OrderType::Flash";
        else if (otype == OrderType.Standing) return "OrderType::Standing";
        else revert("Unknown order mode variant");
    }

    function _hashHookData(address hook, bytes memory hookPayload) internal pure returns (bytes32) {
        if (hook == address(0)) {
            return keccak256("");
        }
        return keccak256(abi.encodePacked(hook, hookPayload));
    }
}
