use pade::{bitvec::field::BitField, PadeDecode, PadeEncode};
use pade_macro::{PadeDecode, PadeEncode};

#[test]
fn supports_struct_with_enum() {
    #[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
    struct OuterStruct {
        #[pade_width(3)]
        x:      i32,
        enum1:  Cases,
        list:   Vec<u128>,
        inside: Inside
    }

    #[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
    struct Inside {
        number:  u128,
        another: u128,
        enum1:   Cases
    }

    #[derive(PadeEncode, PadeDecode, PartialEq, Eq, Debug)]
    pub enum Cases {
        Once { x: u128, y: u128 },
        Twice { a: u128, b: u128 },
        // memes
        Thrice { a: u128, b: u128 }
    }

    let outer = OuterStruct {
        x:      34342,
        enum1:  Cases::Twice { a: 10, b: 2000000 },
        list:   vec![1, 2, 3, 4023, 323424],
        inside: Inside {
            enum1:   Cases::Thrice { a: 123, b: 423 },
            number:  234093323,
            another: 234234
        }
    };

    let encoded = outer.pade_encode();
    let mut slice = encoded.as_slice();
    let decoded = OuterStruct::pade_decode(&mut slice, None).unwrap();

    assert_eq!(outer, decoded);
}
