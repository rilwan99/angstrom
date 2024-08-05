// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {OrdersLib} from "../reference/OrderTypes.sol";

struct ToBOrderBuffer {
    bytes32 typeHash;
    uint256 quantityIn;
    uint256 quantityOut;
    bool useInternal;
    address assetIn;
    address assetOut;
    address recipient;
    bytes32 hookDataHash;
    uint64 validForBlock;
}

using ToBOrderBufferLib for ToBOrderBuffer global;

/// @author philogy <https://github.com/philogy>
library ToBOrderBufferLib {
    // TODO: TEST
    uint256 internal constant BUFFER_BYTES = 288;

    function setTypeHash(ToBOrderBuffer memory self) internal pure {
        self.typeHash = OrdersLib.TOP_OF_BLOCK_ORDER_TYPEHASH;
    }

    function hash(ToBOrderBuffer memory self) internal view returns (bytes32 orderHash) {
        self.validForBlock = uint64(block.number);
        assembly ("memory-safe") {
            orderHash := keccak256(self, BUFFER_BYTES)
        }
    }

    /// @dev WARNING: Using the associated `ToBOrderBuffer` buffer after an attempted free is
    /// **unsafe**.
    function tryFree(ToBOrderBuffer memory self) internal pure {
        assembly ("memory-safe") {
            if eq(mload(0x40), add(self, BUFFER_BYTES)) { mstore(0x40, self) }
        }
    }
}
