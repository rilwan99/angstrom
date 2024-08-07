// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {CalldataReader} from "./CalldataReader.sol";
import {StructArrayLib} from "../libraries/StructArrayLib.sol";
import {AssetArray} from "./Asset.sol";
import {AssetIndexPair} from "./AssetIndexPair.sol";

type PoolSwap is uint256;

using PoolSwapLib for PoolSwap global;

/// @author philogy <https://github.com/philogy>
library PoolSwapLib {
    using StructArrayLib for uint256;

    uint256 internal constant SWAP_BYTES = 19;

    uint256 internal constant INDICES_OFFSET = 0;
    uint256 internal constant AMOUNT_IN_OFFSET = 3;

    function readFrom(CalldataReader reader) internal pure returns (CalldataReader, PoolSwap swap) {
        assembly {
            swap := reader
            reader := add(reader, SWAP_BYTES)
        }
        return (reader, swap);
    }

    function into(PoolSwap self) internal pure returns (uint256) {
        return PoolSwap.unwrap(self);
    }

    function assetIndices(PoolSwap self) internal pure returns (AssetIndexPair) {
        uint24 data = self.into().readU24MemberFromPtr(INDICES_OFFSET);
        return AssetIndexPair.wrap(data);
    }

    function amountIn(PoolSwap self) internal pure returns (uint256) {
        return self.into().readU128MemberFromPtr(AMOUNT_IN_OFFSET);
    }

    function getSwapAssets(PoolSwap self, AssetArray assets)
        internal
        pure
        returns (address asset0, address asset1, bool zeroForOne)
    {
        AssetIndexPair indices = self.assetIndices();
        address assetIn = assets.get(indices.indexA()).addr();
        address assetOut = assets.get(indices.indexB()).addr();
        zeroForOne = assetIn < assetOut;
        (asset0, asset1) = zeroForOne ? (assetIn, assetOut) : (assetOut, assetIn);
    }
}
