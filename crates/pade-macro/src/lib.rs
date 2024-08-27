use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Index};

#[proc_macro_derive(PadeEncode)]
pub fn pade_encode_fn(raw: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(raw as DeriveInput);

    let name = input.ident;
    let (impl_gen, ty_gen, where_clause) = input.generics.split_for_impl();
    let field_encoders = build_field_encoders(&input.data);
    let expanded = quote! {
        impl #impl_gen pade::PadeEncode for #name #ty_gen #where_clause {
            fn pade_encode(&self) -> Vec<u8> {
                #field_encoders
            }
        }
    };
    proc_macro::TokenStream::from(expanded)
}

fn build_field_encoders(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref s) => match s.fields {
            Fields::Unit => unimplemented!(),
            Fields::Named(ref fields) => {
                let encoders = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {f.span()=>
                        pade::PadeEncode::pade_encode(&self.#name)
                    }
                });
                quote! {
                    [#(#encoders),*].concat()
                }
            }
            Fields::Unnamed(ref fields) => {
                let encoders = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let index = Index::from(i);
                    quote_spanned! {f.span()=>
                        pade::PadeEncode::pade_encode(&self.#index)
                    }
                });
                quote! {
                    [#(#encoders),*].concat()
                }
            }
        },
        _ => unimplemented!()
    }
}
