// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {PoolConfigsLib} from "src/libraries/pool-config/PoolConfigs.sol";
import {PoolConfigStore} from "src/libraries/pool-config/PoolConfigStore.sol";
import {ConfigEntry} from "src/libraries/pool-config/ConfigEntry.sol";
import {PartialKey, PartialKeyLib} from "src/libraries/pool-config/PartialKey.sol";
import {ENTRY_SIZE} from "src/libraries/pool-config/ConfigEntry.sol";

/// @author philogy <https://github.com/philogy>
library PoolStoreLib {
    function getStoreIndex(address store, address assetA, address assetB)
        internal
        view
        returns (uint16 index)
    {
        PartialKey pkey =
            PartialKeyLib.toPartialKey(PoolConfigsLib.getFullKeyUnsorted(assetA, assetB));
        uint256 totalEntries = store.code.length / ENTRY_SIZE;
        PoolConfigStore configStore = PoolConfigStore.wrap(store);
        for (index = 0; index < totalEntries; index++) {
            ConfigEntry entry = configStore.getOrDefaultEmpty(pkey, index);
            if (!entry.isEmpty()) return index;
        }
        revert("Pool not enabled");
    }
}
