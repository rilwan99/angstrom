use pade_macro::{PadeDecode, PadeEncode};
use alloy::primitives::Address;


#[test]
fn can_derive_on_sol_struct() {
    #[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
    struct Test {
        index:  Address,
        t0_idx: u16,
    }

    alloy::sol!{
        #[derive(PadeEncode, PadeDecode)]
        struct AlloyTest {
            address index;
            uint16 t0_idx;
        }
    }
}
