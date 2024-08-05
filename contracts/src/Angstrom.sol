// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.26;

import {ERC712} from "./modules/ERC712.sol";
import {NodeManager} from "./modules/NodeManager.sol";
import {Accounter, PoolSwap} from "./modules/Accounter.sol";
import {PoolRewardsManager, PoolRewardsUpdate} from "./modules/PoolRewardsManager.sol";
import {InvalidationManager} from "./modules/InvalidationManager.sol";
import {HookManager} from "./modules/HookManager.sol";
import {UniConsumer} from "./modules/UniConsumer.sol";

import {TypedDataHasher} from "./types/TypedDataHasher.sol";

import {Assets, AssetsLib} from "./types/Assets.sol";
import {Pairs, Pair, PairLib} from "./types/Pair.sol";
import {ToBOrderBuffer} from "./types/ToBOrderBuffer.sol";
import {OrderBuffer} from "./types/OrderBuffer.sol";
import {OrderVariant} from "./types/OrderVariant.sol";
import {HookBuffer, HookBufferLib} from "./types/HookBuffer.sol";
import {CalldataReader, CalldataReaderLib} from "./types/CalldataReader.sol";
import {SignatureLib} from "./libraries/SignatureLib.sol";
import {PriceAB as PriceOutVsIn, AmountA as AmountOut, AmountB as AmountIn} from "./types/Price.sol";

import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";
import {RayMathLib} from "./libraries/RayMathLib.sol";

import {IAngstromComposable} from "./interfaces/IAngstromComposable.sol";
import {IUnlockCallback} from "v4-core/src/interfaces/callback/IUnlockCallback.sol";

import {console} from "forge-std/console.sol";
import {FormatLib} from "super-sol/libraries/FormatLib.sol";
import {tuint256} from "transient-goodies/TransientPrimitives.sol";

