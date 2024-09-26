use itertools::multiunzip;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, Generics, Ident, Index
};

pub fn build_decode(input: DeriveInput) -> proc_macro::TokenStream {
    let expanded = match input.data {
        Data::Struct(ref s) => build_struct_impl(&input.ident, &input.generics, s),
        Data::Enum(ref e) => build_enum_impl(&input.ident, &input.generics, e),
        _ => unimplemented!("Not yet able to derive on this type")
    };
    proc_macro::TokenStream::from(expanded)
}

fn build_struct_impl(name: &Ident, generics: &Generics, s: &DataStruct) -> TokenStream {
    let (impl_gen, ty_gen, where_clause) = generics.split_for_impl();

    let field_list = match s.fields {
        Fields::Named(ref fields) => &fields.named,
        Fields::Unnamed(ref fields) => &fields.unnamed,
        _ => unimplemented!()
    };

    let (assigned_name, default_name, field_decoders): (Vec<TokenStream>, Vec<TokenStream>,Vec<TokenStream>) = multiunzip(field_list
        .iter()
        .enumerate()
        .map(|(idx, f)| {
            let (name, default_name) = f
                .ident
                .as_ref()
                .map(|i| {
                        let id  = format_ident! ("field_{}", i);
                        (quote! { #id }, quote! { #i })
                })
                .unwrap_or_else(|| {
                        let i  = format_ident! ("field_{}", idx );
                        (quote! { #i }, quote! { #idx })
                });

            let field_type = &f.ty;
            // See if we've been given an encoding width override
            let decode_command = f
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("pade_width"))
                .map(|attr| {
                    attr.parse_args::<Literal>()
                        // If we find our literal, set it to do our encode with width
                        .map(|w| {
                            quote_spanned! { attr.span() =>
                                // value is some if we have a enum varient.
                                let is_enum = Some(<$field_type>::PADE_VARIANT_MAP_BITS).filter(|b| b != 0);
                                let #name = if let Some(is_enum) = is_enum {
                                    // the split here naturally will extract out the bitmap fields
                                    let variant_bits = bitmap.split_off(is_enum);
                                    let var_e: u8 = variant_bits.load_be();
                                     <$field_type>::pade_decode_with_width(buf, #w, Some(var_e))?
                                } else {
                                     #field_type::pade_decode_with_width(buf, #w, None)?
                                }
                            }
                        })
                        .unwrap_or_else(|_| {
                            syn::Error::new(
                                attr.span(),
                                "pade_width requires a single literal usize value"
                            )
                            .to_compile_error()
                        })
                })
                .unwrap_or_else(
                    || quote_spanned! { f.span() => 
                        let is_enum = Some(<$field_type>::PADE_VARIANT_MAP_BITS).filter(|b| b != 0);
                        let #name = if let Some(is_enum) = is_enum {
                            // the split here naturally will extract out the bitmap fields
                            let variant_bits = bitmap.split_off(is_enum);
                            let var_e: u8 = variant_bits.load_be();
                             <$field_type>::pade_decode(buf, Some(var_e))?
                        } else {
                             #field_type::pade_decode(buf,  None)?
                        }

                    }
                );

                (name, default_name, decode_command)
        }));

    quote! (
      impl #impl_gen pade::PadeDecode for #name #ty_gen #where_clause {
          fn pade_decode(buf: &mut &[u8], var: Option<u8>) -> Result<Self, ()> {
              let bitmap_bytes = Self::PADE_VARIANT_MAP_BITS.div_ceil(8);
              let mut bitmap = pade::bitvec::BitVec::<u8, bitvec::order::Msb0>::from_slice(&buf[0..bitmap_bytes]);
              #(#field_decoders)*

              Ok(Self {
                  #(
                      #default_name: #assigned_name,
                  )*
              })

          }
      }
    )
}

fn build_enum_impl(name: &Ident, generics: &Generics, e: &DataEnum) -> TokenStream {
    let (impl_gen, ty_gen, where_clause) = generics.split_for_impl();
    let variant_count = e.variants.len();
    // This will panic if there are no variants, is that a legal state?
    let variant_bits = (variant_count.ilog2() + 1) as usize;
    let variant_bytes = variant_bits.div_ceil(8);
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
    quote! {
        impl #impl_gen pade::PadeEncode for #name #ty_gen #where_clause {
            const PADE_VARIANT_MAP_BITS: usize = #variant_bits;
            fn pade_encode(&self) -> Vec<u8> {
                match self {
                    #(#clauses),*
                }
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
