// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {PoolConfigEntry, ENTRY_SIZE, KEY_MASK} from "./PoolConfigEntry.sol";
import {PartialKey} from "./PartialKey.sol";

/// @dev Packed `memoryOffset:u64 ++ storeAddress:u160 ++ cachedEntries:u32`
type PoolConfigStoreCache is uint256;

uint256 constant MEMORY_OFFSET_OFFSET = 192;
uint256 constant STORE_ADDR_OFFSET = 32;
uint256 constant SIZE_OFFSET = 0;
uint256 constant SIZE_MASK = 0xffffffff;
uint256 constant STORE_HEADER_SIZE = 1;

using PoolConfigStoreCacheLib for PoolConfigStoreCache global;

/// @author philogy <https://github.com/philogy>
library PoolConfigStoreCacheLib {
    PoolConfigStoreCache internal constant NULL_CONFIG_CACHE = PoolConfigStoreCache.wrap(0);

    function isNull(PoolConfigStoreCache self) internal pure returns (bool) {
        return PoolConfigStoreCache.unwrap(self) == PoolConfigStoreCache.unwrap(NULL_CONFIG_CACHE);
    }

    function getOrDefaultEmpty(PoolConfigStoreCache self, PartialKey pkey, uint256 relativeIndex)
        internal
        view
        returns (PoolConfigEntry entry)
    {
        assembly {
            let size := and(shr(SIZE_OFFSET, self), SIZE_MASK)
            let memoryOffset := shr(MEMORY_OFFSET_OFFSET, self)
            entry := mload(add(memoryOffset, mul(ENTRY_SIZE, relativeIndex)))
            if iszero(lt(relativeIndex, size)) {
                // If index is out of bounds it either means a nonexistent entry is being referenced
                // or it's just not cached.
                entry := 0
                // If the cache is not a null cache it means the store is already warm so our
                // cheapest path is to do an individual read.
                if self {
                    let store := shr(STORE_ADDR_OFFSET, self)
                    // EXTCODECOPY ignores dirty upper bytes so don't need to clear
                    let storeIndex := sub(relativeIndex, size)
                    extcodecopy(store, 0x00, add(STORE_HEADER_SIZE, mul(ENTRY_SIZE, storeIndex)), ENTRY_SIZE)
                    entry := mload(0x00)
                }
            }
            // Zero out entry if keys do not match.
            entry := mul(entry, eq(pkey, and(entry, KEY_MASK)))
        }
    }

    function cacheStore(address store, uint256 entryOffset, uint256 total)
        internal
        view
        returns (PoolConfigStoreCache cache)
    {
        if (total == 0) return NULL_CONFIG_CACHE;
        assembly ("memory-safe") {
            // Allocate memory for store read.
            let memOffset := mload(0x40)
            let bytesToRead := mul(ENTRY_SIZE, total)
            mstore(0x40, add(memOffset, bytesToRead))
            //
            extcodecopy(store, memOffset, add(STORE_HEADER_SIZE, mul(ENTRY_SIZE, entryOffset)), bytesToRead)
            // Build packed pointer, cleaning `store` address incase upper bytes are dirty.
            cache :=
                or(
                    shr(sub(96, STORE_ADDR_OFFSET), shl(96, store)),
                    or(shl(MEMORY_OFFSET_OFFSET, memOffset), shl(SIZE_OFFSET, total))
                )
        }
    }
}
