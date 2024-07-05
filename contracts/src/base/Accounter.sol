// SPDX-License-Identifier: BUSL-1.1
pragma solidity 0.8.24;

import {SafeCastLib} from "solady/src/utils/SafeCastLib.sol";
import {Currency} from "v4-core/src/types/Currency.sol";
import {SafeTransferLib} from "solady/src/utils/SafeTransferLib.sol";
import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";
import {SilentERC6909} from "./SilentERC6909.sol";
import {tuint256, tint256} from "transient-goodies/TransientPrimitives.sol";
import {AssetForm} from "../interfaces/OrderTypes.sol";
import {DumbDispatch} from "./DumbDispatch.sol";
import {ConversionLib} from "../libraries/ConversionLib.sol";
import {Globals} from "../libraries/Globals.sol";

import {IPoolManager, IUniV4} from "../interfaces/IUniV4.sol";
import {AssetIndex} from "../libraries/PriceGraph.sol";
import {MixedSignLib} from "../libraries/MixedSignLib.sol";
import {PoolKey} from "v4-core/src/types/PoolKey.sol";
import {PoolId, PoolIdLibrary} from "v4-core/src/types/PoolId.sol";

// TODO: Remove debug.
import {console2 as console} from "forge-std/console2.sol";
import {FormatLib} from "../libraries/FormatLib.sol";

/// @author philogy <https://github.com/philogy>
abstract contract Accounter is SilentERC6909, DumbDispatch {
    // TODO: Remove debug.
    using FormatLib for *;

    using SafeCastLib for uint256;
    using SafeTransferLib for address;
    using ConversionLib for *;
    using FixedPointMathLib for uint256;
    using PoolIdLibrary for PoolKey;
    using MixedSignLib for uint256;
    using IUniV4 for IPoolManager;

    error NotUniswap();

    IPoolManager internal immutable UNI_V4;

    mapping(address => uint256) internal _savedFees;
    mapping(PoolId id => mapping(int24 tick => uint256 cumulativeFeeGrowth)) internal feeGrowthOutside;
    mapping(PoolId id => uint256 cumulativeFeeGrowth) internal feeGrowthGlobal;

    mapping(address => mapping(address => tint256)) internal netLiquid;
    mapping(address => mapping(address => tint256)) internal netV4Claims;
    tuint256 internal unresolvedChanges;
    /// @dev Amount of available liquid assets within the angstrom contract.
    mapping(address => tuint256) internal freeBalance;

    constructor(address uniV4) {
        UNI_V4 = IPoolManager(uniV4);
    }

    modifier onlyUniV4() {
        if (msg.sender != address(UNI_V4)) revert NotUniswap();
        _;
    }

    ////////////////////////////////////////////////////////////////
    //                    EXECUTION ACCOUNTING                    //
    ////////////////////////////////////////////////////////////////

    function _accountIn(address from, AssetForm atype, address asset, uint256 amount) internal {
        if (atype == AssetForm.AngstromClaim) {
            _burn(from, asset.into(), amount);
            // Don't need to update unresolved as it doesn't support negatives and is overflow
            // checking, worst case unused balance is lost.
            freeBalance[asset].inc(amount);
            return;
        }
        if (atype == AssetForm.UniV4Claim) {
            _accountChange(netV4Claims[from][asset], amount.neg());
            return;
        }
        if (atype == AssetForm.Liquid) {
            _accountChange(netLiquid[from][asset], amount.neg());
            return;
        }
        // Should be unreachable (<3 to Solidity for not having match statements).
        assert(false);
    }

    function _accountOut(address to, AssetForm atype, address asset, uint256 amount) internal {
        if (atype == AssetForm.AngstromClaim) {
            _mint(to, asset.into(), amount);
            // Don't need to update unresolved as it doesn't support negatives and is overflow
            // checking, worst case unused balance is lost.
            // TODO: Allow negative and do "is resolved checking"?
            freeBalance[asset].dec(amount);
            return;
        }
        if (atype == AssetForm.UniV4Claim) {
            _accountChange(netV4Claims[to][asset], amount.signed());
            return;
        }
        if (atype == AssetForm.Liquid) {
            _accountChange(netLiquid[to][asset], amount.signed());
            return;
        }
        // Should be unreachable (<3 to Solidity for not having match statements).
        assert(false);
    }

    struct Donate {
        AssetIndex asset0Index;
        AssetIndex asset1Index;
        int24 startTick;
        uint128 startLiquidity;
        uint256[] amounts0;
    }

    function _donate(Globals memory g, Donate memory donate) internal {
        address asset0 = g.get(donate.asset0Index);
        address asset1 = g.get(donate.asset1Index);
        PoolId poolid = ConversionLib.toPoolKey(address(this), asset0, asset1).toId();
        int24 currentTick = UNI_V4.getSlot0(poolid).tick();

        uint256 totalDonated = 0;
        uint256 cumulativeFeeGrowth = 0;
        uint256 liquidity = donate.startLiquidity;

        int24 tick = donate.startTick;

        if (tick <= currentTick) {} else {}

        require(liquidity == UNI_V4.getPoolLiquidity(poolid), "WRONG_LIQUIDITY");

        feeGrowthGlobal[poolid] += cumulativeFeeGrowth;
        freeBalance[asset0].dec(totalDonated);
    }

    /**
     * @dev Positive change indicates assets the end-user is owed vs. negative change indicates
     * assets the users owes to the protocol.
     */
    function _accountChange(tint256 storage balance, int256 change) internal returns (uint256 countChange) {
        int256 preBal = balance.get();
        int256 newBal = preBal + change;
        balance.set(newBal);
        unchecked {
            countChange = (newBal != 0).into() - (preBal != 0).into();
            if (countChange != 0) {
                uint256 prevCount = unresolvedChanges.get();
                unresolvedChanges.set(prevCount + countChange);
            }
        }
    }
}
