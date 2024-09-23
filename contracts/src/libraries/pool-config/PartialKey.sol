// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

uint256 constant PARTIAL_KEY_OFFSET = 40;

type PartialKey is uint256;

function keyEq(PartialKey pkey1, PartialKey pkey2) pure returns (bool) {
    return PartialKey.unwrap(pkey1) == PartialKey.unwrap(pkey2);
}

using {keyEq as ==} for PartialKey global;

using PartialKeyLib for PartialKey global;

library PartialKeyLib {
    function toPartialKey(bytes32 fullKey) internal pure returns (PartialKey partialKey) {
        return PartialKey.wrap(uint256(fullKey) << PARTIAL_KEY_OFFSET);
    }
}
