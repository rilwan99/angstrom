#[cfg(not(feature = "testnet"))]
use alloy::primitives::Address;
#[cfg(feature = "testnet")]
use alloy::primitives::{Address, U256};
use alloy::{primitives::B256, sol_types::SolStruct};
#[cfg(feature = "testnet")]
use rand::{rngs::ThreadRng, Rng};

#[cfg(feature = "testnet")]
use crate::sol_bindings::sol::{SolGenericOrder, TopOfBlockOrder};
use crate::{
    primitive::OrderType,
    sol_bindings::sol::{ContractBundle, SolAssetForm, SolOrderMode, SolOrderType}
};

impl ContractBundle {
    pub fn get_filled_hashes(&self) -> Vec<B256> {
        self.top_of_block_orders
            .iter()
            .map(|order| order.eip712_hash_struct())
            .chain(self.orders.iter().map(|order| order.eip712_hash_struct()))
            .collect()
    }

    pub fn get_addresses_touched(&self) -> Vec<Address> {
        self.top_of_block_orders
            .iter()
            .map(|order| order.from)
            .chain(self.orders.iter().map(|order| order.from))
            .collect()
    }
}

#[cfg(feature = "testnet")]
impl ContractBundle {
    pub fn generate_random_bundles(order_count: u64) -> Self {
        let mut rng = rand::thread_rng();

        let assets = vec![Address::random()];

        let mut tob = TopOfBlockOrder::default();
        let rand_am_in: U256 = U256::from_be_bytes(rng.gen::<[u8; 32]>());
        let rand_am_out: U256 = U256::from_be_bytes(rng.gen::<[u8; 32]>());
        tob.amountIn = rand_am_in;
        tob.amountOut = rand_am_out;
        let mut generic_orders: Vec<HelperStruct> = Vec::with_capacity(order_count as usize);
        for _ in 0..order_count {
            let mut default = SolGenericOrder::default();

            let specified: U256 = U256::from_be_bytes(rng.gen::<[u8; 32]>());
            default.amountSpecified = specified;
            generic_orders.push(default.into());
        }

        Self {
            assets,
            top_of_block_orders: vec![tob],
            orders: generic_orders.into_iter().map(|val| val.into()).collect(),
            ..Default::default()
        }
    }
}

struct HelperStruct {
    otype:           HelperOrderType,
    mode:            HelperOrderMode,
    amountSpecified: U256,
    minPrice:        U256,
    assetInIndex:    u16,
    assetInForm:     HelperAssetForm,
    assetOutIndex:   u16,
    assetOutForm:    HelperAssetForm,
    nonce:           u64,
    deadline:        U256,
    recipient:       Address,
    hook:            Address,
    hookPayload:     reth_primitives::Bytes,
    amountFilled:    U256,
    from:            Address,
    signature:       reth_primitives::Bytes
}

impl Into<SolGenericOrder> for HelperStruct {
    fn into(self) -> SolGenericOrder {
        let HelperStruct {
            otype,
            mode,
            amountSpecified,
            minPrice,
            assetInIndex,
            assetInForm,
            assetOutIndex,
            assetOutForm,
            nonce,
            deadline,
            recipient,
            hook,
            hookPayload,
            amountFilled,
            from,
            signature
        } = self;

        SolGenericOrder {
            otype: otype.into(),
            mode: mode.into(),
            amountSpecified,
            minPrice,
            assetInIndex,
            assetInForm: assetInForm.into(),
            assetOutIndex,
            assetOutForm: assetOutForm.into(),
            nonce,
            deadline,
            recipient,
            hook,
            hookPayload,
            amountFilled,
            from,
            signature
        }
    }
}

impl From<SolGenericOrder> for HelperStruct {
    fn from(value: SolGenericOrder) -> Self {
        let SolGenericOrder {
            otype,
            mode,
            amountSpecified,
            minPrice,
            assetInIndex,
            assetInForm,
            assetOutIndex,
            assetOutForm,
            nonce,
            deadline,
            recipient,
            hook,
            hookPayload,
            amountFilled,
            from,
            signature
        } = value;

        Self {
            otype: otype.into(),
            mode: mode.into(),
            amountSpecified,
            minPrice,
            assetInIndex,
            assetInForm: assetInForm.into(),
            assetOutIndex,
            assetOutForm: assetOutForm.into(),
            nonce,
            deadline,
            recipient,
            hook,
            hookPayload,
            amountFilled,
            from,
            signature
        }
    }
}

