// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.0;

import {PoolSwap} from "../modules/Accounter.sol";
import {PoolRewardsUpdate} from "../modules/PoolRewardsManager.sol";

import {Asset} from "../types/Asset.sol";
import {Price} from "../types/PriceGraph.sol";
import {GenericOrder} from "../reference/GenericOrder.sol";
import {TopOfBlockOrderEnvelope} from "../types/TopOfBlockEnvelope.sol";

/// @author philogy <https://github.com/philogy>
library DecoderLib {
    function unpack(bytes calldata data)
        internal
        pure
        returns (
            Asset[] calldata assets,
            Price[] calldata initialPrices,
            TopOfBlockOrderEnvelope[] calldata topOfBlockOrders,
            PoolSwap[] calldata swaps,
            GenericOrder[] calldata orders,
            PoolRewardsUpdate[] calldata poolRewardsUpdates
        )
    {
        assembly {
            let offset := data.offset
            let objOffset

            objOffset := add(offset, calldataload(add(offset, 0x00)))
            assets.length := calldataload(objOffset)
            assets.offset := add(objOffset, 0x20)

            objOffset := add(offset, calldataload(add(offset, 0x20)))
            initialPrices.length := calldataload(objOffset)
            initialPrices.offset := add(objOffset, 0x20)

            objOffset := add(offset, calldataload(add(offset, 0x40)))
            topOfBlockOrders.length := calldataload(objOffset)
            topOfBlockOrders.offset := add(objOffset, 0x20)

            objOffset := add(offset, calldataload(add(offset, 0x60)))
            swaps.length := calldataload(objOffset)
            swaps.offset := add(objOffset, 0x20)

            objOffset := add(offset, calldataload(add(offset, 0x80)))
            orders.length := calldataload(objOffset)
            orders.offset := add(objOffset, 0x20)

            objOffset := add(offset, calldataload(add(offset, 0xa0)))
            poolRewardsUpdates.length := calldataload(objOffset)
            poolRewardsUpdates.offset := add(objOffset, 0x20)
        }
    }
}
