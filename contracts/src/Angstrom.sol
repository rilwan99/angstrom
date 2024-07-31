// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.26;

import {ERC712} from "./modules/ERC712.sol";
import {NodeManager} from "./modules/NodeManager.sol";
import {Accounter, PoolSwap} from "./modules/Accounter.sol";
import {PoolRewardsManager, PoolRewardsUpdate} from "./modules/PoolRewardsManager.sol";
import {UnorderedNonces} from "./modules/UnorderedNonces.sol";
import {UniConsumer} from "./modules/UniConsumer.sol";

import {Globals} from "./types/Globals.sol";
import {Asset} from "./types/Asset.sol";
import {tuint256} from "transient-goodies/TransientPrimitives.sol";
import {PriceGraphLib, PriceGraph, AssetIndex, Price} from "./types/PriceGraph.sol";
import {TopOfBlockOrderEnvelope} from "./types/TopOfBlockEnvelope.sol";
import {TypedDataHasher} from "./types/TypedDataHasher.sol";
import {OrderBuffer, MaybeHook, OrderBufferLib} from "./types/OrderBuffer.sol";
import {Hooks} from "v4-core/src/libraries/Hooks.sol";
import {PoolKey} from "v4-core/src/types/PoolKey.sol";
import {SET_POOL_FEE, TICK_SPACING} from "./Constants.sol";
import {CalldataReader, CalldataReaderLib} from "super-sol/collections/CalldataReader.sol";

import {SafeCastLib} from "solady/src/utils/SafeCastLib.sol";
import {DecoderLib} from "./libraries/DecoderLib.sol";
import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";
import {SignatureCheckerLib} from "solady/src/utils/SignatureCheckerLib.sol";
import {RayMathLib} from "./libraries/RayMathLib.sol";

import {IAngstromComposable} from "./interfaces/IAngstromComposable.sol";
import {IUnlockCallback} from "v4-core/src/interfaces/callback/IUnlockCallback.sol";
import {IPoolManager, IUniV4} from "./interfaces/IUniV4.sol";

