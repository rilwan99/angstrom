// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {AssetLib as ActualAssetLib} from "../types/Asset.sol";
import {SafeCastLib} from "solady/src/utils/SafeCastLib.sol";
import {AssetIndexPair, AssetIndexPairLib} from "../types/AssetIndexPair.sol";
import {BitPackLib} from "./BitPackLib.sol";

struct Asset {
    address addr;
    uint128 borrow;
    uint128 save;
    uint128 settle;
}

using AssetLib for Asset global;

/// @author philogy <https://github.com/philogy>
library AssetLib {
    using SafeCastLib for *;
    using AssetLib for *;
    using BitPackLib for uint256;

    function getIndexPair(Asset[] memory assets, address assetA, address assetB)
        internal
        pure
        returns (AssetIndexPair)
    {
        require(assetA < assetB, "assetA < assetB in index pair");
        uint256 indexAssetA = AssetIndexPairLib._index(assets.getIndex(assetA));
        uint256 indexAssetB = AssetIndexPairLib._index(assets.getIndex(assetB));
        require(indexAssetA < indexAssetB, "indexAssetA < indexAssetB in index pair");
        return AssetIndexPair.wrap(uint24((indexAssetA << AssetIndexPairLib.INDEX_A_OFFSET).bitOverlay(indexAssetB)));
    }

    function encode(Asset memory asset) internal pure returns (bytes memory b) {
        b = abi.encodePacked(asset.addr, asset.borrow, asset.save, asset.settle);
        require(b.length == ActualAssetLib.ASSET_BYTES, "Assets unexpected length");
    }

    function encode(Asset[] memory assets) internal pure returns (bytes memory b) {
        for (uint256 i = 0; i < assets.length; i++) {
            b = bytes.concat(b, assets[i].encode());
        }
        b = bytes.concat(bytes2(b.length.toUint16()), b);
    }

    function sort(Asset[] memory assets) internal pure {
        // Bubble sort because ain't nobody got time for that.
        for (uint256 i = 0; i < assets.length; i++) {
            for (uint256 j = i + 1; j < assets.length; j++) {
                if (assets[i].addr > assets[j].addr) (assets[i], assets[j]) = (assets[j], assets[i]);
            }
        }
    }

    function getIndex(Asset[] memory assets, address asset) internal pure returns (uint256) {
        for (uint256 i = 0; i < assets.length; i++) {
            if (asset == assets[i].addr) return i;
        }
        revert("Asset not found");
    }
}
