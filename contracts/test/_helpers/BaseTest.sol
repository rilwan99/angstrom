// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import {Test} from "forge-std/Test.sol";
import {Trader} from "./types/Trader.sol";
import {console2 as console} from "forge-std/console2.sol";
import {HookDeployer} from "./HookDeployer.sol";

import {FormatLib} from "super-sol/libraries/FormatLib.sol";

/// @author philogy <https://github.com/philogy>
contract BaseTest is Test {
    using FormatLib for *;

    uint256 internal constant REAL_TIMESTAMP = 1721652639;

    function i24(uint256 x) internal pure returns (int24 y) {
        assertLe(x, uint24(type(int24).max), "Unsafe cast to int24");
        y = int24(int256(x));
    }

    function u128(uint256 x) internal pure returns (uint128 y) {
        assertLe(x, type(uint128).max, "Unsafe cast to uint128");
        y = uint128(x);
    }

    function u64(uint256 x) internal pure returns (uint64 y) {
        assertLe(x, type(uint64).max, "Unsafe cast to uint64");
        y = uint64(x);
    }

    function u40(uint256 x) internal pure returns (uint40 y) {
        assertLe(x, type(uint40).max, "Unsafe cast to uint40");
        y = uint40(x);
    }

    function makeTrader(string memory name) internal returns (Trader memory trader) {
        (trader.addr, trader.key) = makeAddrAndKey(name);
    }

    function makeTraders(uint256 n) internal returns (Trader[] memory traders) {
        traders = new Trader[](n);
        for (uint256 i = 0; i < n; i++) {
            traders[i] = makeTrader(string.concat("trader_", (i + 1).toStr()));
        }
    }
}
