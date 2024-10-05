use proc_macro2::Span;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Fields, Generics, Ident, Type, Variant, Visibility
};

pub(crate) fn derive(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    match input.data {
        Data::Struct(s) => Ok(derive_struct(s, input.ident)),
        Data::Enum(e) => Ok(derive_enum(e, input.ident)),
        _ => unimplemented!("Not yet able to derive on this type")
    }
}

fn derive_enum(item: DataEnum, name: Ident) -> proc_macro2::TokenStream {
    let varient_matches = item
        .variants
        .iter()
        .map(|variant| enum_variant_match(&name, &variant.ident, &variant.fields))
        .collect::<Vec<_>>();

    let varient_names = item
        .variants
        .into_iter()
        .map(|variant| variant.ident)
        .collect::<Vec<_>>();

    let any_variant = any_enum_variant(&name, varient_names);

    let matches = quote::quote! {
        #(#varient_matches)*
        #any_variant
    };

    let macro_name = Ident::new(&format!("random_{}", name.to_string()), Span::call_site());
    let mod_name =
        Ident::new(&format!("mod_{}", name.to_string().to_lowercase()), Span::call_site());
    quote::quote! {
        pub use #mod_name::#macro_name;
        #[macro_use]
        mod #mod_name {
            #[macro_export]
            macro_rules! #macro_name {
                #matches
            }
            pub use #macro_name;
        }
    }
}

fn enum_variant_match(
    enum_name: &Ident,
    variant_name: &Ident,
    variant_fields: &Fields
) -> proc_macro2::TokenStream {
    if variant_fields.len() == 0 {
        quote::quote! {
            (#variant_name) => {
                {
                    #enum_name::#variant_name
                }
            };
        }
    } else if variant_fields.len() == 1 {
        let ty = variant_fields.into_iter().next().unwrap().ty.clone();
        let field = match ty {
            // Type::Paren(type_paren) => type_paren,
            // Type::Array(type_array) => todo!(),
            // Type::BareFn(type_bare_fn) => todo!(),
            // Type::Group(type_group) => todo!(),
            // Type::ImplTrait(type_impl_trait) => todo!(),
            // Type::Infer(type_infer) => todo!(),
            // Type::Macro(type_macro) => todo!(),
            // Type::Never(type_never) => todo!(),
            Type::Path(type_path) => type_path.path.get_ident().cloned().unwrap(),
            // Type::Ptr(type_ptr) => todo!(),
            // Type::Reference(type_reference) => todo!(),
            // Type::Slice(type_slice) => todo!(),
            // Type::TraitObject(type_trait_object) => todo!(),
            // Type::Tuple(type_tuple) => todo!(),
            // Type::Verbatim(token_stream) => todo!(),
            _ => unreachable!("type is {:?}", ty)
        };

        let random_field = Ident::new(&format!("random_{}", field.to_string()), Span::call_site());

        quote::quote! {
            (#variant_name | $(($field_name:ident, $value:expr)),*) => {
                {
                    #random_field!($($field_name:ident, $value:expr),*)
                }
            };

            (#variant_name) => {
                {
                    #random_field!()
                }
            };
        }
    } else {
        panic!("enum variant fields can only have 1 or 0 values");
    }
}

fn any_enum_variant(enum_name: &Ident, variant_names: Vec<Ident>) -> proc_macro2::TokenStream {
    let num_variants = (0..variant_names.len()).into_iter().collect::<Vec<_>>();

    quote::quote! {
        () => {
            {
                let mut rng = rand::thread_rng();
                let mut this = rng.gen::<u16>();
                match this {
                    #(
                        #num_variants => random_ #enum_name!(#variant_names),
                    )*
                }
            }
        };
    }
}

fn derive_struct(item: DataStruct, name: Ident) -> proc_macro2::TokenStream {
    if item.fields.iter().any(|f| f.ident.is_none()) {
        unimplemented!("unit struct not implemented");
    } else if item
        .fields
        .iter()
        .any(|f| !matches!(f.vis, Visibility::Public(_)))
    {
        panic!("struct must have all public fields")
    }

    let fields_str = item
        .fields
        .iter()
        .map(|field| field.ident.as_ref().map(|f| f.to_string()))
        .collect::<Vec<_>>();

    let macro_name = Ident::new(&format!("random_{}", name.to_string()), Span::call_site());
    let mod_name =
        Ident::new(&format!("mod_{}", name.to_string().to_lowercase()), Span::call_site());
    quote::quote! {
        pub use #mod_name::#macro_name;
        #[macro_use]
        mod #mod_name {
            #[macro_export]
            macro_rules! #macro_name {
                ($(($field_name:ident, $value:expr)),*) => {
                    {
                        let mut rng = rand::thread_rng();
                        let mut this = rng.gen::<#name>();
                        let fields_to_get = std::collections::HashSet::from([#(#fields_str),*]);
                        $(
                            if fields_to_get.contains(std::stringify!($field_name)) {
                                this.$field_name = $value;
                            }
                        )*
                        this
                    }
                };
            }
            pub use #macro_name;
        }
    }
}
