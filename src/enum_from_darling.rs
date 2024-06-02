use darling::{FromDeriveInput, FromField, FromVariant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_enum_from_darling(input: DeriveInput) -> TokenStream {
    let EnumFromDarling {
        ident,
        generics,
        data,
    } = EnumFromDarling::from_derive_input(&input).unwrap();
    println!("{:#?}", ident);
    println!("{:#?}", generics);
    println!("{:#?}", data);

    let from_impl = match data {
        darling::ast::Data::Enum(data1) => {
            let iter = data1.into_iter().map(|variant| {
                let var = &variant.ident;
                let style = &variant.fields.style;
                match style {
                    darling::ast::Style::Tuple if variant.fields.len() == 1 => {
                        let field = variant.fields.iter().next().expect("should have 1 field");
                        let ty = &field.ty;
                        quote! {
                            impl #generics From<#ty> for #ident #generics {
                                fn from(v: #ty) -> Self {
                                    #ident::#var(v)
                                }
                            }
                        }
                    }
                    _ => quote! {},
                }
            });
            iter
        }
        _ => panic!("EnumFromDarling can only be derived for enums"),
    };

    quote! {#(#from_impl)*}
}

#[derive(Debug, FromDeriveInput)]
struct EnumFromDarling {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<EnumVariant, ()>,
}

#[derive(Debug, FromVariant)]
struct EnumVariant {
    ident: syn::Ident,
    fields: darling::ast::Fields<EnumVariantField>,
}

#[derive(Debug, FromField)]
struct EnumVariantField {
    ty: syn::Type,
}
