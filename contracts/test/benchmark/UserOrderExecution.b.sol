// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.24;

import {BaseTest} from "../_helpers/BaseTest.sol";

import {ConversionLib} from "../../src/libraries/ConversionLib.sol";
import {UserOrder, UserOrderLib} from "../../src/reference/UserOrder.sol";
import {
    PartialStandingOrder,
    ExactStandingOrder,
    PartialFlashOrder,
    ExactFlashOrder,
    OrderMeta
} from "../../src/reference/OrderTypes.sol";
import {ExtAngstrom} from "../_view-ext/ExtAngstrom.sol";
import {TopOfBlockOrderEnvelope, PoolSwap, GenericOrder, PoolRewardsUpdate} from "src/Angstrom.sol";

import {PoolId, PoolIdLibrary} from "v4-core/src/types/PoolId.sol";
import {UniV4Inspector} from "../_view-ext/UniV4Inspector.sol";
import {MockERC20} from "super-sol/mocks/MockERC20.sol";
import {HookDeployer} from "../_helpers/HookDeployer.sol";
import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";
import {RayMathLib} from "../../src/libraries/RayMathLib.sol";
import {PairIterator, Pair, PairIteratorLib} from "./_helpers/PairIterator.sol";
import {TypedDataHasher, TypedDataHasherLib} from "src/types/TypedDataHasher.sol";
import {Asset} from "src/types/Asset.sol";
import {Price, AssetIndex} from "src/types/PriceGraph.sol";

import {PoolGate} from "../_helpers/PoolGate.sol";
import {Trader} from "../_helpers/types/Trader.sol";
import {PRNG} from "super-sol/collections/PRNG.sol";
import {FormatLib} from "super-sol/libraries/FormatLib.sol";

import {console2 as console} from "forge-std/console2.sol";

