// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {AssetIndex} from "../types/PriceGraph.sol";
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
    bool useInternal;
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

library OrderLib {
    using FormatLib for *;

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
