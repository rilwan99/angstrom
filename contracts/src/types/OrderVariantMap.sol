// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

type OrderVariantMap is uint8;

using OrderVariantMapLib for OrderVariantMap global;

/// @author philogy <https://github.com/philogy>
library OrderVariantMapLib {
    uint256 internal constant USE_INTERNAL_BIT = 0x01;
    uint256 internal constant HAS_RECIPIENT_BIT = 0x02;
    uint256 internal constant HAS_HOOK_BIT = 0x04;
    uint256 internal constant A_TO_B_BIT = 0x08;
    uint256 internal constant IS_STANDING_BIT = 0x10;
    uint256 internal constant QTY_PARTIAL_BIT = 0x20;
    uint256 internal constant IS_EXACT_IN_BIT = 0x40;
    uint256 internal constant IS_ECDSA_BIT = 0x80;

    function useInternal(OrderVariantMap variant) internal pure returns (bool) {
        return OrderVariantMap.unwrap(variant) & USE_INTERNAL_BIT != 0;
    }

    function recipienIsSome(OrderVariantMap variant) internal pure returns (bool) {
        return OrderVariantMap.unwrap(variant) & HAS_RECIPIENT_BIT != 0;
    }

    function hookDataIsSome(OrderVariantMap variant) internal pure returns (bool) {
        return OrderVariantMap.unwrap(variant) & HAS_HOOK_BIT != 0;
    }

    function aToB(OrderVariantMap variant) internal pure returns (bool) {
        return OrderVariantMap.unwrap(variant) & A_TO_B_BIT != 0;
    }

    function standingValidationIsSome(OrderVariantMap variant) internal pure returns (bool) {
        return OrderVariantMap.unwrap(variant) & IS_STANDING_BIT != 0;
    }

    function quantitiesPartial(OrderVariantMap variant) internal pure returns (bool) {
        return OrderVariantMap.unwrap(variant) & QTY_PARTIAL_BIT != 0;
    }

    function exactIn(OrderVariantMap variant) internal pure returns (bool) {
        return OrderVariantMap.unwrap(variant) & IS_EXACT_IN_BIT != 0;
    }

    function sigIsEcdsa(OrderVariantMap variant) internal pure returns (bool) {
        return OrderVariantMap.unwrap(variant) & IS_ECDSA_BIT != 0;
    }
}
