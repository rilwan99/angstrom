use pade_macro::PadeEncode;

#[test]
fn can_derive_on_struct() {
    #[derive(PadeEncode)]
    struct Test {
        index:  u128,
        vector: Vec<u128>
    }
}

#[test]
fn can_derive_on_unnamed_struct() {
    #[derive(PadeEncode)]
    struct UnNamed(u128, u128);
}

#[test]
fn can_derive_on_nested_struct() {
    #[derive(PadeEncode)]
    struct Inside {
        number:  u128,
        another: u128
    }

    #[derive(PadeEncode)]
    struct Outside {
        inner: Inside
    }
}

#[test]
fn can_derive_on_basic_enum() {
    #[derive(PadeEncode)]
    enum BasicEnum {
        StateA,
        StateB,
        StateC
    }
}

#[test]
fn can_derive_on_enums_with_data() {
    #[derive(PadeEncode)]
    enum DataEnum {
        Point(u128),
        Line(u128, u128, u128)
    }
}

#[test]
fn can_derive_on_structlike_enums() {
    #[derive(PadeEncode)]
    enum StructEnum {
        Once { x: u128, y: u128 },
        Twice { a: u128, b: u128 }
    }
}
