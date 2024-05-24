//! proc macro crate
//!
//! for enum, we'd like to generate From impls for each variant
//!

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let ident = input.ident;
    let variants = match input.data {
        syn::Data::Enum(ref data) => &data.variants,
        _ => panic!("EnumFrom can only be derived for enums"),
    };
    let from_impls = variants.iter().map(|v| {
        let var = &v.ident;
        let fields = match &v.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().unwrap();
                    let ty = &field.ty;
                    quote! {
                        impl From<#ty> for #ident {
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }
            _ => quote! {},
        };
        fields
    });

    // quote return proc-macro2::TokenStream, so we need to convert it to TokenStream
    quote! {
        // #用来插入变量
        // #()* 类似$()，表示重复0次或多次，他会自动展开可迭代的内容
        // 需要注意的是，这里的 iter 必须是一个实现了 IntoIterator trait 的类型，
        // 这样 quote 宏才能对其进行迭代和展开。如果 iter 是一个迭代器，那么它会被展开。
        #( #from_impls )*
    }
    .into()
}
