// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {CalldataReader} from "./CalldataReader.sol";
import {StructArrayLib} from "../libraries/StructArrayLib.sol";
import {AssetArray} from "./Asset.sol";
import {RayMathLib} from "../libraries/RayMathLib.sol";
import {FormatLib} from "super-sol/libraries/FormatLib.sol";
import {AssetIndexPair, AssetIndexPairLib} from "./AssetIndexPair.sol";

import {console} from "forge-std/console.sol";

type Pair is uint256;

type PairArray is uint256;

using PairLib for Pair global;
using PairLib for PairArray global;

/// @author philogy <https://github.com/philogy>
library PairLib {
    using FormatLib for *;
    using RayMathLib for uint256;
    using StructArrayLib for uint256;

    error OutOfOrderOrDuplicatePairs();
    error UnsortedPair();

    uint256 internal constant PAIR_BYTES = 35;

    uint256 internal constant INDICES_OFFSET = 0;
    uint256 internal constant PRICE_AB_OFFSET = 3;

    function readFromAndValidate(CalldataReader reader) internal pure returns (CalldataReader, PairArray) {
        uint256 packed;
        (reader, packed) = StructArrayLib.readPackedFrom(reader, PAIR_BYTES);
        return (reader, PairArray.wrap(packed)._validated());
    }

    function into(PairArray pairs) internal pure returns (uint256) {
        return PairArray.unwrap(pairs);
    }

    function into(Pair pair) internal pure returns (uint256) {
        return Pair.unwrap(pair);
    }

    function len(PairArray pairs) internal pure returns (uint256) {
        return pairs.into().len();
    }

    function _validated(PairArray self) internal pure returns (PairArray) {
        uint256 length = self.len();
        if (length == 0) return self;

        AssetIndexPair lastIndices = self.get(0).assetIndices();
        for (uint256 i = 1; i < length; i++) {
            AssetIndexPair indices = self.get(i).assetIndices();
            if (indices <= lastIndices) revert OutOfOrderOrDuplicatePairs();
            lastIndices = indices;
        }

        return self;
    }

    function get(PairArray self, uint256 index) internal pure returns (Pair asset) {
        self.into()._checkBounds(index);
        return Pair.wrap(self.into().ptr() + index * PAIR_BYTES);
    }

    function assetIndices(Pair self) internal pure returns (AssetIndexPair) {
        uint24 data = self.into().readU24MemberFromPtr(INDICES_OFFSET);
        return _validated(AssetIndexPair.wrap(data));
    }

    function priceAB(Pair self) internal pure returns (uint256 priceAB_) {
        priceAB_ = self.into().readU256MemberFromPtr(PRICE_AB_OFFSET);
    }

    function decodeAndLookupPair(PairArray self, CalldataReader reader, AssetArray assets, bool aToB)
        internal
        pure
        returns (CalldataReader, address assetIn, address assetOut, uint256 priceOutVsIn)
    {
        uint256 pairIndex;
        (reader, pairIndex) = reader.readU16();
        Pair pair = self.get(pairIndex);
        uint256 pAB = pair.priceAB();

        AssetIndexPair indices = pair.assetIndices();
        address assetA = assets.get(indices.indexA()).addr();
        address assetB = assets.get(indices.indexB()).addr();

        (assetIn, assetOut, priceOutVsIn) = aToB ? (assetA, assetB, pAB.invRay()) : (assetB, assetA, pAB);

        return (reader, assetIn, assetOut, priceOutVsIn);
    }

    function _validated(AssetIndexPair self) internal pure returns (AssetIndexPair) {
        if (self.indexB() <= self.indexA()) revert UnsortedPair();
        return self;
    }
}
