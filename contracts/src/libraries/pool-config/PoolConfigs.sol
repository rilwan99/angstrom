// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import {POOL_CONFIG_STORAGE_PREFIX} from "src/Constants.sol";
import {
    ConfigEntry,
    ConfigEntryLib,
    ENTRY_SIZE,
    KEY_MASK,
    TICK_SPACING_OFFSET,
    TICK_SPACING_MASK,
    FEE_OFFSET,
    FEE_MASK
} from "./ConfigEntry.sol";
import {PartialKey, PartialKeyLib} from "./PartialKey.sol";
import {
    PoolConfigStore,
    PoolConfigStoreLib,
    MEMORY_OFFSET_OFFSET,
    STORE_HEADER_SIZE
} from "./PoolConfigStore.sol";

struct PoolConfig {
    int24 tickSpacing;
    uint24 feeInE6;
}

struct PoolConfigs {
    uint256 __placeholder;
}

using PoolConfigsLib for PoolConfigs global;

/// @author philogy <https://github.com/philogy>
library PoolConfigsLib {
    error FailedToDeployNewStore();
    error InvalidKey();
    error InvalidTickSpacing();
    error FeeAboveMax();

    uint24 internal constant MAX_FEE = 0.2e6;

    /*
     * @dev Generated from ./StoreDeployer.huff using https://github.com/Philogy/py-huff (commit: 44bbb4b)
     *  PC   OP BYTES   INSTRUCTION   STACK (Top -> Down)          COMMENT
     * ----------------------------------------------------------------------------------------------
     * Constructor Code
     *   0   600b       PUSH1 11      [11]                         Push constructor size
     *   2   38         CODESIZE      [codesize, 11]               Sum of all bytes including runtime
     *   3   03         SUB           [run_size]                   Subtracting to compute the runtime size
     *   4   80         DUP1          [run_size, run_size]         Duplicate for later
     *   5   600b       PUSH1 11      [11, run_size, run_size]     Push constructor size again
     *   7   5f         PUSH0         [0, 11, run_size, run_size]
     *   8   39         CODECOPY      [run_size]                   Copy the runtime code into memory
     *                                                             (`memory[0:run_size] = code[11:run_size]`)
     *   9   5f         PUSH0         [0, run_size]
     *  10   f3         RETURN        []                           Return runtime from memory as final code
     *                                                             (`runtime = memory[0:runsize]`)
     * Runtime Code
     *   0   00         STOP          []                           Stop execution. Ensure that even if
     *                                                             called the store contract cannot do
     *                                                             anything like SELFDESTRUCTing itself.
     **/
    uint256 internal constant STORE_DEPLOYER = 0x600b380380600b5f395ff300;
    uint256 internal constant STORE_DEPLOYER_BYTES = 12;

    function getFullKeyUnsorted(address assetA, address assetB)
        internal
        pure
        returns (bytes32 fullKey)
    {
        assembly ("memory-safe") {
            let aOffset := shl(5, gt(assetA, assetB))
            mstore(aOffset, assetA)
            mstore(xor(aOffset, 0x20), assetB)
            fullKey := keccak256(0x00, 0x40)
        }
    }

    function getFullKeyUnchecked(address asset0, address asset1)
        internal
        pure
        returns (bytes32 fullKey)
    {
        assembly ("memory-safe") {
            mstore(0x00, asset0)
            mstore(0x20, asset1)
            fullKey := keccak256(0x00, 0x40)
        }
    }

    function setConfig(
        PoolConfigs storage self,
        address previousStore,
        bytes32 fullKey,
        uint16 tickSpacing,
        uint24 feeInE6
    ) internal returns (address newStore) {
        if (tickSpacing == 0) revert InvalidTickSpacing();
        if (feeInE6 > MAX_FEE) revert FeeAboveMax();
        // Update direct mapping.
        PoolConfig storage config = self.get(fullKey);
        config.tickSpacing = int24(uint24(tickSpacing));
        config.feeInE6 = feeInE6;
        // Update store.
        uint256 totalEntries = previousStore.code.length / ENTRY_SIZE;
        PartialKey pkey = PartialKeyLib.toPartialKey(fullKey);

        assembly ("memory-safe") {
            // Get free memory & copy in the entire store's contents.
            let free := mload(0x40)
            let entryOffset := add(free, 0x20)
            let totalEntryBytes := mul(ENTRY_SIZE, totalEntries)
            extcodecopy(previousStore, entryOffset, STORE_HEADER_SIZE, totalEntryBytes)
            // Construct new entry by splicing in the values.
            let newEntry :=
                or(
                    pkey,
                    or(
                        shl(TICK_SPACING_OFFSET, and(tickSpacing, TICK_SPACING_MASK)),
                        shl(FEE_OFFSET, and(feeInE6, FEE_MASK))
                    )
                )
            let entriesEnd := add(entryOffset, totalEntryBytes)
            mstore(entriesEnd, newEntry)
            for {} lt(entryOffset, entriesEnd) { entryOffset := add(entryOffset, 0x20) } {
                let entry := mload(entryOffset)
                if eq(pkey, and(entry, KEY_MASK)) {
                    mstore(entryOffset, newEntry)
                    break
                }
            }
            // Increase `totalEntryBytes` by 0x20 if we broke in the loop.
            totalEntryBytes := add(totalEntryBytes, shl(5, lt(entryOffset, entriesEnd)))
            // Add deployment code to the front and deploy
            mstore(free, STORE_DEPLOYER)
            newStore :=
                create(
                    0,
                    add(free, sub(32, STORE_DEPLOYER_BYTES)),
                    add(totalEntryBytes, STORE_DEPLOYER_BYTES)
                )
        }

        if (newStore == address(0)) revert FailedToDeployNewStore();
    }

    function getWithStore(
        PoolConfigs storage configs,
        PoolConfigStore store,
        bytes32 fullKey,
        uint256 relativeIndex
    ) internal view returns (int24 tickSpacing, uint24 feeInE6) {
        if (store.isNull()) {
            PoolConfig storage config = configs.get(fullKey);
            tickSpacing = config.tickSpacing;
            // If config was never set `feeInE6` will be the default 0.
            feeInE6 = config.feeInE6;
        } else {
            // `entry` will be empty if the index was out of bounds or the key did not match the fetched entry.
            ConfigEntry entry =
                store.getOrDefaultEmpty(PartialKeyLib.toPartialKey(fullKey), relativeIndex);
            tickSpacing = entry.tickSpacing();
            // Will default to 0 if the entry is empty.
            feeInE6 = entry.feeInE6();
        }
        if (tickSpacing <= 0) revert InvalidTickSpacing();
    }

    function get(PoolConfigs storage configs, bytes32 fullKey)
        internal
        pure
        returns (PoolConfig storage config)
    {
        assembly ("memory-safe") {
            mstore(0x00, fullKey)
            mstore(0x20, configs.slot)
            config.slot := or(shr(8, keccak256(0x00, 0x40)), shl(248, POOL_CONFIG_STORAGE_PREFIX))
        }
    }
}
