use alloy_sol_macro::sol;
use serde::{Deserialize, Serialize};

sol! {
    #[derive(Debug, Default, PartialEq, Eq, Hash,Serialize, Deserialize)]
    struct OrderMeta {
        bool isEcdsa;
        address from;
        bytes signature;
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    struct PartialStandingOrder {
        uint128 minAmountIn;
        uint128 maxAmountIn;
        uint256 minPrice;
        bool useInternal;
        address assetIn;
        address assetOut;
        address recipient;
        address hook;
        bytes hookPayload;
        uint64 nonce;
        uint40 deadline;
        uint128 amountFilled;
        OrderMeta meta;
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    struct ExactStandingOrder {
        bool exactIn;
        uint128 amount;
        uint256 minPrice;
        bool useInternal;
        address assetIn;
        address assetOut;
        address recipient;
        address hook;
        bytes hookPayload;
        uint64 nonce;
        uint40 deadline;
        OrderMeta meta;
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    struct PartialFlashOrder {
        uint128 minAmountIn;
        uint128 maxAmountIn;
        uint256 minPrice;
        bool useInternal;
        address assetIn;
        address assetOut;
        address recipient;
        address hook;
        bytes hookPayload;
        uint64 validForBlock;
        uint128 amountFilled;
        OrderMeta meta;
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    struct ExactFlashOrder {
        bool exactIn;
        uint128 amount;
        uint256 minPrice;
        bool useInternal;
        address assetIn;
        address assetOut;
        address recipient;
        address hook;
        bytes hookPayload;
        uint64 validForBlock;
        OrderMeta meta;
    }

    #[derive(Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    struct TopOfBlockOrder {
        uint128 quantityIn;
        uint128 quantityOut;
        bool useInternal;
        address assetIn;
        address assetOut;
        address recipient;
        address hook;
        bytes hookPayload;
        uint64 validForBlock;
        OrderMeta meta;
    }
}
