// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

struct Asset {
    address addr;
    uint256 borrow;
    uint256 save;
    uint256 settle;
}
