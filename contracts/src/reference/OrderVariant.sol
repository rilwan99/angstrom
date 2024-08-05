// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {OrderVariant as Variant, OrderVariantLib as VariantLib} from "../types/OrderVariant.sol";
import {BitPackLib} from "./BitPackLib.sol";

struct OrderVariant {
    bool isExact;
    bool isFlash;
    bool isOut;
    bool noHook;
    bool useInternal;
    bool hasRecipient;
    bool isEcdsa;
    bool aToB;
}

using OrderVariantLib for OrderVariant global;

/// @author philogy <https://github.com/philogy>
library OrderVariantLib {
    using BitPackLib for uint256;

    function encode(OrderVariant memory variant) internal pure returns (Variant) {
        // forgefmt: disable-next-item
        return Variant.wrap(
            uint8(
                (variant.isExact ? VariantLib.VARIANT_IS_EXACT_BIT : 0)
                    .bitOverlay(variant.isFlash ? VariantLib.VARIANT_IS_FLASH_BIT : 0)
                    .bitOverlay(variant.isOut ? VariantLib.VARIANT_IS_OUT_BIT : 0)
                    .bitOverlay( variant.noHook ? VariantLib.VARIANT_NO_HOOK_BIT : 0)
                    .bitOverlay(variant.useInternal ? VariantLib.VARIANT_USE_INTERNAL_BIT : 0)
                    .bitOverlay( variant.hasRecipient ? VariantLib.VARIANT_HAS_RECIPIENT : 0)
                    .bitOverlay(variant.isEcdsa ? VariantLib.VARIANT_IS_ECDSA_BIT : 0)
            )
        );
    }
}
