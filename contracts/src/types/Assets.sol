// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {CalldataReader} from "./CalldataReader.sol";
import {StructArrayLib} from "../libraries/StructArrayLib.sol";

type Asset is uint256;

type Assets is uint256;

using AssetsLib for Asset global;
using AssetsLib for Assets global;

/// @author philogy <https://github.com/philogy>
library AssetsLib {
    using StructArrayLib for uint256;

    error AssetsOutOfOrderOrNotUnique();

    /// @dev Size of a single encoded asset (b20:addr ++ b16:borrow ++ b16:save ++ b16:settle)
    uint256 internal constant ASSET_BYTES = 68;

    uint256 internal constant ADDR_OFFSET = 0;
    uint256 internal constant BORROW_OFFSET = 20;
    uint256 internal constant SAVE_OFFSET = 36;
    uint256 internal constant SETTLE_OFFSET = 52;

    function readFromAndValidate(CalldataReader reader) internal pure returns (CalldataReader, Assets) {
        uint256 packed;
        (reader, packed) = StructArrayLib.readPackedFrom(reader, ASSET_BYTES);
        return (reader, Assets.wrap(packed)._validated());
    }

    function len(Assets assets) internal pure returns (uint256) {
        return assets.into().len();
    }

    function ptr(Assets assets) internal pure returns (uint256) {
        return assets.into().ptr();
    }

    function _validated(Assets self) internal pure returns (Assets) {
        uint256 length = self.len();
        address lastAddr = address(0);
        for (uint256 i = 0; i < length; i++) {
            address newAddr = self.get(i).addr();
            if (newAddr <= lastAddr) revert AssetsOutOfOrderOrNotUnique();
            lastAddr = newAddr;
        }
        return self;
    }

    function into(Assets self) internal pure returns (uint256) {
        return Assets.unwrap(self);
    }

    function into(Asset self) internal pure returns (uint256) {
        return Asset.unwrap(self);
    }

    function get(Assets self, uint256 index) internal pure returns (Asset asset) {
        self.into()._checkBounds(index);
        return Asset.wrap(self.into().ptr() + index * ASSET_BYTES);
    }

    function addr(Asset self) internal pure returns (address) {
        return self.into().readAddressMemberFromPtr(ADDR_OFFSET);
    }

    function borrow(Asset self) internal pure returns (uint256) {
        return self.into().readU128MemberFromPtr(BORROW_OFFSET);
    }

    function save(Asset self) internal pure returns (uint256) {
        return self.into().readU128MemberFromPtr(SAVE_OFFSET);
    }

    function settle(Asset self) internal pure returns (uint256) {
        return self.into().readU128MemberFromPtr(SETTLE_OFFSET);
    }
}
