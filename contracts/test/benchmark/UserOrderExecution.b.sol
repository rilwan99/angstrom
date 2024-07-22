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
import {IntrospectiveAngstrom} from "../_introspective/IntrospectiveAngstrom.sol";

import {PoolId, PoolIdLibrary} from "v4-core/src/types/PoolId.sol";
import {UniV4Inspector} from "../_introspective/UniV4Inspector.sol";
import {MockERC20} from "super-sol/mocks/MockERC20.sol";
import {HookDeployer} from "../_helpers/HookDeployer.sol";
import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";
import {RayMathLib} from "../../src/libraries/RayMathLib.sol";
import {PairIterator, Pair, PairIteratorLib} from "./_helpers/PairIterator.sol";
import {TypedDataHasher, TypedDataHasherLib} from "src/types/TypedDataHasher.sol";

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
    UniV4Inspector uniV4;
    PoolGate gate;
    IntrospectiveAngstrom angstrom;

    uint256 constant TOTAL_ASSETS = 4;
    uint256 constant TOTAL_PAIRS = TOTAL_ASSETS * (TOTAL_ASSETS - 1) / 2;
    address[] assets;

    uint256 constant TOTAL_ACTORS = 20;

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
            abi.encodePacked(type(IntrospectiveAngstrom).creationCode, abi.encode(address(uniV4), gov)),
            _angstromFlags(),
            _newFactory()
        );
        assertTrue(succ, "Failed to deploy angstrom");
        angstrom = IntrospectiveAngstrom(addr);

        vm.warp(REAL_TIMESTAMP);
    }

    struct WIPOrder {
        bool zeroToOne;
        uint256 refPrice;
        uint256 amountOut;
        uint256 amountIn;
        bool isFlash;
        uint256 minPrice;
        Trader trader;
        uint256 p;
        uint256 minAmountIn;
        uint256 maxAmountIn;
        uint256 deadline;
    }

    mapping(uint256 => bool) internal usedIndices;

    function test_gas_execute_limit_1() public {
        PRNG memory rng = _rng("exec_1");

        PairIterator memory pairIter =
            PairIteratorLib.init(assets, _randPrices(rng, TOTAL_PAIRS), _randOrderCounts(rng, TOTAL_PAIRS, 40, 75));

        UserOrder[] memory orders = new UserOrder[](pairIter.totalOrders());
        Trader[] memory traders = makeTraders(20);
        OrderMeta memory empty;

        TypedDataHasher typedHasher = TypedDataHasherLib.init(angstrom.DOMAIN_SEPARATOR());

        while (!pairIter.done()) {
            Pair memory pair = pairIter.next();
            for (uint256 i = 0; i < pair.orderCount; i++) {
                uint256 oi = pair.orderOffset + i;
                assertFalse(usedIndices[oi]);
                usedIndices[oi] = true;
                WIPOrder memory r;
                r.zeroToOne = rng.randbool();
                r.refPrice = r.zeroToOne ? pair.price : pair.price.invRay();
                r.amountOut = rng.randuint((r.refPrice / 10).rayToWad(), (r.refPrice * 10).rayToWad());
                r.amountIn = r.amountOut.divRay(r.refPrice);
                r.isFlash = rng.randbool();
                r.minPrice = r.refPrice.mulWad(rng.randuint(0.89e18, 1.0e18));
                r.trader = traders[rng.randuint(traders.length)];
                r.deadline = block.timestamp + 240 + rng.randuint(0, 3600);

                r.p = rng.randuint(1e18);
                if (r.p <= 0.4e18) {
                    r.minAmountIn = r.amountIn.mulWad(rng.randuint(0.2e18, 1.0e18));
                    r.maxAmountIn = r.amountIn.mulWad(rng.randuint(1.0e18, 10.0e18));
                    // Partial order
                    orders[oi] = r.isFlash
                        ? UserOrderLib.from(
                            PartialFlashOrder({
                                minAmountIn: r.minAmountIn,
                                maxAmountIn: r.maxAmountIn,
                                minPrice: r.minPrice,
                                assetIn: r.zeroToOne ? pair.asset0 : pair.asset1,
                                assetOut: r.zeroToOne ? pair.asset1 : pair.asset0,
                                recipient: r.trader.addr,
                                hookData: new bytes(0),
                                validForBlock: u64(block.number),
                                amountFilled: r.amountIn,
                                meta: empty
                            })
                        )
                        : UserOrderLib.from(
                            PartialStandingOrder({
                                minAmountIn: r.minAmountIn,
                                maxAmountIn: r.maxAmountIn,
                                minPrice: r.minPrice,
                                assetIn: r.zeroToOne ? pair.asset0 : pair.asset1,
                                assetOut: r.zeroToOne ? pair.asset1 : pair.asset0,
                                recipient: r.trader.addr,
                                hookData: new bytes(0),
                                nonce: u64(r.trader.getFreshNonce()),
                                deadline: u40(r.deadline),
                                amountFilled: r.amountIn,
                                meta: empty
                            })
                        );
                } else {
                    bool exactIn = r.p <= 0.7e18;
                    // Exact in/out order
                    orders[oi] = r.isFlash
                        ? UserOrderLib.from(
                            ExactFlashOrder({
                                exactIn: exactIn,
                                amount: exactIn ? r.amountIn : r.amountOut,
                                minPrice: r.minPrice,
                                assetIn: r.zeroToOne ? pair.asset0 : pair.asset1,
                                assetOut: r.zeroToOne ? pair.asset1 : pair.asset0,
                                recipient: r.trader.addr,
                                hookData: new bytes(0),
                                validForBlock: u64(block.number),
                                meta: empty
                            })
                        )
                        : UserOrderLib.from(
                            ExactStandingOrder({
                                exactIn: exactIn,
                                amount: exactIn ? r.amountIn : r.amountOut,
                                minPrice: r.minPrice,
                                assetIn: r.zeroToOne ? pair.asset0 : pair.asset1,
                                assetOut: r.zeroToOne ? pair.asset1 : pair.asset0,
                                recipient: r.trader.addr,
                                hookData: new bytes(0),
                                nonce: u64(r.trader.getFreshNonce()),
                                deadline: u40(r.deadline),
                                meta: empty
                            })
                        );
                }
                r.trader.sign(orders[oi], typedHasher);
            }
        }
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
