#![feature(iterator_try_collect)]
#![warn(unreachable_pub)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
#![allow(clippy::wrong_self_convention)]

use syn::{parse_macro_input, DeriveInput};

mod randomizer;

#[allow(unused_extern_crates)]
extern crate proc_macro;

#[proc_macro_derive(RandomizerWith)]
pub fn pade_encode_fn(raw: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(raw as DeriveInput);
    randomizer::derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
