// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {UserOrder, UserOrderLib} from "./UserOrder.sol";
import {Asset, AssetLib} from "./Asset.sol";
import {Pair, PairLib} from "./Pair.sol";
import {TopOfBlockOrder, OrdersLib} from "./OrderTypes.sol";
import {PoolUpdate, PoolUpdateLib} from "./PoolUpdate.sol";

import {console} from "forge-std/console.sol";

struct Bundle {
    Asset[] assets;
    Pair[] pairs;
    PoolUpdate[] poolUpdates;
    TopOfBlockOrder[] toBOrders;
    UserOrder[] userOrders;
}

using BundleLib for Bundle global;

/// @author philogy <https://github.com/philogy>
library BundleLib {
    using OrdersLib for TopOfBlockOrder[];
    using UserOrderLib for UserOrder[];
    using AssetLib for Asset[];
    using PairLib for Pair[];
    using PoolUpdateLib for PoolUpdate[];

    function encode(Bundle memory bundle) internal pure returns (bytes memory) {
        return bytes.concat(
            bundle.assets.encode(),
            bundle.pairs.encode(bundle.assets),
            bundle.poolUpdates.encode(bundle.assets),
            bundle.toBOrders.encode(bundle.assets),
            bundle.userOrders.encode(bundle.pairs)
        );
    }
}
