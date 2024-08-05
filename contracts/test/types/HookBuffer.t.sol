// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Test} from "forge-std/Test.sol";
import {HookBuffer, HookBufferLib} from "src/types/HookBuffer.sol";

/// @author philogy <https://github.com/philogy>
contract HookBufferTest is Test {
    function test_emptyBytesHash() public pure {
        assertEq(bytes32(HookBufferLib.EMPTY_BYTES_HASH), keccak256(""));
    }
}
