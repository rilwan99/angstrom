// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @dev Abstraction for memory pointer to 66 bytes of memory used for ERC712 typed data hashing.
type TypedDataHasher is uint256;

using TypedDataHasherLib for TypedDataHasher global;

/// @author philogy <https://github.com/philogy>
library TypedDataHasherLib {
    function init(bytes32 separator) internal pure returns (TypedDataHasher hasher) {
        assembly ("memory-safe") {
            // Get free memory.
            hasher := mload(0x40)
            // Pre-store ERC721 header bytes and domain separator in memory.
            mstore(hasher, hex"1901")
            mstore(add(hasher, 2), separator)
            // Allocate 66 (0x42) bytes of memory (last 32 may be uncleared but we overwrite it in
            // `hash` anyway).
            mstore(0x40, add(hasher, 0x42))
        }
    }

    function hash(TypedDataHasher hasher, bytes32 structHash) internal pure returns (bytes32 digest) {
        assembly ("memory-safe") {
            mstore(add(hasher, 0x22), structHash)
            digest := keccak256(hasher, 0x42)
        }
    }
}