/// @author philogy <https://github.com/philogy>
contract Angstrom is
    ERC712,
    Accounter,
    InvalidationManager,
    PoolRewardsManager,
    NodeManager,
    HookManager,
    IUnlockCallback
{
    using FormatLib for *;

    using RayMathLib for uint256;
    using FixedPointMathLib for uint256;
    using SignatureLib for CalldataReader;

    error LimitViolated();

    error FillingTooLittle();
    error FillingTooMuch();

    uint256 public halfSpreadRay;

    constructor(address uniV4PoolManager, address governance) UniConsumer(uniV4PoolManager) NodeManager(governance) {}

    function execute(bytes calldata data) external onlyNode {
        UNI_V4.unlock(data);
    }

    function unlockCallback(bytes calldata data)
        external
        override
        onlyUniV4
        blockWideNonReentrant
        returns (bytes memory)
    {
        CalldataReader reader = CalldataReaderLib.from(data);

        Assets assets;
        (reader, assets) = AssetsLib.readFromAndValidate(reader);
        Pairs pairs;
        (reader, pairs) = PairLib.readFromAndValidate(reader);

        _borrowAssets(assets);
        // _execPoolSwaps(swaps);
        reader = _validateAndExecuteToB(reader, assets);
        // _rewardPools(poolRewardsUpdates, freeBalance);
        reader = _validateAndExecuteOrders(reader, assets, pairs);
        _saveAndSettle(assets);

        reader.requireInBoundsOf(data);

        return new bytes(0);
    }

    function _validateAndExecuteToB(CalldataReader reader, Assets assets) internal returns (CalldataReader) {
        CalldataReader end;
        (reader, end) = reader.readU24End();

        ToBOrderBuffer memory buffer;
        // No ERC712 variants so typehash can remain unchanged.
        buffer.setTypeHash();
        TypedDataHasher typedHasher = _erc712Hasher();

        // Purposefully devolve into an endless loop if the specified length isn't exactly used s.t.
        // `reader == end` at some point.
        while (reader != end) {
            // Load variant.
            OrderVariant variant;
            (reader, variant) = reader.readVariant();

            (reader, buffer.quantityIn) = reader.readU128();
            (reader, buffer.quantityOut) = reader.readU128();
            buffer.useInternal = variant.useInternal();

            // Load and lookup asset in/out and dependent values.
            {
                uint256 assetInIndex;
                (reader, assetInIndex) = reader.readU8();
                buffer.assetIn = assets.get(assetInIndex).addr();

                uint256 assetOutIndex;
                (reader, assetOutIndex) = reader.readU8();
                buffer.assetOut = assets.get(assetOutIndex).addr();
            }

            (reader, buffer.recipient) = variant.hasRecipient() ? reader.readAddr() : (reader, address(0));

            HookBuffer hook;
            {
                bytes32 hookDataHash;
                (reader, hook, hookDataHash) = HookBufferLib.readFrom(reader, variant);
                buffer.hookDataHash = hookDataHash;
            }

            // The `.hash` method validates the `block.number` for flash orders.
            bytes32 orderHash = typedHasher.hashTypedData(buffer.hash());

            _invalidateOrderHash(orderHash);

            address from;
            (reader, from) =
                variant.isEcdsa() ? reader.readAndCheckEcdsa(orderHash) : reader.readAndCheckERC1271(orderHash);

            hook.tryTriggerAndFree(from);

            _accountIn(from, buffer.assetIn, buffer.quantityIn, variant.useInternal());
            address to = _defaultOr(buffer.recipient, from);
            _accountOut(to, buffer.assetOut, buffer.quantityOut, variant.useInternal());
        }

        // Must not use `buffer` past this point as it would be unsafe.
        buffer.tryFree();

        return reader;
    }

    function _validateAndExecuteOrders(CalldataReader reader, Assets assets, Pairs pairs)
        internal
        returns (CalldataReader)
    {
        TypedDataHasher typedHasher = _erc712Hasher();
        OrderBuffer memory buffer;

        CalldataReader end;
        (reader, end) = reader.readU24End();

        // Purposefully devolve into an endless loop if the specified length isn't exactly used s.t.
        // `reader == end` at some point.
        while (reader != end) {
            // Load variant.
            OrderVariant variant;
            (reader, variant) = reader.readVariant();

            buffer.setTypeHash(variant);

            // Load and lookup asset in/out and dependent values.
            PriceOutVsIn price;
            {
                uint256 pairIndex;
                (reader, pairIndex) = reader.readU16();

                uint256 priceOutVsIn;
                (buffer.assetIn, buffer.assetOut, priceOutVsIn) = pairs.decodePair(pairIndex, assets, variant.aToB());
                price = PriceOutVsIn.wrap(priceOutVsIn);
            }

            // Load & validate price.
            {
                uint256 minPrice;
                (reader, minPrice) = reader.readU256();
                if (price.into() < minPrice) revert LimitViolated();
                buffer.minPrice = minPrice;
            }

            HookBuffer hook;
            {
                bytes32 hookDataHash;
                (reader, hook, hookDataHash) = HookBufferLib.readFrom(reader, variant);
                buffer.hookDataHash = hookDataHash;
            }

            // Adds current block number for flash orders.
            reader = buffer.readOrderValidation(reader, variant);

            AmountIn amountIn;
            AmountOut amountOut;
            (reader, amountIn, amountOut) = _loadAndComputeQuantity(reader, buffer, variant, price);

            buffer.useInternal = variant.useInternal();

            (reader, buffer.recipient) = variant.hasRecipient() ? reader.readAddr() : (reader, address(0));

            bytes32 orderHash = typedHasher.hashTypedData(buffer.hash(variant));

            address from;
            (reader, from) = variant.isEcdsa()
                ? SignatureLib.readAndCheckEcdsa(reader, orderHash)
                : SignatureLib.readAndCheckERC1271(reader, orderHash);

            if (variant.isFlash()) {
                _invalidateOrderHash(orderHash);
            } else {
                _checkDeadline(buffer.deadline_or_empty);
                _invalidateNonce(from, buffer.nonce_or_validForBlock);
            }

            // Must not use `hook` past this point (could've been freed).
            hook.tryTriggerAndFree(from);

            _accountIn(from, buffer.assetIn, amountIn.into(), variant.useInternal());
            address to = _defaultOr(buffer.recipient, from);
            _accountOut(to, buffer.assetOut, amountOut.into(), variant.useInternal());
        }

        return reader;
    }

    function _loadAndComputeQuantity(
        CalldataReader reader,
        OrderBuffer memory buffer,
        OrderVariant variant,
        PriceOutVsIn price
    ) internal view returns (CalldataReader, AmountIn quantityIn, AmountOut quantityOut) {
        uint256 quantity;
        if (variant.isExact()) {
            (reader, quantity) = reader.readU128();
            buffer.setQuantityExact(variant, quantity);
        } else {
            // Partial order.
            uint256 minQuantityIn;
            uint256 maxQuantityIn;
            (reader, minQuantityIn) = reader.readU128();
            (reader, maxQuantityIn) = reader.readU128();
            (reader, quantity) = reader.readU128();
            buffer.exactIn_or_minQuantityIn = minQuantityIn;
            buffer.quantity_or_maxQuantityIn = maxQuantityIn;

            if (quantity < minQuantityIn) revert FillingTooLittle();
            if (quantity > maxQuantityIn) revert FillingTooMuch();
        }

        uint256 feeRay = halfSpreadRay;
        if (variant.isExactOut()) {
            quantityOut = AmountOut.wrap(quantity);
            quantityIn = price.convert(quantityOut);
            quantityIn = quantityIn + quantityIn.mulRayScalar(feeRay);
        } else {
            quantityIn = AmountIn.wrap(quantity);
            quantityOut = price.convert(quantityIn);
            quantityOut = quantityOut - quantityOut.mulRayScalar(feeRay);
        }

        return (reader, quantityIn, quantityOut);
    }

    function _defaultOr(address defaultAddr, address alt) internal pure returns (address addr) {
        assembly {
            addr := xor(shr(defaultAddr, alt), defaultAddr)
        }
    }
}