#[derive(Default)]
enum HelperOrderType {
    Flash,
    #[default]
    Standing,
    __Invalid
}

impl From<SolOrderType> for HelperOrderType {
    fn from(value: SolOrderType) -> Self {
        match value {
            SolOrderType::Flash => Self::Flash,
            SolOrderType::Standing => Self::Standing,
            SolOrderType::__Invalid => Self::__Invalid
        }
    }
}

impl Into<SolOrderType> for HelperOrderType {
    fn into(self) -> SolOrderType {
        match self {
            Self::Flash => SolOrderType::Flash,
            Self::Standing => SolOrderType::Standing,
            Self::__Invalid => SolOrderType::__Invalid
        }
    }
}

pub enum HelperOrderMode {
    ExactIn,
    ExactOut,
    Partial,
    __Invalid
}

impl From<crate::sol_bindings::sol::OrderMode> for HelperOrderMode {
    fn from(value: crate::sol_bindings::sol::OrderMode) -> Self {
        match value {
            crate::sol_bindings::sol::OrderMode::ExactIn => Self::ExactIn,
            crate::sol_bindings::sol::OrderMode::ExactOut => Self::ExactOut,
            crate::sol_bindings::sol::OrderMode::Partial => Self::Partial,
            crate::sol_bindings::sol::OrderMode::__Invalid => Self::__Invalid
        }
    }
}

impl Into<crate::sol_bindings::sol::OrderMode> for HelperOrderMode {
    fn into(self) -> crate::sol_bindings::sol::OrderMode {
        match self {
            Self::ExactIn => crate::sol_bindings::sol::OrderMode::ExactIn,
            Self::ExactOut => crate::sol_bindings::sol::OrderMode::ExactOut,
            Self::Partial => crate::sol_bindings::sol::OrderMode::Partial,
            Self::__Invalid => crate::sol_bindings::sol::OrderMode::__Invalid
        }
    }
}

pub enum HelperAssetForm {
    Liquid,
    UniV4Claim,
    AngstromClaim,
    __Invalid
}

impl From<crate::sol_bindings::sol::AssetForm> for HelperAssetForm {
    fn from(value: crate::sol_bindings::sol::AssetForm) -> Self {
        match value {
            crate::sol_bindings::sol::AssetForm::Liquid => Self::Liquid,
            crate::sol_bindings::sol::AssetForm::UniV4Claim => Self::UniV4Claim,
            crate::sol_bindings::sol::AssetForm::AngstromClaim => Self::AngstromClaim,
            crate::sol_bindings::sol::AssetForm::__Invalid => Self::__Invalid
        }
    }
}

impl Into<crate::sol_bindings::sol::AssetForm> for HelperAssetForm {
    fn into(self) -> crate::sol_bindings::sol::AssetForm {
        match self {
            Self::Liquid => crate::sol_bindings::sol::AssetForm::Liquid,
            Self::UniV4Claim => crate::sol_bindings::sol::AssetForm::UniV4Claim,
            Self::AngstromClaim => crate::sol_bindings::sol::AssetForm::AngstromClaim,
            Self::__Invalid => crate::sol_bindings::sol::AssetForm::__Invalid
        }
    }
}

#[cfg(test)]
pub mod test {

    #[test]
    #[cfg(feature = "testnet")]
    pub fn test_contract_bundle_encode_decode() {
        use alloy::sol_types::SolValue;

        use crate::sol_bindings::sol::ContractBundle;

        let rand = ContractBundle::generate_random_bundles(10);
        let encoded = rand.abi_encode();
        let decoded = ContractBundle::abi_decode(&encoded, false).unwrap();
        assert_eq!(rand, decoded);
    }
}
