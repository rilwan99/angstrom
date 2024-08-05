// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

type OrderVariant is uint8;

using OrderVariantLib for OrderVariant global;

/// @author philogy <https://github.com/philogy>
library OrderVariantLib {
    uint256 internal constant VARIANT_IS_EXACT_BIT = 0x01;
    uint256 internal constant VARIANT_IS_FLASH_BIT = 0x02;
    uint256 internal constant VARIANT_IS_OUT_BIT = 0x04;
    uint256 internal constant VARIANT_NO_HOOK_BIT = 0x08;
    uint256 internal constant VARIANT_USE_INTERNAL_BIT = 0x10;
    uint256 internal constant VARIANT_HAS_RECIPIENT = 0x20;
    uint256 internal constant VARIANT_IS_ECDSA_BIT = 0x40;
    uint256 internal constant VARIANT_A_TO_B_BIT = 0x80;

    function isExact(OrderVariant variant) internal pure returns (bool) {
        return OrderVariant.unwrap(variant) & VARIANT_IS_EXACT_BIT != 0;
    }

    function isFlash(OrderVariant variant) internal pure returns (bool) {
        return OrderVariant.unwrap(variant) & VARIANT_IS_FLASH_BIT != 0;
    }

    function isExactOut(OrderVariant variant) internal pure returns (bool) {
        return OrderVariant.unwrap(variant) & VARIANT_IS_OUT_BIT != 0;
    }

    function noHook(OrderVariant variant) internal pure returns (bool) {
        return OrderVariant.unwrap(variant) & VARIANT_NO_HOOK_BIT != 0;
    }

    function useInternal(OrderVariant variant) internal pure returns (bool) {
        return OrderVariant.unwrap(variant) & VARIANT_USE_INTERNAL_BIT != 0;
    }

    function hasRecipient(OrderVariant variant) internal pure returns (bool) {
        return OrderVariant.unwrap(variant) & VARIANT_HAS_RECIPIENT != 0;
    }

    function isEcdsa(OrderVariant variant) internal pure returns (bool) {
        return OrderVariant.unwrap(variant) & VARIANT_IS_ECDSA_BIT != 0;
    }

    function aToB(OrderVariant variant) internal pure returns (bool) {
        return OrderVariant.unwrap(variant) & VARIANT_A_TO_B_BIT != 0;
    }
}
