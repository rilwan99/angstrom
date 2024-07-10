// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {IPoolManager} from "../interfaces/IUniV4.sol";

/// @author philogy <https://github.com/philogy>
abstract contract UniConsumer {
    error NotUniswap();

    IPoolManager internal immutable UNI_V4;

    modifier onlyUniV4() {
        if (msg.sender != address(UNI_V4)) revert NotUniswap();
        _;
    }

    constructor(address uniV4PoolManager) {
        UNI_V4 = IPoolManager(uniV4PoolManager);
    }
}
