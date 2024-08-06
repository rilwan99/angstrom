// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {OrdersLib} from "../reference/OrderTypes.sol";
import {CalldataReader} from "./CalldataReader.sol";
import {OrderVariant} from "./OrderVariant.sol";

import {safeconsole as console} from "forge-std/safeconsole.sol";

struct UserOrderBuffer {
    bytes32 typeHash;
    uint256 exactIn_or_minQuantityIn;
    uint256 quantity_or_maxQuantityIn;
    uint256 minPrice;
    bool useInternal;
    address assetIn;
    address assetOut;
    address recipient;
    bytes32 hookDataHash;
    uint64 nonce_or_validForBlock;
    uint40 deadline_or_empty;
}

using UserOrderBufferLib for UserOrderBuffer global;

/// @author philogy <https://github.com/philogy>
library UserOrderBufferLib {
    // TODO: Make test that ensures that buffer space is always enough.
    uint256 internal constant STANDING_ORDER_BYTES = 352;
    uint256 internal constant FLASH_ORDER_BYTES = 320;

    function setTypeHash(UserOrderBuffer memory self, OrderVariant variant) internal pure {
        if (variant.isExact()) {
            if (variant.isFlash()) {
                self.typeHash = OrdersLib.EXACT_FLASH_ORDER_TYPEHASH;
            } else {
                // is standing order.
                self.typeHash = OrdersLib.EXACT_STANDING_ORDER_TYPEHASH;
            }
        } else {
            // is partial order.
            if (variant.isFlash()) {
                self.typeHash = OrdersLib.PARTIAL_FLASH_ORDER_TYPEHASH;
            } else {
                // is standing order.
                self.typeHash = OrdersLib.PARTIAL_STANDING_ORDER_TYPEHASH;
            }
        }
    }

    function hash(UserOrderBuffer memory self, OrderVariant variant) internal pure returns (bytes32 hashed) {
        uint256 structLength = variant.isFlash() ? FLASH_ORDER_BYTES : STANDING_ORDER_BYTES;
        assembly ("memory-safe") {
            hashed := keccak256(self, structLength)
        }
    }

    function logBytes(UserOrderBuffer memory self, OrderVariant variant) internal pure {
        uint256 structLength = variant.isFlash() ? FLASH_ORDER_BYTES : STANDING_ORDER_BYTES;
        uint256 offset;
        assembly ("memory-safe") {
            offset := self
        }
        console.log("structLength: %s", structLength);
        console.logMemory(offset, structLength);
    }

    function setQuantityExact(UserOrderBuffer memory self, OrderVariant variant, uint256 quantity) internal pure {
        self.exactIn_or_minQuantityIn = variant.isExactOut() ? 0 : 1;
        self.quantity_or_maxQuantityIn = quantity;
    }

    function readOrderValidation(UserOrderBuffer memory self, CalldataReader reader, OrderVariant variant)
        internal
        view
        returns (CalldataReader)
    {
        if (variant.isFlash()) {
            self.nonce_or_validForBlock = uint64(block.number);
            // Nothing loaded from calldata, reader stays unmodified.
        } else {
            // Copy slice directly from calldata into memory.
            assembly ("memory-safe") {
                calldatacopy(add(self, add(0x120, sub(0x20, 8))), reader, 8)
                reader := add(reader, 8)
                calldatacopy(add(self, add(0x140, sub(0x20, 5))), reader, 5)
                reader := add(reader, 5)
            }
        }
        return reader;
    }
}
