// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Asset, ASSET_ENCODED_BYTES} from "./Asset.sol";
import {CalldataReader} from "super-sol/collections/CalldataReader.sol";

import {safeconsole as console} from "forge-std/safeconsole.sol";
import {mem as memc} from "test/_helpers/mem.sol";

type OrderBuffer is uint256;

type MaybeHook is uint256;

using OrderBufferLib for OrderBuffer global;
using OrderBufferLib for MaybeHook global;

/// @author philogy <https://github.com/philogy>
library OrderBufferLib {
    error InvalidVariant(uint256 variant);
    error InvalidHookReturn();

    /// @dev Harcoded hashes derived from variant.
    /// @custom:test test/types/OrderIterator.t.sol:test_hardcodedTypehashes
    uint256 internal constant PARTIAL_STANDING_ORDER_TYPEHASH =
        0xa9e5294d444fbaeb5ca05f2b24c5d8294410f31413048e445d881e2ef69e6000;
    uint256 internal constant EXACT_STANDING_ORDER_TYPEHASH =
        0xef0cce88fda04ab3f84618823372cf76f64b4511ce8c79630eabc29ebc9b968f;
    uint256 internal constant PARTIAL_FLASH_ORDER_TYPEHASH =
        0xebd5091c396f79056e45455f4a93bbb016c217314bb2356b22d1c13833ac8861;
    uint256 internal constant EXACT_FLASH_ORDER_TYPEHASH =
        0x5b9d49cfed48f8c9d1a863f61c2479c579fa299c25a33211fc857390cecce6d4;

    /// @dev Magic bit offset that yields variant from typehash for every order type.
    /// @custom:test test/types/OrderIterator.t.sol:test_variantMagicOffset
    uint256 internal constant VARIANT_MAGIC_OFFSET = 39;

    uint256 internal constant PARTIAL_STANDING_VARIANT = 0;
    uint256 internal constant EXACT_STANDING_VARIANT = 1;
    uint256 internal constant PARTIAL_FLASH_VARIANT = 2;
    uint256 internal constant EXACT_FLASH_VARIANT = 3;

    uint256 internal constant VARIANT_MASK = 0x3;

    uint256 internal constant VARIANT_ECDSA_SIG = 0x40;
    uint256 internal constant VARIANT_HAS_RECIPIENT = 0x20;
    uint256 internal constant VARIANT_USE_INTERNAL_BIT = 0x10;
    uint256 internal constant VARIANT_NO_HOOK_BIT = 0x08;
    uint256 internal constant VARIANT_IS_OUT_BIT = 0x04;
    uint256 internal constant VARIANT_IS_FLASH_BIT = 0x02;
    uint256 internal constant VARIANT_IS_EXACT_BIT = 0x01;

    /// @dev Hash of empty sequence of bytes `keccak256("")`
    /// @custom:test test/types/OrderIterator.t.sol:test_emptyBytesHash
    uint256 internal constant EMPTY_BYTES_HASH = 0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470;

    // TODO: Make test that ensures that buffer space is always enough.
    /// @dev Size of memory area to reserve to store order contents and typehash for hashing.
    uint256 internal constant ORDER_BUFFER_BYTES = 352;

    uint256 internal constant HOOK_MEM_PTR_OFFSET = 32;
    uint256 internal constant HOOK_LENGTH_MASK = 0xffffffff;

    /// @dev Left-shifted hook selector (`compose(address, bytes)`).
    uint256 internal constant HOOK_SELECTOR_LEFT_ALIGNED =
        0x7407905c00000000000000000000000000000000000000000000000000000000;
    /// @dev Expected return magic (`keccak256("Angstrom.hook.return-magic")`)
    uint256 internal constant EXPECTED_HOOK_RETURN_MAGIC = 0x24a2e44b;

    function initBuffer() internal pure returns (OrderBuffer buf) {
        assembly ("memory-safe") {
            buf := mload(0x40)
            mstore(0x40, add(buf, ORDER_BUFFER_BYTES))
            // Clear buffer region incase free memory was used.
            calldatacopy(buf, calldatasize(), ORDER_BUFFER_BYTES)
        }
    }

    function clear(OrderBuffer self) internal pure {
        assembly ("memory-safe") {
            // Copies 0 bytes clearing memory region.
            calldatacopy(self, calldatasize(), ORDER_BUFFER_BYTES)
        }
    }

    function setTypeHash(OrderBuffer self, uint256 variant) internal pure {
        assembly ("memory-safe") {
            variant := and(variant, VARIANT_MASK)
            for {} 1 {} {
                if eq(variant, PARTIAL_STANDING_VARIANT) {
                    mstore(self, PARTIAL_STANDING_ORDER_TYPEHASH)
                    break
                }
                if eq(variant, EXACT_STANDING_VARIANT) {
                    mstore(self, EXACT_STANDING_ORDER_TYPEHASH)
                    break
                }
                if eq(variant, PARTIAL_FLASH_VARIANT) {
                    mstore(self, PARTIAL_FLASH_ORDER_TYPEHASH)
                    break
                }
                if eq(variant, EXACT_FLASH_VARIANT) {
                    mstore(self, EXACT_FLASH_ORDER_TYPEHASH)
                    break
                }

                mstore(0x00, 0x7feae7ca /* InvalidVariant(uint256) */ )
                mstore(0x20, variant)
                revert(0x1c, 0x24)
            }
        }
    }

    function log(OrderBuffer self, uint256 variant) internal pure {
        uint256 length;
        assembly {
            let flashLength := sub(ORDER_BUFFER_BYTES, 32)
            let extraWord := iszero(and(variant, VARIANT_IS_FLASH_BIT))
            length := add(flashLength, shl(5, extraWord))
        }
        memc.logMemory(OrderBuffer.unwrap(self), length);
    }

    function hash(OrderBuffer self, uint256 variant) internal pure returns (bytes32 hashed) {
        assembly ("memory-safe") {
            let flashLength := sub(ORDER_BUFFER_BYTES, 32)
            let extraWord := iszero(and(variant, VARIANT_IS_FLASH_BIT))
            let length := add(flashLength, shl(5, extraWord))
            hashed := keccak256(self, length)
        }
    }

    function readAndHashHook(OrderBuffer self, CalldataReader reader, uint256 variant)
        internal
        pure
        returns (CalldataReader, MaybeHook hook)
    {
        assembly ("memory-safe") {
            let hookDataHash := EMPTY_BYTES_HASH
            if iszero(and(variant, VARIANT_NO_HOOK_BIT)) {
                // Load hook address and payload length from reader.
                let packedAddrLength := calldataload(reader)
                let length := and(0xffffff, shr(72, packedAddrLength))
                reader := add(reader, 23)

                // Allocate hook call memory..
                hook := mload(0x40)
                let payloadOffset := add(hook, 0x64)
                mstore(0x40, add(payloadOffset, length))

                // Store target address into memory, will get swapped with signer address later.
                mstore(add(hook, 0x50), packedAddrLength)
                // Read payload bytes into memory.
                calldatacopy(payloadOffset, reader, length)
                reader := add(reader, length)
                hookDataHash := keccak256(sub(payloadOffset, 20), add(length, 20))

                mstore(hook, HOOK_SELECTOR_LEFT_ALIGNED)
                // Store hook address at offset 0x04 (swapped later for `from` address).
                mstore(add(hook, 16), packedAddrLength)
                // Calldata offset of payload, also overwrites length bytes of `packedAddrLength`.
                mstore(add(hook, 0x24), 0x40)
                mstore(add(hook, 0x44), length)

                // Set final value for hook object (packed: u224:memOffset ++ u32:callLength)
                hook := or(shl(HOOK_MEM_PTR_OFFSET, hook), add(length, 0x64))
            }
            mstore(add(self, 0x100), hookDataHash)
        }
        return (reader, hook);
    }

    function readOrderValidation(OrderBuffer self, CalldataReader reader, uint256 variant)
        internal
        view
        returns (CalldataReader)
    {
        if (variant & VARIANT_IS_FLASH_BIT != 0) {
            assembly ("memory-safe") {
                mstore(add(self, 0x120), number())
            }
            // Nothing read, reader stays unmodified.
        } else {
            assembly ("memory-safe") {
                calldatacopy(add(self, add(0x120, sub(0x20, 8))), reader, 8)
                reader := add(reader, 8)
                calldatacopy(add(self, add(0x140, sub(0x20, 5))), reader, 5)
                reader := add(reader, 5)
            }
        }
        return reader;
    }

    function setMinPrice(OrderBuffer self, uint256 minPrice) internal pure {
        assembly ("memory-safe") {
            mstore(add(self, 0x60), minPrice)
        }
    }

    function setUseInternal(OrderBuffer self, bool useInternal) internal pure {
        assembly ("memory-safe") {
            mstore(add(self, 0x80), useInternal)
        }
    }

    function setAssetIn(OrderBuffer self, address assetIn) internal pure {
        assembly ("memory-safe") {
            mstore(add(self, 0xa0), assetIn)
        }
    }

    function setAssetOut(OrderBuffer self, address assetOut) internal pure {
        assembly ("memory-safe") {
            mstore(add(self, 0xc0), assetOut)
        }
    }

    function setRecipient(OrderBuffer self, address recipient) internal pure {
        assembly ("memory-safe") {
            mstore(add(self, 0xe0), recipient)
        }
    }

    function setQuantityExact(OrderBuffer self, uint256 variant, uint256 quantity) internal pure {
        assembly ("memory-safe") {
            mstore(add(self, 0x20), iszero(and(variant, VARIANT_IS_OUT_BIT)))
            mstore(add(self, 0x40), quantity)
        }
    }

    function setQuantityPartialMinMax(OrderBuffer self, uint256 minQuantityIn, uint256 maxQuantityIn) internal pure {
        assembly ("memory-safe") {
            mstore(add(self, 0x20), minQuantityIn)
            mstore(add(self, 0x40), maxQuantityIn)
        }
    }

    function nonce(OrderBuffer self) internal pure returns (uint64 nonce_) {
        assembly ("memory-safe") {
            nonce_ := mload(add(self, 0x120))
        }
    }

    function deadline(OrderBuffer self) internal pure returns (uint256 deadline_) {
        assembly ("memory-safe") {
            deadline_ := mload(add(self, 0x140))
        }
    }

    function triggerIfSome(MaybeHook self, address from) internal {
        assembly ("memory-safe") {
            if self {
                // Unpack hook.
                let length := and(self, HOOK_LENGTH_MASK)
                let mem := shr(HOOK_MEM_PTR_OFFSET, self)
                // Swap hook address with `from` address.
                let addrOffset := add(mem, 0x04)
                let hookAddr := mload(addrOffset)
                mstore(addrOffset, from)
                // Call hook
                let success := call(gas(), hookAddr, 0, mem, length, 0x00, 0x20)

                if iszero(and(success, and(gt(returndatasize(), 31), eq(mload(0x00), EXPECTED_HOOK_RETURN_MAGIC)))) {
                    mstore(0x00, 0xf959fdae /* InvalidHookReturn() */ )
                    revert(0x1c, 0x04)
                }

                // What allocator? I am the allocator.
                // (checks if end of hook memory allocation is free so we can move down the free
                // pointer, effectively freeing the memory.)
                if eq(mload(0x40), add(mem, length)) { mstore(0x40, mem) }
            }
        }
    }
}
