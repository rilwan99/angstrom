// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.26;

import {PoolConfigs, PoolConfigsLib} from "../libraries/pool-config/PoolConfigs.sol";
import {SafeCastLib} from "solady/src/utils/SafeCastLib.sol";

/// @author philogy <https://github.com/philogy>
abstract contract NodeManager {
    error NotController();
    error OnlyOncePerBlock();
    error NotNode();
    error AssetsUnsorted();

    address internal immutable _CONTROLLER;

    mapping(address => bool) internal _isNode;

    PoolConfigs configs;
    uint64 public lastBlockUpdated;
    address internal configStore;

    constructor(address controller) {
        _CONTROLLER = controller;
    }

    modifier onlyController() {
        if (msg.sender != _CONTROLLER) revert NotController();
        _;
    }

    /// @dev Allow controller to set parameters of a given pool.
    function configurePool(address asset0, address asset1, uint16 tickSpacing, uint24 feeInE6)
        external
        onlyController
    {
        if (asset1 >= asset0) revert AssetsUnsorted();
        bytes32 fullKey = PoolConfigsLib.getFullKeyUnchecked(asset0, asset1);
        configStore = configs.setConfig(configStore, fullKey, tickSpacing, feeInE6);
    }

    function govToggleNodes(address[] calldata nodes) external onlyController {
        for (uint256 i = 0; i < nodes.length; i++) {
            address node = nodes[i];
            _isNode[node] = !_isNode[node];
        }
    }

    /// @dev Validates that the caller is a node and that the last call is at least 1 block old.
    /// Blocks reentrant calls as well as separate calls in the same block.
    function _nodeBundleLock() internal {
        // Block-wide reentrancy lock. Prevents general reentrancy and replay of flash orders.
        if (lastBlockUpdated == block.number) revert OnlyOncePerBlock();
        if (!_isNode[msg.sender]) revert NotNode();
        lastBlockUpdated = SafeCastLib.toUint64(block.number);
    }
}
