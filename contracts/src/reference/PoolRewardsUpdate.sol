// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {SafeCastLib} from "solady/src/utils/SafeCastLib.sol";
import {Asset, AssetLib} from "./Asset.sol";
import {AssetIndexPair} from "../types/AssetIndexPair.sol";

struct RewardsUpdate {
    int24 startTick;
    uint128 startLiquidity;
    uint128[] quantities;
}

struct PoolRewardsUpdate {
    address asset0;
    address asset1;
    RewardsUpdate update;
}

using PoolRewardsUpdateLib for RewardsUpdate global;
using PoolRewardsUpdateLib for PoolRewardsUpdate global;

/// @author philogy <https://github.com/philogy>
library PoolRewardsUpdateLib {
    using SafeCastLib for uint256;
    using AssetLib for Asset[];

    function encode(RewardsUpdate memory self) internal pure returns (bytes memory) {
        bytes memory encodedQuantities;
        for (uint256 i = 0; i < self.quantities.length; i++) {
            encodedQuantities = bytes.concat(encodedQuantities, bytes16(self.quantities[i]));
        }
        encodedQuantities = bytes.concat(bytes2(encodedQuantities.length.toUint16()), encodedQuantities);

        return bytes.concat(bytes3(uint24(self.startTick)), bytes16(self.startLiquidity), encodedQuantities);
    }

    function encode(PoolRewardsUpdate memory self, Asset[] memory assets) internal pure returns (bytes memory) {
        AssetIndexPair indices = assets.getIndexPair({assetA: self.asset0, assetB: self.asset1});

        return bytes.concat(bytes3(indices.into()), self.update.encode());
    }

    function encode(PoolRewardsUpdate[] memory updates, Asset[] memory assets) internal pure returns (bytes memory b) {
        for (uint256 i = 0; i < updates.length; i++) {
            b = bytes.concat(b, updates[i].encode(assets));
        }
        b = bytes.concat(bytes3(b.length.toUint24()), b);
    }
}
