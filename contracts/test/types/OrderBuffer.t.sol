// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {Test} from "forge-std/Test.sol";
import {OrdersLib} from "../../src/reference/OrderTypes.sol";
import {OrderBufferLib} from "../../src/types/OrderBuffer.sol";

import {console2 as console} from "forge-std/console2.sol";

/// @author philogy <https://github.com/philogy>
contract OrderBufferTest is Test {
    function setUp() public {}

    struct OrderType {
        uint256 typeHash;
        uint256 variant;
    }

    function test_hardcodedTypehashes() public pure {
        assertEq(OrderBufferLib.PARTIAL_STANDING_ORDER_TYPEHASH, uint256(OrdersLib.PARTIAL_STANDING_ORDER_TYPEHASH));
        assertEq(OrderBufferLib.EXACT_STANDING_ORDER_TYPEHASH, uint256(OrdersLib.EXACT_STANDING_ORDER_TYPEHASH));
        assertEq(OrderBufferLib.PARTIAL_FLASH_ORDER_TYPEHASH, uint256(OrdersLib.PARTIAL_FLASH_ORDER_TYPEHASH));
        assertEq(OrderBufferLib.EXACT_FLASH_ORDER_TYPEHASH, uint256(OrdersLib.EXACT_FLASH_ORDER_TYPEHASH));
    }

    function test_variantMagicOffset() public pure {
        OrderType[4] memory otypes = _orderTypes();
        uint256 realOffset = 0;
        for (; realOffset < 255; realOffset++) {
            bool wrong = false;
            for (uint256 i = 0; i < otypes.length; i++) {
                OrderType memory otype = otypes[i];
                wrong = wrong || (otype.typeHash >> realOffset) & 0x3 != otype.variant;
            }
            if (!wrong) break;
        }
        assertLt(realOffset, 255, "No possible offset");
        assertEq(OrderBufferLib.VARIANT_MAGIC_OFFSET, realOffset, "Lib offset != actual");
    }

    function test_noDuplicateVariant() public pure {
        OrderType[4] memory otypes = _orderTypes();
        for (uint256 i = 0; i < otypes.length; i++) {
            for (uint256 j = 0; j < otypes.length; j++) {
                if (i != j) assertTrue(otypes[i].variant != otypes[j].variant);
            }
        }
    }

    function test_emptyBytesHash() public pure {
        assertEq(bytes32(OrderBufferLib.EMPTY_BYTES_HASH), keccak256(""));
    }

    function _orderTypes() internal pure returns (OrderType[4] memory) {
        return [
            OrderType(OrderBufferLib.PARTIAL_STANDING_ORDER_TYPEHASH, OrderBufferLib.PARTIAL_STANDING_VARIANT),
            OrderType(OrderBufferLib.EXACT_STANDING_ORDER_TYPEHASH, OrderBufferLib.EXACT_STANDING_VARIANT),
            OrderType(OrderBufferLib.PARTIAL_FLASH_ORDER_TYPEHASH, OrderBufferLib.PARTIAL_FLASH_VARIANT),
            OrderType(OrderBufferLib.EXACT_FLASH_ORDER_TYPEHASH, OrderBufferLib.EXACT_FLASH_VARIANT)
        ];
    }
}