/// @author philogy <https://github.com/philogy>
contract Angstrom is ERC712, Accounter, UnorderedNonces, PoolRewardsManager, NodeManager, IUnlockCallback {
    using RayMathLib for uint256;
    using IUniV4 for IPoolManager;
    using SafeCastLib for uint256;
    using FixedPointMathLib for uint256;

    error InvalidPoolKey();

    error AssetsOutOfOrder();
    error OnlyOncePerBlock();

    error LimitViolated();
    error Expired();
    error OrderAlreadyExecuted();

    error FillingTooLittle();
    error FillingTooMuch();
    error InvalidSignature();
    error Unresolved();

    uint256 internal constant ECRECOVER_ADDR = 1;

    // persistent storage
    uint256 public lastBlockUpdated;
    uint256 public halfSpreadRay;

    // transient storage
    mapping(bytes32 => tuint256) internal alreadyExecuted;

    constructor(address uniV4PoolManager, address governance) UniConsumer(uniV4PoolManager) NodeManager(governance) {
        _checkHookPermissions(Hooks.BEFORE_SWAP_FLAG | Hooks.BEFORE_INITIALIZE_FLAG);
    }

    function beforeInitialize(address, PoolKey calldata poolKey, uint160) external view onlyUniV4 returns (bytes4) {
        if (poolKey.tickSpacing != TICK_SPACING || poolKey.fee != SET_POOL_FEE) revert InvalidPoolKey();
        return this.beforeInitialize.selector;
    }

    function execute(bytes calldata data) external onlyNode {
        UNI_V4.unlock(data);
    }

    function unlockCallback(bytes calldata data) external override onlyUniV4 returns (bytes memory) {
        (
            Asset[] calldata assets,
            Price[] calldata initialPrices,
            TopOfBlockOrderEnvelope[] calldata topOfBlockOrders,
            PoolSwap[] calldata swaps,
            bytes calldata encodedOrders,
            PoolRewardsUpdate[] calldata poolRewardsUpdates
        ) = DecoderLib.unpack(data);

        Globals memory g = _validateAndInitGlobals(assets, initialPrices);

        _borrowAssets(assets);
        _execPoolSwaps(g, swaps);
        _validateAndExecuteToB(g, topOfBlockOrders);
        _rewardPools(g, poolRewardsUpdates, freeBalance);
        _validateAndExecuteOrders(g, encodedOrders);
        _saveAndSettle(assets);

        return new bytes(0);
    }

    function _validateAndInitGlobals(Asset[] calldata assets, Price[] calldata initialPrices)
        internal
        returns (Globals memory)
    {
        // Global bundle lock (prevents reentrancy & replaying flash orders).
        if (lastBlockUpdated == block.number) revert OnlyOncePerBlock();
        lastBlockUpdated = block.number;

        // Validate asset list.
        address lastAsset = assets[0].addr;
        for (uint256 i = 1; i < assets.length; i++) {
            address nextAsset = assets[i].addr;
            if (nextAsset <= lastAsset) revert AssetsOutOfOrder();
            lastAsset = nextAsset;
        }

        // Initialize and validate price graph.
        PriceGraph prices = PriceGraphLib.init(assets.length);
        for (uint256 i = 0; i < initialPrices.length; i++) {
            Price calldata init = initialPrices[i];
            prices.set(init.outIndex, init.inIndex, init.price);
        }

        return Globals({prices: prices, assets: assets});
    }

    function _validateAndExecuteToB(Globals memory g, TopOfBlockOrderEnvelope[] calldata orders) internal {
        for (uint256 i = 0; i < orders.length; i++) {
            TopOfBlockOrderEnvelope calldata order = orders[i];

            address assetIn = g.get(order.assetInIndex);
            address assetOut = g.get(order.assetOutIndex);

            // The `.hash` method validates the `block.number` for flash orders.
            bytes32 orderHash = order.hash(assetIn, assetOut);

            tuint256 storage executed = alreadyExecuted[orderHash];
            if (executed.get() != 0) revert OrderAlreadyExecuted();
            executed.set(1);

            if (!SignatureCheckerLib.isValidSignatureNow(order.from, _hashTypedData(orderHash), order.signature)) {
                revert InvalidSignature();
            }

            // TODO: Hook for ToB
            // if (order.hook != address(0)) {
            //     if (
            //         IAngstromComposable(order.hook).compose(order.from, order.hookPayload)
            //             != ~uint32(IAngstromComposable.compose.selector)
            //     ) revert InvalidHookReturn();
            // }

            _accountIn(order.from, assetIn, order.amountIn, order.useInternal);
            _accountOut(order.from, assetOut, order.amountOut, order.useInternal);
        }
    }

    function _validateAndExecuteOrders(Globals memory g, bytes calldata encodedOrders) internal {
        TypedDataHasher typedHasher = _erc712Hasher();
        CalldataReader reader = CalldataReaderLib.from(encodedOrders);
        OrderBuffer orderBuffer = OrderBufferLib.initBuffer();

        uint256 i = 0;

        while (reader.notAtEnd(encodedOrders)) {
            // Load 3-bit variant
            uint256 variant = reader.readU8();
            reader = reader.shifted(1);

            orderBuffer.clear();
            orderBuffer.setTypeHash(variant);

            // Load and lookup asset in/out and dependent values.
            uint256 priceOutVsIn;
            address assetIn;
            address assetOut;
            {
                AssetIndex assetInIndex = AssetIndex.wrap(reader.readU16());
                reader = reader.shifted(2);
                AssetIndex assetOutIndex = AssetIndex.wrap(reader.readU16());
                reader = reader.shifted(2);
                priceOutVsIn = g.prices.get(assetOutIndex, assetInIndex);
                assetIn = g.get(assetInIndex);
                orderBuffer.setAssetIn(assetIn);
                assetOut = g.get(assetOutIndex);
                orderBuffer.setAssetOut(assetOut);
            }

            // Load & validate price.
            {
                uint256 minPrice = reader.readU256();
                reader = reader.shifted(32);
                if (priceOutVsIn < minPrice) revert LimitViolated();
                orderBuffer.setMinPrice(minPrice);
            }

            MaybeHook hook;
            (reader, hook) = orderBuffer.readAndHashHook(reader, variant);
            // Adds current block number for flash orders.
            reader = orderBuffer.readOrderValidation(reader, variant);
            uint256 amountIn;
            uint256 amountOut;
            (reader, amountIn, amountOut) = _loadAndComputeQuantity(orderBuffer, reader, variant, priceOutVsIn);

            bool useInternal = variant & OrderBufferLib.VARIANT_USE_INTERNAL_BIT != 0;
            orderBuffer.setUseInternal(useInternal);

            address recipient;
            if (variant & OrderBufferLib.VARIANT_HAS_RECIPIENT != 0) {
                recipient = reader.readAddr();
                reader = reader.shifted(20);
            }
            orderBuffer.setRecipient(recipient);

            bytes32 orderHash = typedHasher.hashTypedData(orderBuffer.hash(variant));

            address from;
            (reader, from) = _authenticate(reader, orderHash, variant);

            recipient = recipient == address(0) ? from : recipient;

            if (variant & OrderBufferLib.VARIANT_IS_FLASH_BIT != 0) {
                tuint256 storage executed = alreadyExecuted[orderHash];
                if (executed.get() != 0) revert OrderAlreadyExecuted();
                executed.set(1);
            } else {
                {
                    uint256 dead = orderBuffer.deadline();
                    if (block.timestamp > dead) revert Expired();
                }
                _useNonce(from, orderBuffer.nonce());
            }

            hook.triggerIfSome(from);

            _accountIn(from, assetIn, amountIn, useInternal);
            _accountOut(recipient, assetOut, amountOut, useInternal);
        }
    }

    function _loadAndComputeQuantity(OrderBuffer buffer, CalldataReader reader, uint256 variant, uint256 priceOutVsIn)
        internal
        view
        returns (CalldataReader, uint256 quantityIn, uint256 quantityOut)
    {
        uint256 quantity;
        if (variant & OrderBufferLib.VARIANT_IS_EXACT_BIT != 0) {
            quantity = reader.readU128();
            reader = reader.shifted(16);
            buffer.setQuantityExact(variant, quantity);
        } else {
            uint256 minQuantityIn = reader.readU128();
            reader = reader.shifted(16);
            uint256 maxQuantityIn = reader.readU128();
            reader = reader.shifted(16);
            quantity = reader.readU128();
            reader = reader.shifted(16);
            buffer.setQuantityPartialMinMax(minQuantityIn, maxQuantityIn);

            if (quantity < minQuantityIn) revert FillingTooLittle();
            if (quantity > maxQuantityIn) revert FillingTooMuch();
        }

        uint256 feeRay = halfSpreadRay;
        if (variant & OrderBufferLib.VARIANT_IS_OUT_BIT != 0) {
            quantityOut = quantity;
            quantityIn = quantityOut.divRay(priceOutVsIn);
            quantityIn += quantityIn.mulRay(feeRay);
        } else {
            quantityIn = quantity;
            quantityOut = quantityIn.mulRay(priceOutVsIn);
            quantityOut -= quantityOut.mulRay(feeRay);
        }

        return (reader, quantityIn, quantityOut);
    }

    function _authenticate(CalldataReader reader, bytes32 orderHash, uint256 variant)
        internal
        view
        returns (CalldataReader, address from)
    {
        if (variant & OrderBufferLib.VARIANT_ECDSA_SIG != 0) {
            assembly ("memory-safe") {
                let free := mload(0x40)
                mstore(free, orderHash)
                // Ensure next word is clear
                mstore(add(free, 0x20), 0)
                // Read signature.
                calldatacopy(add(free, 0x3f), reader, 65)
                reader := add(reader, 65)
                // Call ec-Recover pre-compile (addr: 0x01).
                // Credit to Vectorized's ECDSA in Solady: https://github.com/Vectorized/solady/blob/a95f6714868cfe5d590145f936d0661bddff40d2/src/utils/ECDSA.sol#L108-L123
                from := mload(staticcall(gas(), ECRECOVER_ADDR, free, 0x80, 0x01, 0x20))
                if iszero(returndatasize()) {
                    mstore(0x00, 0x8baa579f /* InvalidSignature() */ )
                    revert(0x1c, 0x04)
                }
            }
        } else {
            from = reader.readAddr();
            reader = reader.shifted(20);
            bytes calldata signature;
            assembly ("memory-safe") {
                signature.length := shr(232, calldataload(reader))
                reader := add(reader, 3)
                signature.offset := reader
                reader := add(reader, signature.length)
            }
            if (!SignatureCheckerLib.isValidERC1271SignatureNowCalldata(from, orderHash, signature)) {
                revert InvalidSignature();
            }
        }

        return (reader, from);
    }
}
