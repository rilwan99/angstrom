use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::sol_bindings::{
    grouped_orders::{AllOrders, FlashVariants, StandingVariants},
    rpc_orders::{
        ExactFlashOrder, ExactStandingOrder, OrderMeta, PartialFlashOrder, PartialStandingOrder,
        TopOfBlockOrder
    },
    testnet::random::RandomizerSized
};

impl Distribution<AllOrders> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AllOrders {
        let rand_variant = rng.gen_range(0..3);

        match rand_variant {
            0 => AllOrders::Flash(rng.gen()),
            1 => AllOrders::Standing(rng.gen()),
            2 => AllOrders::TOB(rng.gen()),
            _ => unreachable!()
        }
    }
}

impl Distribution<FlashVariants> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FlashVariants {
        let rand_variant = rng.gen_range(0..2);

        match rand_variant {
            0 => FlashVariants::Exact(rng.gen()),
            1 => FlashVariants::Partial(rng.gen()),
            _ => unreachable!()
        }
    }
}

impl Distribution<ExactFlashOrder> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ExactFlashOrder {
        ExactFlashOrder {
            exactIn:       rng.gen(),
            amount:        rng.gen(),
            minPrice:      rng.gen(),
            useInternal:   rng.gen(),
            assetIn:       rng.gen(),
            assetOut:      rng.gen(),
            recipient:     rng.gen(),
            hook:          rng.gen(),
            hookPayload:   rng.gen_sized::<150>(),
            validForBlock: rng.gen(),
            meta:          rng.gen()
        }
    }
}

impl Distribution<PartialFlashOrder> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PartialFlashOrder {
        PartialFlashOrder {
            minPrice:      rng.gen(),
            useInternal:   rng.gen(),
            assetIn:       rng.gen(),
            assetOut:      rng.gen(),
            recipient:     rng.gen(),
            hook:          rng.gen(),
            hookPayload:   rng.gen_sized::<150>(),
            validForBlock: rng.gen(),
            meta:          rng.gen(),
            minAmountIn:   rng.gen(),
            maxAmountIn:   rng.gen(),
            amountFilled:  rng.gen()
        }
    }
}

impl Distribution<StandingVariants> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StandingVariants {
        let rand_variant = rng.gen_range(0..2);

        match rand_variant {
            0 => StandingVariants::Exact(rng.gen()),
            1 => StandingVariants::Partial(rng.gen()),
            _ => unreachable!()
        }
    }
}

impl Distribution<ExactStandingOrder> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ExactStandingOrder {
        ExactStandingOrder {
            exactIn:     rng.gen(),
            amount:      rng.gen(),
            minPrice:    rng.gen(),
            useInternal: rng.gen(),
            assetIn:     rng.gen(),
            assetOut:    rng.gen(),
            recipient:   rng.gen(),
            hook:        rng.gen(),
            hookPayload: rng.gen_sized::<150>(),
            meta:        rng.gen(),
            nonce:       rng.gen(),
            deadline:    rng.gen()
        }
    }
}

impl Distribution<PartialStandingOrder> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PartialStandingOrder {
        PartialStandingOrder {
            minPrice:     rng.gen(),
            useInternal:  rng.gen(),
            assetIn:      rng.gen(),
            assetOut:     rng.gen(),
            recipient:    rng.gen(),
            hook:         rng.gen(),
            hookPayload:  rng.gen_sized::<150>(),
            meta:         rng.gen(),
            minAmountIn:  rng.gen(),
            maxAmountIn:  rng.gen(),
            amountFilled: rng.gen(),
            nonce:        rng.gen(),
            deadline:     rng.gen()
        }
    }
}

impl Distribution<TopOfBlockOrder> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TopOfBlockOrder {
        TopOfBlockOrder {
            recipient:     rng.gen(),
            quantityIn:    rng.gen(),
            quantityOut:   rng.gen(),
            useInternal:   rng.gen(),
            assetIn:       rng.gen(),
            assetOut:      rng.gen(),
            hook:          rng.gen(),
            hookPayload:   rng.gen_sized::<150>(),
            validForBlock: rng.gen(),
            meta:          rng.gen()
        }
    }
}

impl Distribution<OrderMeta> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> OrderMeta {
        OrderMeta { isEcdsa: rng.gen(), from: rng.gen(), signature: rng.gen_sized::<64>() }
    }
}

/*

impl Distribution<ContractBundle> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ContractBundle {
        let vec_sizes = rng.gen_range(0..=10);
        ContractBundle {
            assets:               rng.gen_many(vec_sizes),
            initial_prices:       rng.gen_many(vec_sizes),
            pre_transformations:  rng.gen_many_sized::<0>(vec_sizes),
            top_of_block_orders:  rng.gen_many(vec_sizes),
            swaps:                rng.gen_many(vec_sizes),
            orders:               rng.gen_many(vec_sizes),
            post_transformations: rng.gen_many(vec_sizes),
            donates:              rng.gen_many(vec_sizes)
        }
    }
}

*/

// impl Distribution<SolPrice> for Standard {
//     fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SolPrice {
//         SolPrice { outIndex: rng.gen(), inIndex: rng.gen(), price: rng.gen()
// }     }
// }

#[cfg(test)]
mod tests {
    use crate::sol_bindings::grouped_orders::{random_AllOrders, AllOrders};

    #[test]
    fn test_all_orders() {
        let all_orders = random_AllOrders!(TOB);
        assert!(matches!(all_orders, AllOrders::TOB(_)));
    }
}
