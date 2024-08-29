use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, Generics,
    Ident, Index
};

#[proc_macro_derive(PadeEncode)]
pub fn pade_encode_fn(raw: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(raw as DeriveInput);

    let expanded = match input.data {
        Data::Struct(ref s) => build_struct_impl(&input.ident, &input.generics, s),
        Data::Enum(ref e) => build_enum_impl(&input.ident, &input.generics, e),
        _ => unimplemented!("Not yet able to derive on this type")
    };
    proc_macro::TokenStream::from(expanded)
}

fn build_struct_impl(name: &Ident, generics: &Generics, s: &DataStruct) -> TokenStream {
    let (impl_gen, ty_gen, where_clause) = generics.split_for_impl();

    let field_encoders = match s.fields {
        Fields::Unit => unimplemented!(),
        Fields::Named(ref fields) => {
            // let encoders = fields.named.iter().map(encode_named_field);
            let encoders = fields.named.iter().map(|f| {
                let name = f.ident.as_ref().unwrap();
                let encoded = format_ident!("{}_encoded", name);
                let header_bytes = format_ident!("{}_header_bytes", name);
                // TODO:  get quote_spanned in here
                quote! {
                    let #encoded = self.#name.pade_encode();
                    let #header_bytes = self.#name.pade_header_bits().div_ceil(8) as usize;
                    output.extend(
                        if #header_bytes > 0 {
                            headers.extend_from_bitslice(
                                pade::bitvec::view::BitView::view_bits::<pade::bitvec::order::Lsb0>(
                                    &#encoded[0..#header_bytes]
                                ).split_at(self.#name.pade_header_bits() as usize).0);
                        #encoded[#header_bytes..].iter()
                    } else { #encoded[0..].iter() });
                }
            });
            quote! {
                #(#encoders)*
            }
        }
        Fields::Unnamed(ref fields) => {
            let encoders = fields.unnamed.iter().enumerate().map(|(i, _f)| {
                let name = Index::from(i);
                let encoded = format_ident!("field_{}_encoded", name);
                let header_bytes = format_ident!("field_{}_header_bytes", name);
                quote! {
                    let #encoded = self.#name.pade_encode();
                    let #header_bytes = self.#name.pade_header_bits().div_ceil(8) as usize;
                    output.extend(
                        if #header_bytes > 0 {
                            headers.extend_from_bitslice(
                            pade::bitvec::view::BitView::view_bits::<pade::bitvec::order::Lsb0>(
                                &#encoded[0..#header_bytes]
                            ).split_at(self.#name.pade_header_bits() as usize).0);
                        #encoded[0..#header_bytes].iter()
                    } else { #encoded[0..].iter() });
                }
            });
            quote! {
                #(#encoders)*
            }
        }
    };
    quote! {
        impl #impl_gen pade::PadeEncode for #name #ty_gen #where_clause {
            fn pade_encode(&self) -> Vec<u8> {
                let mut headers = pade::bitvec::vec::BitVec::<u8, pade::bitvec::order::Lsb0>::new();
                let mut output: Vec<u8> = Vec::new();
                #field_encoders
                [headers.as_raw_slice().to_vec(), output].concat()
            }
        }
    }
}

fn build_enum_impl(name: &Ident, generics: &Generics, e: &DataEnum) -> TokenStream {
    let (impl_gen, ty_gen, where_clause) = generics.split_for_impl();
    let variant_count = e.variants.len();
    // This will panic if there are no variants, is that a legal state?
    let variant_bits = variant_count.ilog2() + 1;
    let variant_bytes = variant_bits.div_ceil(8) as usize;
    // Each variant gets a clause in the match
    let clauses = e.variants.iter().enumerate().map(|(i, v)| {
        let raw_number = number_to_literals(i, variant_bytes);
        let number_encoder = std::iter::once(quote! {
            [#(#raw_number),*].to_vec()
        });
        let name = &v.ident;
        match v.fields {
            Fields::Named(ref fields) => {
                let unnamed_fields = fields.named.iter().map(|f| {
                    let name = f.ident.as_ref().unwrap();
                    let field_encoder = quote_spanned! {f.span()=>
                        pade::PadeEncode::pade_encode(#name)
                    };
                    (name, field_encoder)
                });
                let (field_names, field_encoders): (Vec<&Ident>, Vec<TokenStream>) =
                    unnamed_fields.unzip();
                let all_encoders = number_encoder.chain(field_encoders);
                quote! {
                    Self::#name { #(#field_names),* } => [#(#all_encoders),*].concat()
                }
            }
            Fields::Unnamed(ref fields) => {
                let unnamed_fields = fields.unnamed.iter().enumerate().map(|(i, f)| {
                    let num = Index::from(i);
                    let field_name = format_ident!("field_{}", num);
                    let field_encoder = quote_spanned! {f.span()=>
                        pade::PadeEncode::pade_encode(#field_name)
                    };
                    (field_name, field_encoder)
                });
                let (field_names, field_encoders): (Vec<Ident>, Vec<TokenStream>) =
                    unnamed_fields.unzip();
                let all_encoders = number_encoder.chain(field_encoders);
                quote! {
                    Self::#name(#(#field_names),*) => [#(#all_encoders),*].concat()
                }
            }
            Fields::Unit => {
                quote! {
                    Self::#name => [#(#raw_number),*].to_vec()
                }
            }
        }
    });
    let match_statement = quote! {
        match self {
            #(#clauses),*
        }
    };
    quote! {
        impl #impl_gen pade::PadeEncode for #name #ty_gen #where_clause {
            fn pade_encode(&self) -> Vec<u8> {
                #match_statement
            }
        }
    }
}

fn number_to_literals(value: usize, bytes: usize) -> Vec<Literal> {
    value.to_le_bytes()[0..bytes]
        .iter()
        .map(|n| Literal::u8_suffixed(*n))
        .collect()
}
