// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {NodeManager} from "./NodeManager.sol";
import {UniConsumer} from "./UniConsumer.sol";
import {Hooks} from "v4-core/src/libraries/Hooks.sol";
import {PoolKey} from "v4-core/src/types/PoolKey.sol";
import {Currency} from "v4-core/src/types/Currency.sol";

import {console} from "forge-std/console.sol";
import {POOL_FEE} from "../Constants.sol";
import {PoolConfigsLib} from "../libraries/pool-config/PoolConfigs.sol";
import {IBeforeInitializeHook} from "../interfaces/IHooks.sol";

/// @author philogy <https://github.com/philogy>
abstract contract HookManager is UniConsumer, NodeManager, IBeforeInitializeHook {
    error InvalidPoolKey();

    constructor() {
        _checkHookPermissions(Hooks.BEFORE_SWAP_FLAG | Hooks.BEFORE_INITIALIZE_FLAG);
    }

    function beforeInitialize(address, PoolKey calldata poolKey, uint160, bytes calldata)
        external
        view
        onlyUniV4
        returns (bytes4)
    {
        bytes32 fullKey = PoolConfigsLib.getFullKeyUnchecked(
            Currency.unwrap(poolKey.currency0), Currency.unwrap(poolKey.currency1)
        );
        int24 tickSpacing = configs.get(fullKey).tickSpacing;
        if (poolKey.tickSpacing != tickSpacing || poolKey.fee != POOL_FEE) revert InvalidPoolKey();
        return this.beforeInitialize.selector;
    }
}
