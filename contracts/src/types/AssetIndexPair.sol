// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

type AssetIndexPair is uint32;

using AssetIndexPairLib for AssetIndexPair global;
using {le as <=, ge as >=, lt as <, gt as >} for AssetIndexPair global;

function le(AssetIndexPair x, AssetIndexPair y) pure returns (bool) {
    return x.into() <= y.into();
}

function ge(AssetIndexPair x, AssetIndexPair y) pure returns (bool) {
    return x.into() >= y.into();
}

function lt(AssetIndexPair x, AssetIndexPair y) pure returns (bool) {
    return x.into() < y.into();
}

function gt(AssetIndexPair x, AssetIndexPair y) pure returns (bool) {
    return x.into() > y.into();
}

/// @author philogy <https://github.com/philogy>
library AssetIndexPairLib {
    error IndexTooLarge();

    uint256 internal constant INDEX_A_OFFSET = 16;
    uint256 internal constant INDEX_B_MASK = 0xffff;
    uint256 internal constant MAX_INDEX = (1 << 16) - 1;

    function _index(uint256 x) internal pure returns (uint256) {
        if (x > MAX_INDEX) revert IndexTooLarge();
        return x;
    }

    function into(AssetIndexPair self) internal pure returns (uint32) {
        return AssetIndexPair.unwrap(self);
    }

    function indexA(AssetIndexPair self) internal pure returns (uint16 index) {
        assembly {
            index := shr(INDEX_A_OFFSET, self)
        }
    }

    function indexB(AssetIndexPair self) internal pure returns (uint16 index) {
        assembly {
            index := and(INDEX_B_MASK, self)
        }
    }
}