/// @author philogy <https://github.com/philogy>
contract UserOrderExecution is BaseTest, HookDeployer {
    using FormatLib for *;
    using FixedPointMathLib for *;
    using RayMathLib for uint256;

    address gov = makeAddr("gov");
    address owner = makeAddr("owner");
    address node = makeAddr("node");

    UniV4Inspector uniV4;
    PoolGate gate;
    ExtAngstrom angstrom;

    uint256 constant TOTAL_ASSETS = 4;
    uint256 constant TOTAL_PAIRS = TOTAL_ASSETS * (TOTAL_ASSETS - 1) / 2;
    address[] assets;

    uint256 constant TOTAL_ACTORS = 1_000;

    function setUp() public {
        for (uint256 i = 0; i < TOTAL_ASSETS; i++) {
            address newToken = address(new MockERC20());
            assets.push(newToken);
            for (uint256 j = 0; j < i; j++) {
                if (assets[j] > newToken) {
                    (assets[j], assets[i]) = (assets[i], assets[j]);
                    break;
                }
            }
        }

        vm.startPrank(owner);
        uniV4 = new UniV4Inspector();
        gate = new PoolGate(address(uniV4));
        vm.stopPrank();

        (bool succ, address addr,) = deployHook(
            abi.encodePacked(type(ExtAngstrom).creationCode, abi.encode(address(uniV4), gov)),
            _angstromFlags(),
            _newFactory()
        );
        assertTrue(succ, "Failed to deploy angstrom");
        angstrom = ExtAngstrom(addr);

        vm.warp(REAL_TIMESTAMP);

        address[] memory nodes = new address[](1);
        nodes[0] = node;
        vm.prank(gov);
        angstrom.updateNodes(nodes);
    }

    mapping(uint256 => bool) usedIndices;
    mapping(address trader => mapping(address asset => uint256)) traderTotals;

    mapping(address => uint256) totalOuts;

    struct test_gas1Vars {
        PairIterator pairIter;
        Trader[] traders;
        UserOrder[] orders;
        OrderMeta empty;
        Asset[] assets;
        Price[] prices;
        Pair pair;
        TypedDataHasher hasher;
        uint256 amount0Cleared;
        uint256 amount1Cleared;
        bool zeroToOne;
        uint256 refPrice;
        uint256 amountOut;
        uint256 amountIn;
        address assetIn;
        address assetOut;
        bool isFlash;
        uint256 minPrice;
        Trader trader;
        uint256 p;
        uint256 minAmountIn;
        uint256 maxAmountIn;
        uint256 deadline;
        GenericOrder[] finalOrders;
    }

    function test_gas_execute_limit_1() public {
        vm.pauseGasMetering();

        angstrom.updateLastBlock();
        vm.roll(block.number + 1);

        test_gas1Vars memory v;
        PRNG memory rng = _rng("exec_1");

        // Initialize random pairs with order counts.
        v.pairIter =
            PairIteratorLib.init(assets, _randPrices(rng, TOTAL_PAIRS), _randOrderCounts(rng, TOTAL_PAIRS, 12, 32));
        v.orders = new UserOrder[](v.pairIter.totalOrders());

        // Initialize trader actors.
        v.traders = makeTraders(TOTAL_ACTORS);

        v.prices = new Price[](v.pairIter.prices.length);
        v.hasher = TypedDataHasherLib.init(angstrom.DOMAIN_SEPARATOR());
        v.finalOrders = new GenericOrder[](v.orders.length);

        for (uint256 j = 0; j < v.pairIter.prices.length; j++) {
            Price memory price = v.prices[j];
            price.outIndex = AssetIndex.wrap(u16(v.pairIter.asset1Index));
            price.inIndex = AssetIndex.wrap(u16(v.pairIter.asset0Index));
            v.pair = v.pairIter.next(); // price = amount1 / amount0
            price.price = v.pair.price;

            v.amount0Cleared = 0;
            v.amount1Cleared = 0;

            for (uint256 i = 0; i < v.pair.orderCount; i++) {
                uint256 oi = v.pair.orderOffset + i;
                assertFalse(usedIndices[oi]);
                usedIndices[oi] = true;

                bool isLast = i == v.pair.orderCount - 1;
                v.zeroToOne = isLast ? v.amount1Cleared < v.amount0Cleared.mulRay(v.pair.price) : rng.randbool();

                v.refPrice = v.zeroToOne ? v.pair.price : v.pair.price.invRay(); // out / in
                v.amountOut = isLast
                    ? (
                        v.zeroToOne
                            ? v.amount0Cleared.mulRay(v.pair.price) - v.amount1Cleared
                            : v.amount1Cleared.divRay(v.pair.price) - v.amount0Cleared
                    )
                    : rng.randuint((v.refPrice / 10).rayToWad(), (v.refPrice * 10).rayToWad());
                v.amountIn = v.amountOut.divRay(v.refPrice);

                if (v.zeroToOne) {
                    v.amount1Cleared += v.amountOut;
                } else {
                    v.amount0Cleared += v.amountOut;
                }

                v.assetIn = v.zeroToOne ? v.pair.asset0 : v.pair.asset1;
                v.assetOut = v.zeroToOne ? v.pair.asset1 : v.pair.asset0;

                v.isFlash = rng.randbool();
                v.minPrice = v.refPrice.mulWad(rng.randuint(0.89e18, 1.0e18));
                v.trader = v.traders[rng.randuint(v.traders.length)];
                v.deadline = block.timestamp + 240 + rng.randuint(0, 3600);
                traderTotals[v.trader.addr][v.assetIn] += v.amountIn;

                v.p = rng.randuint(1e18);
                if (v.p <= 0.4e18) {
                    v.minAmountIn = v.amountIn.mulWad(rng.randuint(0.2e18, 1.0e18));
                    v.maxAmountIn = v.amountIn.mulWad(rng.randuint(1.0e18, 10.0e18));
                    // Partial order
                    v.orders[oi] = v.isFlash
                        ? UserOrderLib.from(
                            PartialFlashOrder({
                                minAmountIn: v.minAmountIn,
                                maxAmountIn: v.maxAmountIn,
                                minPrice: v.minPrice,
                                assetIn: v.assetIn,
                                assetOut: v.assetOut,
                                recipient: v.trader.addr,
                                hookData: new bytes(0),
                                validForBlock: u64(block.number),
                                amountFilled: v.amountIn,
                                meta: v.empty
                            })
                        )
                        : UserOrderLib.from(
                            PartialStandingOrder({
                                minAmountIn: v.minAmountIn,
                                maxAmountIn: v.maxAmountIn,
                                minPrice: v.minPrice,
                                assetIn: v.assetIn,
                                assetOut: v.assetOut,
                                recipient: v.trader.addr,
                                hookData: new bytes(0),
                                nonce: u64(v.trader.getFreshNonce()),
                                deadline: u40(v.deadline),
                                amountFilled: v.amountIn,
                                meta: v.empty
                            })
                        );
                } else {
                    bool exactIn = v.p <= 0.7e18;
                    // Exact in/out order
                    v.orders[oi] = v.isFlash
                        ? UserOrderLib.from(
                            ExactFlashOrder({
                                exactIn: exactIn,
                                amount: exactIn ? v.amountIn : v.amountOut,
                                minPrice: v.minPrice,
                                assetIn: v.assetIn,
                                assetOut: v.assetOut,
                                recipient: v.trader.addr,
                                hookData: new bytes(0),
                                validForBlock: u64(block.number),
                                meta: v.empty
                            })
                        )
                        : UserOrderLib.from(
                            ExactStandingOrder({
                                exactIn: exactIn,
                                amount: exactIn ? v.amountIn : v.amountOut,
                                minPrice: v.minPrice,
                                assetIn: v.assetIn,
                                assetOut: v.assetOut,
                                recipient: v.trader.addr,
                                hookData: new bytes(0),
                                nonce: u64(v.trader.getFreshNonce()),
                                deadline: u40(v.deadline),
                                meta: v.empty
                            })
                        );
                }
                v.trader.sign(v.orders[oi], v.hasher);
                v.finalOrders[oi] = v.orders[oi].into(assets);
            }
            totalOuts[v.pair.asset0] += v.amount0Cleared;
            totalOuts[v.pair.asset1] += v.amount1Cleared;
        }
        assertTrue(v.pairIter.done());

        for (uint256 i = 0; i < v.traders.length; i++) {
            address trader = v.traders[i].addr;
            vm.startPrank(trader);
            for (uint256 j = 0; j < assets.length; j++) {
                address asset = assets[j];
                MockERC20 erc20 = MockERC20(asset);
                erc20.approve(address(angstrom), type(uint256).max);
                erc20.mint(trader, traderTotals[trader][asset]);
            }
            vm.stopPrank();
        }

        v.assets = new Asset[](assets.length);
        for (uint256 i = 0; i < assets.length; i++) {
            address addr = assets[i];
            Asset memory asset = v.assets[i];
            asset.addr = addr;
            asset.borrow = totalOuts[addr];
            asset.save = 0;
            asset.settle = totalOuts[addr];
            gate.mint(addr, totalOuts[addr]);
        }

        bytes memory payload = abi.encode(
            v.assets,
            v.prices,
            new TopOfBlockOrderEnvelope[](0),
            new PoolSwap[](0),
            v.finalOrders,
            new PoolRewardsUpdate[](0)
        );

        uint256 zeros;
        uint256 nonZeros;
        for (uint256 i = 0; i < payload.length; i++) {
            payload[i] == 0x00 ? zeros++ : nonZeros++;
        }

        console.log("totalOrders: %s\n", v.finalOrders.length);
        console.log("calldata cost: %s (%s, %s)", zeros * 4 + nonZeros * 16, zeros, nonZeros);

        vm.resumeGasMetering();

        vm.prank(node);
        angstrom.execute(payload);
    }

    function _rng(bytes32 seed) internal pure returns (PRNG memory) {
        return PRNG(uint256(seed));
    }

    function _randPrices(PRNG memory rng, uint256 n) internal pure returns (uint256[] memory prices) {
        prices = new uint256[](n);
        for (uint256 i = 0; i < n; i++) {
            uint256 mag = uint256(int256(10e18).powWad(int256(rng.randuint(4e18))));

            prices[i] = mag.wadToRay().mulRay(0.01e27);
        }
    }

    function _randOrderCounts(PRNG memory rng, uint256 n, uint256 low, uint256 high)
        internal
        pure
        returns (uint256[] memory counts)
    {
        counts = new uint256[](n);
        for (uint256 i = 0; i < n; i++) {
            counts[i] = rng.randchoice(0.2e18, 0, rng.randuint(low, high + 1));
        }
    }
}
