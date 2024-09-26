use pade::{bitvec::field::BitField, PadeEncode};
use pade_macro::{PadeDecode, PadeEncode};

#[test]
fn can_derive_on_struct() {
    #[derive(PadeEncode, PadeDecode)]
    struct Test {
        index:  u128,
        vector: Vec<u128>
    }

    let test_struct = Test { index: 12345, vector: vec![123, 234, 345] };
    test_struct.pade_encode();
}

#[test]
fn can_derive_on_unnamed_struct() {
    #[derive(PadeEncode, PadeDecode)]
    struct UnNamed(u128, u128);

    let test_struct = UnNamed(12345, 23456);
    test_struct.pade_encode();
}

#[test]
fn can_derive_on_nested_struct() {
    #[derive(PadeEncode, PadeDecode)]
    struct Inside {
        number:  u128,
        another: u128
    }

    #[derive(PadeEncode, PadeDecode)]
    struct Outside {
        inner: Inside
    }

    let test_struct = Outside { inner: Inside { number: 12345, another: 23456 } };
    test_struct.pade_encode();
}

#[test]
fn can_derive_on_basic_enum() {
    #[derive(PadeEncode, PadeDecode)]
    enum BasicEnum {
        StateA,
        StateB,
        StateC
    }

    let test_enum = BasicEnum::StateB;
    test_enum.pade_encode();
}

#[test]
fn can_derive_on_enums_with_data() {
    #[derive(PadeEncode, PadeDecode)]
    enum DataEnum {
        Point(u128),
        Line(u128, u128, u128)
    }

    let test_enum = DataEnum::Line(12345, 23456, 34567);
    test_enum.pade_encode();
}

#[test]
fn can_derive_on_structlike_enums() {
    #[derive(PadeEncode, PadeDecode)]
    enum StructEnum {
        Once { x: u128, y: u128 },
        Twice { a: u128, b: u128 }
    }

    let struct_enum = StructEnum::Twice { a: 12345, b: 23456 };
    struct_enum.pade_encode();
}

#[test]
fn will_respect_item_width() {
    #[derive(PadeEncode, PadeDecode)]
    struct TooBig {
        #[pade_width(3)]
        x: i32
    }

    let test_struct = TooBig { x: 123 };
    let output = test_struct.pade_encode();
    assert_eq!(output.len(), 3, "Didn't respect byte limit");
}
