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

import {AssetArray, AssetLib} from "./types/Asset.sol";
import {PairArray, Pair, PairLib} from "./types/Pair.sol";
import {ToBOrderBuffer} from "./types/ToBOrderBuffer.sol";
import {UserOrderBuffer} from "./types/UserOrderBuffer.sol";
import {OrderVariant} from "./types/OrderVariant.sol";
import {HookBuffer, HookBufferLib} from "./types/HookBuffer.sol";
import {CalldataReader, CalldataReaderLib} from "./types/CalldataReader.sol";
import {SignatureLib} from "./libraries/SignatureLib.sol";
import {PriceAB as PriceOutVsIn, AmountA as AmountOut, AmountB as AmountIn} from "./types/Price.sol";

import {RayMathLib} from "./libraries/RayMathLib.sol";

import {IUnlockCallback} from "v4-core/src/interfaces/callback/IUnlockCallback.sol";

import {safeconsole as console} from "forge-std/safeconsole.sol";
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
    using SignatureLib for CalldataReader;

    error LimitViolated();

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

        AssetArray assets;
        (reader, assets) = AssetLib.readFromAndValidate(reader);
        PairArray pairs;
        (reader, pairs) = PairLib.readFromAndValidate(reader);

        _borrowAssets(assets);
        reader = _execPoolSwaps(reader, assets);
        reader = _validateAndExecuteToB(reader, assets);
        // _rewardPools(poolRewardsUpdates, freeBalance);
        reader = _validateAndExecuteOrders(reader, assets, pairs);
        _saveAndSettle(assets);

        reader.requireInBoundsOf(data);

        return new bytes(0);
    }

    function _validateAndExecuteToB(CalldataReader reader, AssetArray assets) internal returns (CalldataReader) {
        CalldataReader end;
        (reader, end) = reader.readU24End();

        TypedDataHasher typedHasher = _erc712Hasher();
        ToBOrderBuffer memory buffer;
        // No ERC712 variants so typehash can remain unchanged.
        buffer.setTypeHash();

        // Purposefully devolve into an endless loop if the specified length isn't exactly used s.t.
        // `reader == end` at some point.
        while (reader != end) {
            OrderVariant variant;
            (reader, variant) = reader.readVariant();
            buffer.useInternal = variant.useInternal();

            (reader, buffer.quantityIn) = reader.readU128();
            (reader, buffer.quantityOut) = reader.readU128();

            (reader, buffer.assetIn) = assets.readAssetAddrFrom(reader);
            (reader, buffer.assetOut) = assets.readAssetAddrFrom(reader);
            (reader, buffer.recipient) = variant.hasRecipient() ? reader.readAddr() : (reader, address(0));

            HookBuffer hook;
            (reader, hook, buffer.hookDataHash) = HookBufferLib.readFrom(reader, variant.noHook());

            // The `.hash` method validates the `block.number` for flash orders.
            bytes32 orderHash = typedHasher.hashTypedData(buffer.hash());

            _invalidateOrderHash(orderHash);

            address from;
            (reader, from) =
                variant.isEcdsa() ? reader.readAndCheckEcdsa(orderHash) : reader.readAndCheckERC1271(orderHash);

            hook.tryTrigger(from);

            _accountIn(from, buffer.assetIn, AmountIn.wrap(buffer.quantityIn), variant.useInternal());
            address to = _defaultOr(buffer.recipient, from);
            _accountOut(to, buffer.assetOut, AmountOut.wrap(buffer.quantityOut), variant.useInternal());
        }

        return reader;
    }

    function _validateAndExecuteOrders(CalldataReader reader, AssetArray assets, PairArray pairs)
        internal
        returns (CalldataReader)
    {
        TypedDataHasher typedHasher = _erc712Hasher();
        UserOrderBuffer memory buffer;

        CalldataReader end;
        (reader, end) = reader.readU24End();

        // Purposefully devolve into an endless loop if the specified length isn't exactly used s.t.
        // `reader == end` at some point.
        while (reader != end) {
            OrderVariant variant;
            (reader, variant) = reader.readVariant();

            buffer.setTypeHash(variant);
            buffer.useInternal = variant.useInternal();

            // Load and lookup asset in/out and dependent values.
            PriceOutVsIn price;
            {
                uint256 priceOutVsIn;
                (reader, buffer.assetIn, buffer.assetOut, priceOutVsIn) =
                    pairs.decodeAndLookupPair(reader, assets, variant.aToB());
                price = PriceOutVsIn.wrap(priceOutVsIn);
            }

            (reader, buffer.minPrice) = reader.readU256();
            if (price.into() < buffer.minPrice) revert LimitViolated();

            HookBuffer hook;
            (reader, hook, buffer.hookDataHash) = HookBufferLib.readFrom(reader, variant.noHook());

            // For flash orders sets the current block number as `validForBlock` so that it's
            // implicitly validated via hashing later.
            reader = buffer.readOrderValidation(reader, variant);

            AmountIn amountIn;
            AmountOut amountOut;
            (reader, amountIn, amountOut) = buffer.loadAndComputeQuantity(reader, variant, price, halfSpreadRay);

            (reader, buffer.recipient) = variant.hasRecipient() ? reader.readAddr() : (reader, address(0));

            bytes32 orderHash = buffer.hash712(variant, typedHasher);

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

            hook.tryTrigger(from);

            _accountIn(from, buffer.assetIn, amountIn, variant.useInternal());
            address to = _defaultOr(buffer.recipient, from);
            _accountOut(to, buffer.assetOut, amountOut, variant.useInternal());
        }

        return reader;
    }

    function _defaultOr(address defaultAddr, address alt) internal pure returns (address addr) {
        assembly {
            addr := xor(shr(defaultAddr, alt), defaultAddr)
        }
    }
}
