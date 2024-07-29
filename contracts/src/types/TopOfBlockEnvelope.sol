// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {AssetIndex} from "./PriceGraph.sol";
import {FormatLib} from "super-sol/libraries/FormatLib.sol";

struct TopOfBlockOrderEnvelope {
    uint256 amountIn;
    uint256 amountOut;
    bool useInternal;
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

    /// forgefmt: disable-next-item
    bytes32 internal constant TOP_OF_BLOCK_ORDER_TYPEHASH = keccak256(
        "TopOfBlockOrder("
           "uint256 amount_in,"
           "uint256 amount_out,"
           "bool use_internal,"
           "address asset_in,"
           "address asset_out,"
           "address recipient,"
           "bytes hookData,"
           "uint256 valid_for_block"
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
                order.useInternal,
                assetIn,
                assetOut,
                order.recipient,
                _hashHookData(order.hook, order.hookPayload),
                block.number
            )
        );
    }

    function _hashHookData(address hook, bytes memory hookPayload) internal pure returns (bytes32) {
        if (hook == address(0)) {
            return keccak256("");
        }
        return keccak256(abi.encodePacked(hook, hookPayload));
    }
}
