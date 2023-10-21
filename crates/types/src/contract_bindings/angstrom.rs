use alloy_sol_macro::sol;
use alloy_rlp_derive::{RlpEncodable, RlpDecodable};
use serde::{Deserialize, Serialize};
use alloy_rlp::{Decodable, Encodable, Error};
use crate::contract_bindings::Angstrom::OrderType;
use alloy_primitives::U256;

sol!{
    #![sol(all_derives = true)]
    
    interface Angstrom {
        event OwnershipHandoverCanceled(address indexed pendingOwner);
        event OwnershipHandoverRequested(address indexed pendingOwner);
        event OwnershipTransferred(address indexed oldOwner, address indexed newOwner);

        type Currency is address;

        #[derive(RlpDecodable, RlpEncodable)]
        struct Bundle {
            ExecutedOrder[] orders;
            bytes uniswapData;
        }
        #[derive(RlpDecodable, RlpEncodable)]
        struct ExecutedOrder {
            Order order;
            bytes signature;
            uint256 amountInActual;
            uint256 amountOutActual;
        }

        #[derive(RlpDecodable, RlpEncodable)]
        struct SwapParams {
            bool zeroForOne;
            int256 amountSpecified;
            uint160 sqrtPriceLimitX96;
        }

        #[derive(RlpDecodable, RlpEncodable)]
        struct Order {
            uint256 nonce;
            uint8 orderType;
            address currencyIn;
            address currencyOut;
            uint128 amountIn;
            uint128 amountOutMin;
            uint256 deadline;
            bytes preHook;
            bytes postHook;
        }

        enum OrderType {
            User,
            Searcher,
            Limit,
            UserFallible,
            SearcherFallible
        }

        #[derive(RlpDecodable, RlpEncodable)]
        struct PoolKey {
            address currency0;
            address currency1;
            uint24 fee;
            int24 tickSpacing;
            address hooks;
        }

        #[derive(RlpDecodable, RlpEncodable)]
        /// @notice Uniswap instructions to execute after lock is taken.
        struct UniswapData {
            /// @member The discrete swaps to perform, there should be at most one entry
            ///         per pool.
            PoolSwap[] swaps;
            /// @member The currency settlements to perform, there should be at most one
            ///         entry per currency.
            CurrencySettlement[] currencies;
            /// @member The fees to pay to each pool, there should be at most one entry
            ///         per pool.
            PoolFees[] pools;
        }

        #[derive(RlpDecodable, RlpEncodable)]
        /// @notice Instruction to execute a swap on UniswapV4.
        struct PoolSwap {
            /// @member The pool to perform the swap on.
            PoolKey pool;
            /// @member The input currency.
            Currency currencyIn;
            /// @member The amount of input.
            uint256 amountIn;
        }

        #[derive(RlpDecodable, RlpEncodable)]
        /// @notice Instruction to settle an amount of currency.
        struct CurrencySettlement {
            /// @member The currency to settle.
            Currency currency;
            /// @member The amount to settle, positive indicates we must pay, negative
            ///         indicates we are paid.
            int256 amountNet;
        }

        #[derive(RlpDecodable, RlpEncodable)]
        /// @notice Instruction to donate revenue to a pool.
        struct PoolFees {
            /// @member The pool to pay fees to.
            PoolKey pool;
            /// @member The amount0 fee.
            uint256 fees0;
            /// @member The amount1 fee.
            uint256 fees1;
        }


        function beforeSwap(address aSender, SwapParams memory, SwapParams memory, bytes memory)
            external
            view
            returns (bytes4 rSelector);
        function cancelOwnershipHandover() external payable;
        function claimFees(address aCurrency, address aRecipient) external;
        function completeOwnershipHandover(address pendingOwner) external payable;
        function eip712Domain()
            external
            view
            returns (
                bytes1 fields,
                string memory name,
                string memory version,
                uint256 chainId,
                address verifyingContract,
                bytes32 salt,
                uint256[] memory extensions
            );
        function invalidateUnorderedNonces(uint256 wordPos, uint256 mask) external;
        function lockAcquired(bytes memory aBundle) external returns (bytes memory);
        function nonceBitmap(address, uint256) external view returns (uint256);
        function owner() external view returns (address result);
        function ownershipHandoverExpiresAt(address pendingOwner) external view returns (uint256 result);
        function poolManager() external view returns (address);
        function process(Bundle memory aBundle) external;
        function renounceOwnership() external payable;
        function requestOwnershipHandover() external payable;
        function transferOwnership(address newOwner) external payable;
    }
}


impl Encodable for OrderType {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        let byte: u8 = unsafe { std::mem::transmute(*self) };
        out.put_u8(byte)
    }
}
impl Decodable for OrderType {
    fn decode(buf: &mut &[u8]) -> Result<Self, Error> {
        unsafe { std::mem::transmute(u8::decode(buf)) }
    }
}

