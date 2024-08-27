use proc_macro2::{Literal, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, Field, Fields, Generics,
    Ident, Index, Type
};

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

fn build_struct_impl(name: &Ident, generics: &Generics, s: &DataStruct) -> TokenStream {
    let (impl_gen, ty_gen, where_clause) = generics.split_for_impl();

    let field_encoders = match s.fields {
        Fields::Unit => unimplemented!(),
        Fields::Named(ref fields) => {
            let encoders = fields.named.iter().map(encode_named_field);
            quote! {
                [#(#encoders),*].concat()
            }
        }
        Fields::Unnamed(ref fields) => {
            let encoders = fields.unnamed.iter().enumerate().map(encode_unnamed_field);
            quote! {
                [#(#encoders),*].concat()
            }
        }
    };
    quote! {
        impl #impl_gen pade::PadeEncode for #name #ty_gen #where_clause {
            fn pade_encode(&self) -> Vec<u8> {
                #field_encoders
            }
        }
    }
}

fn build_field_encoders(data: &Data) -> TokenStream {
    match data {
        Data::Struct(ref s) => match s.fields {
            Fields::Unit => unimplemented!(),
            Fields::Named(ref fields) => {
                let encoders = fields.named.iter().map(encode_named_field);
                quote! {
                    [#(#encoders),*].concat()
                }
            }
            Fields::Unnamed(ref fields) => {
                let encoders = fields.unnamed.iter().enumerate().map(encode_unnamed_field);
                quote! {
                    [#(#encoders),*].concat()
                }
            }
        },
        Data::Enum(ref e) => {
            let variant_count = e.variants.len();
            // This will panic if there are no variants, is that a legal state?
            let variant_bits = variant_count.ilog2() + 1;
            let variant_bytes = variant_bits.div_ceil(8) as usize;
            // Each variant gets a clause in the match
            let clauses = e.variants.iter().enumerate().map(|(i, v)| {
                let raw_number = number_to_literals(i, variant_bytes);
                let number_encoder = std::iter::once(quote! {
                    [#(#raw_number),*]
                });
                let name = &v.ident;
                let encoders: Vec<TokenStream> = match v.fields {
                    Fields::Named(ref fields) => number_encoder
                        .chain(fields.named.iter().map(encode_named_field))
                        .collect(),
                    Fields::Unnamed(ref fields) => number_encoder
                        .chain(fields.unnamed.iter().enumerate().map(encode_unnamed_field))
                        .collect(),
                    Fields::Unit => number_encoder.collect()
                };
                quote! {
                    Self::#name => [#(#encoders),*].concat()
                }
            });
            let match_statement = quote! {
                match self {
                    #(#clauses),*
                }
            };
            match_statement
        }
        _ => unimplemented!()
    }
}

fn number_to_literals(value: usize, bytes: usize) -> Vec<Literal> {
    value.to_le_bytes()[0..bytes]
        .iter()
        .map(|n| Literal::u8_suffixed(*n))
        .collect()
}

fn encode_named_field(f: &Field) -> TokenStream {
    let name = &f.ident;
    quote_spanned! {f.span()=>
        pade::PadeEncode::pade_encode(&self.#name)
    }
}

fn encode_unnamed_field(params: (usize, &Field)) -> TokenStream {
    let (i, f) = params;
    let index = Index::from(i);
    quote_spanned! {f.span()=>
        pade::PadeEncode::pade_encode(&self.#index)
    }
}
