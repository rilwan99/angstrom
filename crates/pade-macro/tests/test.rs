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
