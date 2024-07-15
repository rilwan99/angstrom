mod private {
    use alloy_sol_macro::sol;

    sol! {
        #[derive(Debug, Default, PartialEq, Eq)]
        struct PartialStandingOrder {
            uint256 min_amount_in;
            uint256 max_amount_in;
            uint256 min_price;
            address asset_in;
            address asset_out;
            address recipient;
            bytes hook_data;
            uint64 nonce;
            uint40 deadline;
        }

        #[derive(Debug, Default, PartialEq, Eq)]
        struct ExactStandingOrder {
            bool exact_in;
            uint256 amount;
            uint256 min_price;
            address asset_in;
            address asset_out;
            address recipient;
            bytes hook_data;
            uint64 nonce;
            uint40 deadline;
        }

        #[derive(Debug, Default, PartialEq, Eq)]
        struct PartialFlashOrder {
            uint256 min_amount_in;
            uint256 max_amount_in;
            uint256 min_price;
            address asset_in;
            address asset_out;
            address recipient;
            bytes hook_data;
            uint64 valid_for_block;
        }

        #[derive(Debug, Default, PartialEq, Eq)]
        struct ExactFlashOrder {
            bool exact_in;
            uint256 amount;
            uint256 min_price;
            address asset_in;
            address asset_out;
            address recipient;
            bytes hook_data;
            uint64 valid_for_block;
        }

        #[derive(Debug, Default, PartialEq, Eq)]
        struct TopOfBlockOrder {
            uint256 amount_in;
            uint256 amount_out;
            address asset_in;
            address asset_out;
            address recipient;
            bytes hook_data;
            uint64 valid_for_block;
        }

        #[derive(Debug, Default, PartialEq, Eq)]
        type AssetIndex is uint16;

        #[derive(Debug, Default, PartialEq, Eq)]
        struct TopOfBlockOrderEnvelope {
            uint256 amountIn;
            uint256 amountOut;
            AssetIndex assetInIndex;
            AssetIndex assetOutIndex;
            address recipient;
            address hook;
            bytes hookPayload;
            address from;
            bytes signature;
        }

        #[derive(Debug, Default, PartialEq, Eq)]
        struct Price {
            AssetIndex outIndex;
            AssetIndex inIndex;
            uint256 price;
        }

        #[derive(Debug, Default, PartialEq, Eq)]
        struct Swap {
            AssetIndex asset0Index;
            AssetIndex asset1Index;
            bool zeroForOne;
            uint256 amountIn;
        }

        #[derive(Debug, Default, PartialEq, Eq)]
        struct Donate {
            AssetIndex asset0Index;
            AssetIndex asset1Index;
            int24 startTick;
            uint256 totalTicks;
            uint128 startLiquidity;
            uint256[] amounts0;
        }

        #[derive(Debug, Default, PartialEq, Eq)]
        struct ContractBundle {
            address[] assets;
            Price[] initial_prices;
            bytes[] pre_transformations;
            TopOfBlockOrderEnvelope[] top_of_block_orders;
            Swap[] swaps;
            GenericOrder[] orders;
            bytes[] post_transformations;
            Donate[] donates;
        }

        #[derive(Debug)]
        contract AngstromContract {
            function execute(bytes calldata data) external;
        }
    }
}

pub use private::{
    AngstromContract, ContractBundle, Donate as SolDonate,
    PartialFlashOrder as SolPartialFlashOrder, PartialStandingOrder as SolPartialStandingOrder,
    Price as SolPrice, Swap as SolSwap, TopOfBlockOrder as SolTopOfBlockOrder,
    TopOfBlockOrderEnvelope as SolTopOfBlockOrderEnvelope
};

#[derive(Default, Debug, Clone)]
pub struct InvalidSolEnumVariant;

impl std::error::Error for InvalidSolEnumVariant {}

impl std::fmt::Display for InvalidSolEnumVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidSolEnumVariant")
    }
}

#[cfg(test)]
mod test {
    use alloy_sol_types::{SolStruct, SolType};

    use super::*;

    #[test]
    fn test_get_eip712_names() {
        println!("TopOfBlockOrder::NAME: {}", SolTopOfBlockOrder::NAME);
        println!("TopOfBlockOrder::encode_type: {}", SolTopOfBlockOrder::eip712_encode_type());
        println!("ToB::SOL_NAME: {}", SolTopOfBlockOrder::SOL_NAME)
    }
}
