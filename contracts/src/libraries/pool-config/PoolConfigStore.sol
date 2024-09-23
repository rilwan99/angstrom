// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {ConfigEntry, ENTRY_SIZE, KEY_MASK} from "./ConfigEntry.sol";
import {PartialKey} from "./PartialKey.sol";

type PoolConfigStore is address;

uint256 constant MEMORY_OFFSET_OFFSET = 192;
uint256 constant STORE_ADDR_OFFSET = 32;
uint256 constant SIZE_OFFSET = 0;
uint256 constant SIZE_MASK = 0xffffffff;
uint256 constant STORE_HEADER_SIZE = 1;

using PoolConfigStoreLib for PoolConfigStore global;

/// @author philogy <https://github.com/philogy>
library PoolConfigStoreLib {
    PoolConfigStore internal constant NULL_CONFIG_CACHE = PoolConfigStore.wrap(address(0));

    function isNull(PoolConfigStore self) internal pure returns (bool) {
        return PoolConfigStore.unwrap(self) == PoolConfigStore.unwrap(NULL_CONFIG_CACHE);
    }

    function getOrDefaultEmpty(PoolConfigStore self, PartialKey pkey, uint256 index)
        internal
        view
        returns (ConfigEntry entry)
    {
        assembly {
            // Copy from store into scratch space.
            extcodecopy(self, 0x00, add(STORE_HEADER_SIZE, mul(ENTRY_SIZE, index)), ENTRY_SIZE)
            // Zero out entry if keys do not match.
            entry := mul(mload(0x00), eq(pkey, and(entry, KEY_MASK)))
        }
    }
}
