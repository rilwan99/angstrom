// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {BaseTest} from "test/_helpers/BaseTest.sol";
import {DecoderLib} from "src/libraries/DecoderLib.sol";
import {PRNG} from "super-sol/collections/PRNG.sol";

import {PoolSwap} from "src/modules/Accounter.sol";
import {PoolRewardsUpdate} from "src/modules/PoolRewardsManager.sol";

import {Asset} from "src/types/Asset.sol";
import {Price, AssetIndex} from "src/types/PriceGraph.sol";
import {GenericOrder, TopOfBlockOrderEnvelope, OrderType, OrderMode} from "src/types/OrderTypes.sol";

/// @author philogy <https://github.com/philogy>
contract DecoderLibTest is BaseTest {
    uint256 constant MAX_RAND_LEN = 5;

    function test_fuzzing_unpacks(uint256 seed) public {
        PRNG memory rng = PRNG(seed);

        Asset[] memory assets = new Asset[](rng.randuint(MAX_RAND_LEN));
        for (uint256 i = 0; i < assets.length; i++) {
            assets[i] = _randAsset(rng);
        }
        Price[] memory initialPrices = new Price[](rng.randuint(MAX_RAND_LEN));
        for (uint256 i = 0; i < initialPrices.length; i++) {
            initialPrices[i] = _randPrice(rng);
        }
        TopOfBlockOrderEnvelope[] memory topOfBlockOrders = new TopOfBlockOrderEnvelope[](rng.randuint(MAX_RAND_LEN));
        for (uint256 i = 0; i < topOfBlockOrders.length; i++) {
            topOfBlockOrders[i] = _randToB(rng);
        }
        PoolSwap[] memory swaps = new PoolSwap[](rng.randuint(MAX_RAND_LEN));
        for (uint256 i = 0; i < swaps.length; i++) {
            swaps[i] = _randSwap(rng);
        }
        GenericOrder[] memory orders = new GenericOrder[](rng.randuint(MAX_RAND_LEN));
        for (uint256 i = 0; i < orders.length; i++) {
            orders[i] = _randOrder(rng);
        }
        PoolRewardsUpdate[] memory poolRewardsUpdates = new PoolRewardsUpdate[](rng.randuint(MAX_RAND_LEN));
        for (uint256 i = 0; i < poolRewardsUpdates.length; i++) {
            poolRewardsUpdates[i] = _randUpdate(rng);
        }

        (
            Asset[] memory assetsOut,
            Price[] memory initialPricesOut,
            TopOfBlockOrderEnvelope[] memory topOfBlockOrdersOut,
            PoolSwap[] memory swapsOut,
            GenericOrder[] memory ordersOut,
            PoolRewardsUpdate[] memory poolRewardsUpdatesOut
        ) = this.unpack(abi.encode(assets, initialPrices, topOfBlockOrders, swaps, orders, poolRewardsUpdates));

        assertEq(abi.encode(assets), abi.encode(assetsOut));
        assertEq(abi.encode(initialPrices), abi.encode(initialPricesOut));
        assertEq(abi.encode(topOfBlockOrders), abi.encode(topOfBlockOrdersOut));
        assertEq(abi.encode(swaps), abi.encode(swapsOut));
        assertEq(abi.encode(orders), abi.encode(ordersOut));
        assertEq(abi.encode(poolRewardsUpdates), abi.encode(poolRewardsUpdatesOut));
    }

    function _randAsset(PRNG memory rng) internal pure returns (Asset memory) {
        return Asset(rng.randaddr(), rng.next(), rng.next(), rng.next());
    }

    function _randPrice(PRNG memory rng) internal pure returns (Price memory) {
        return Price(AssetIndex.wrap(rng.randuint16()), AssetIndex.wrap(rng.randuint16()), rng.next());
    }

    function _randToB(PRNG memory rng) internal pure returns (TopOfBlockOrderEnvelope memory) {
        return TopOfBlockOrderEnvelope(
            rng.next(),
            rng.next(),
            rng.randbool(),
            AssetIndex.wrap(rng.randuint16()),
            AssetIndex.wrap(rng.randuint16()),
            rng.randaddr(),
            rng.randaddr(),
            rng.randBytes(0, 200),
            rng.randaddr(),
            rng.randBytes(0, 200)
        );
    }

    function _randSwap(PRNG memory rng) internal pure returns (PoolSwap memory) {
        return
            PoolSwap(AssetIndex.wrap(rng.randuint16()), AssetIndex.wrap(rng.randuint16()), rng.randbool(), rng.next());
    }

    function _randOrder(PRNG memory rng) internal pure returns (GenericOrder memory) {
        return GenericOrder(
            OrderType(rng.randuint8(2)),
            OrderMode(rng.randuint(3)),
            rng.next(),
            rng.next(),
            rng.next(),
            rng.randbool(),
            AssetIndex.wrap(rng.randuint16()),
            AssetIndex.wrap(rng.randuint16()),
            rng.randuint64(),
            u40(rng.next()),
            rng.randaddr(),
            rng.randaddr(),
            rng.randBytes(0, 200),
            rng.next(),
            rng.randaddr(),
            rng.randBytes(0, 200)
        );
    }

    function _randUpdate(PRNG memory rng) internal pure returns (PoolRewardsUpdate memory) {
        uint256[] memory amounts = new uint256[](rng.randuint(6));
        for (uint256 i = 0; i < amounts.length; i++) {
            amounts[i] = rng.next();
        }
        return PoolRewardsUpdate(
            AssetIndex.wrap(rng.randuint16()),
            AssetIndex.wrap(rng.randuint16()),
            int24(uint24(rng.randuint(1 << 24))),
            uint128(rng.randuint(1 << 128)),
            rng.next(),
            amounts
        );
    }

    function unpack(bytes calldata data)
        external
        pure
        returns (
            Asset[] calldata,
            Price[] calldata,
            TopOfBlockOrderEnvelope[] calldata,
            PoolSwap[] calldata,
            GenericOrder[] calldata,
            PoolRewardsUpdate[] calldata
        )
    {
        return DecoderLib.unpack(data);
    }
}
