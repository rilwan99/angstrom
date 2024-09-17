// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Angstrom} from "../../src/Angstrom.sol";
import {PoolId} from "v4-core/src/types/PoolId.sol";
import {IUniV4, IPoolManager} from "../../src/interfaces/IUniV4.sol";
import {SafeCastLib} from "solady/src/utils/SafeCastLib.sol";

/// @author philogy <https://github.com/philogy>
contract ExtAngstrom is Angstrom {
    using IUniV4 for IPoolManager;

    constructor(address uniV4PoolManager, address governance) Angstrom(uniV4PoolManager, governance) {}

    function __ilegalMint(address to, address asset, uint256 amount) external {
        _angstromReserves[to][asset] += amount;
    }

    function updateLastBlock() public {
        lastBlockUpdated = SafeCastLib.toUint64(block.number);
    }

    function isNode(address addr) public view returns (bool) {
        return _isNode[addr];
    }

    function DOMAIN_SEPARATOR() external view returns (bytes32) {
        return _domainSeparator();
    }

    function hashTyped(bytes32 structHash) external view returns (bytes32) {
        return _hashTypedData(structHash);
    }

    function globalGrowthInside(PoolId id, int24 lowerTick, int24 upperTick) external view returns (uint256) {
        int24 currentTick = UNI_V4.getSlot0(id).tick();
        return poolRewards[id].getGrowthInside(currentTick, lowerTick, upperTick);
    }
}
