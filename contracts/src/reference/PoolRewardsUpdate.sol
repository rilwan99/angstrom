// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {SafeCastLib} from "solady/src/utils/SafeCastLib.sol";
import {Asset, AssetLib} from "./Asset.sol";
import {AssetIndexPair} from "../types/AssetIndexPair.sol";

struct RewardsUpdate {
    int24 startTick;
    uint128 startLiquidity;
    uint128 currentTickReward;
    uint128[] amounts;
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
        bytes memory encodedAmounts;
        for (uint256 i = 0; i < self.amounts.length; i++) {
            encodedAmounts = bytes.concat(encodedAmounts, bytes16(self.amounts[i]));
        }
        encodedAmounts = bytes.concat(bytes2(encodedAmounts.length.toUint16()), encodedAmounts);

        return bytes.concat(
            bytes3(uint24(self.startTick)),
            bytes16(self.startLiquidity),
            encodedAmounts,
            bytes16(self.currentTickReward)
        );
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
