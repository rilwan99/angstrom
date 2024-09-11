use crate::sol_bindings::sol::{FlashOrder, StandingOrder, TopOfBlockOrder};
use alloy_primitives::{Signature, U256};
use alloy_sol_macro::sol;
use alloy_sol_types::SolStruct;
use alloy_sol_types::{eip712_domain, Eip712Domain};
use serde::{Deserialize, Serialize};

// Direct mapping to
sol! {
    #[derive(Debug, Serialize, Deserialize)]
    struct StandingOrderRequest {
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
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct FlashOrderRequest {
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
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct TopOfBlockOrderRequest {
        uint128 quantityIn;
        uint128 quantityOut;
        bool useInternal;
        address assetIn;
        address assetOut;
        address recipient;
        address hook;
        bytes hookPayload;
        uint64 validForBlock;
    }
}



pub const ANGSTROM_DOMAIN: Eip712Domain = eip712_domain!(
   name: "Angstrom",
   version: "1",
);


#[derive(Serialize, Deserialize)]
pub struct SignedOrder<T> {
    pub order:     T,
    pub signature: Signature
}


impl From<StandingOrderRequest> for StandingOrder {
    fn from(request: StandingOrderRequest) -> Self {
        StandingOrder {
            mode: if request.exactIn { "ExactIn".to_string() } else { "ExactOut".to_string() },
            max_amount_in_or_out: U256::from(request.amount),
            min_price: request.minPrice,
            // this is an address?
            // asset_in: request.assetIn.into(),
            asset_in: 0u16,
            // what here
            asset_in_form: Default::default(),
            // this is an address?
            // asset_out: request.assetOut.into(),
            asset_out: 0u16,
            // what here
            asset_out_form: Default::default(),
            recipient: request.recipient,
            hook_data: request.hookPayload,
            nonce: request.nonce,
            deadline: U256::from(request.deadline),
            signature: Default::default(),
        }
    }
}

impl From<FlashOrderRequest> for FlashOrder {
    fn from(request: FlashOrderRequest) -> Self {
        FlashOrder {
            mode: if request.exactIn { "ExactIn".to_string() } else { "ExactOut".to_string() },
            max_amount_in_or_out: U256::from(request.amount),
            min_price: request.minPrice,
            // this is an address?
            // asset_in: request.assetIn.into(),
            asset_in: 0u16,
            // what here
            asset_in_form: Default::default(), // Assuming Liquid as default
            // this is an address?
            // asset_out: request.assetOut.into(),
            asset_out: 0u16,
            // what here
            asset_out_form: Default::default(), // Assuming Liquid as default
            recipient: request.recipient,
            hook_data: request.hookPayload,
            valid_for_block: request.validForBlock,
            signature: Default::default(),
        }
    }
}

impl From<TopOfBlockOrderRequest> for TopOfBlockOrder {
    fn from(request: TopOfBlockOrderRequest) -> Self {
        TopOfBlockOrder {
            amountIn: U256::from(request.quantityIn),
            amountOut: U256::from(request.quantityOut),
            // this is an address?
            // assetInIndex: request.assetIn.into(),
            assetInIndex: 0u16,
            assetInForm: Default::default(),
            // this is an address?
            // assetOutIndex: request.assetOut.into(),
            assetOutIndex: 0u16,
            assetOutForm: Default::default(),
            recipient: request.recipient,
            hook: request.hook,
            hookPayload: request.hookPayload,
            // fill a bit later
            from: Default::default(),
            signature: Default::default(),
        }
    }
}
