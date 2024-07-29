// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

struct Asset {
    address addr;
    uint256 borrow;
    uint256 save;
    uint256 settle;
}

// TODO: Add test for magic offset
uint256 constant ASSET_ENCODED_BYTES = 128;
