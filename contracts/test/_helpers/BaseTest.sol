// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {console2 as console} from "forge-std/console2.sol";

/// @author philogy <https://github.com/philogy>
contract BaseTest is Test {
    function sign(Account memory account, bytes32 hash) internal pure returns (bytes memory) {
        console.log("%s signing %x", account.addr, uint256(hash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(account.key, hash);
        // Compress.
        uint256 vs = (uint256(v - 27) << 255) | uint256(s);
        return abi.encodePacked(r, vs);
    }

    function i24(uint256 x) internal pure returns (int24 y) {
        assertLe(x, uint24(type(int24).max), "Unsafe cast to int24");
        y = int24(int256(x));
    }

    function u128(uint256 x) internal pure returns (uint128 y) {
        assertLe(x, type(uint128).max, "Unsafe cast to uint128");
        y = uint128(x);
    }
}
