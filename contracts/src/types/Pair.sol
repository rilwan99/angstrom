// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {CalldataReader} from "./CalldataReader.sol";
import {StructArrayLib} from "../libraries/StructArrayLib.sol";
import {Assets} from "./Assets.sol";
import {RayMathLib} from "../libraries/RayMathLib.sol";
import {FormatLib} from "super-sol/libraries/FormatLib.sol";

import {console} from "forge-std/console.sol";

type Pair is uint256;

type Pairs is uint256;

using PairLib for Pair global;
using PairLib for Pairs global;

/// @author philogy <https://github.com/philogy>
library PairLib {
    using FormatLib for *;
    using RayMathLib for uint256;
    using StructArrayLib for uint256;
    using PairLib for uint24;

    error OutOfOrderOrDuplicatePairs();
    error UnsortedPair();

    uint256 internal constant PAIR_BYTES = 35;

    uint256 internal constant INDICES_OFFSET = 0;
    uint256 internal constant PRICE_AB_OFFSET = 3;

    uint256 internal constant INDEX_A_OFFSET = 12;
    uint256 internal constant INDEX_B_MASK = 0xfff;

    function readFromAndValidate(CalldataReader reader) internal pure returns (CalldataReader, Pairs) {
        uint256 packed;
        (reader, packed) = StructArrayLib.readPackedFrom(reader, PAIR_BYTES);
        return (reader, Pairs.wrap(packed)._validated());
    }

    function into(Pairs pairs) internal pure returns (uint256) {
        return Pairs.unwrap(pairs);
    }

    function into(Pair pair) internal pure returns (uint256) {
        return Pair.unwrap(pair);
    }

    function len(Pairs pairs) internal pure returns (uint256) {
        return pairs.into().len();
    }

    function _validated(Pairs self) internal pure returns (Pairs) {
        uint256 length = self.len();
        uint256 lastIndices = 0;
        for (uint256 i = 0; i < length; i++) {
            uint24 newIndices = self.get(i).indices();
            if (newIndices <= lastIndices) revert OutOfOrderOrDuplicatePairs();
            if (newIndices.indexB() <= newIndices.indexA()) revert UnsortedPair();
            lastIndices = newIndices;
        }
        return self;
    }

    function get(Pairs self, uint256 index) internal pure returns (Pair asset) {
        self.into()._checkBounds(index);
        return Pair.wrap(self.into().ptr() + index * PAIR_BYTES);
    }

    function indices(Pair self) internal pure returns (uint24 indices_) {
        indices_ = self.into().readU24MemberFromPtr(INDICES_OFFSET);
    }

    function priceAB(Pair self) internal pure returns (uint256 priceAB_) {
        priceAB_ = self.into().readU256MemberFromPtr(PRICE_AB_OFFSET);
    }

    function decodePair(Pairs self, uint256 pairIndex, Assets assets, bool aToB)
        internal
        pure
        returns (address assetIn, address assetOut, uint256 priceOutVsIn)
    {
        Pair pair = self.get(pairIndex);
        uint24 packedIndices = pair.indices();
        address assetA = assets.get(packedIndices.indexA()).addr();
        address assetB = assets.get(packedIndices.indexB()).addr();
        uint256 pAB = pair.priceAB();

        (assetIn, assetOut, priceOutVsIn) = aToB ? (assetA, assetB, pAB.invRay()) : (assetB, assetA, pAB);
    }

    function indexA(uint24 packedIndices) internal pure returns (uint16 index) {
        assembly {
            index := shr(INDEX_A_OFFSET, packedIndices)
        }
    }

    function indexB(uint24 packedIndices) internal pure returns (uint16 index) {
        assembly {
            index := and(INDEX_B_MASK, packedIndices)
        }
    }
}
