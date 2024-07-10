// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {PriceGraphLib, PriceGraph, AssetIndex} from "./PriceGraph.sol";
import {Asset} from "./Asset.sol";

struct Globals {
    PriceGraph prices;
    Asset[] assets;
}

using GlobalsLib for Globals global;

library GlobalsLib {
    function get(Globals memory self, AssetIndex index) internal pure returns (address) {
        return self.assets[index.into()].addr;
    }
}
