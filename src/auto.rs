use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[allow(unused)]
#[derive(Debug, FromDeriveInput)]
#[darling(attributes(deref))]
struct AutoDerefInfo {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), FieldsInfo>,

    #[darling(default)]
    mutable: bool,
    #[darling(default)]
    field: Option<syn::Ident>,
}

#[allow(unused)]
#[derive(Debug, FromField)]
struct FieldsInfo {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}

pub(crate) fn process_auto_deref(input: DeriveInput) -> TokenStream {
    if let AutoDerefInfo {
        ident,
        generics,
        data: Data::Struct(fields),
        mutable,
        field,
    } = AutoDerefInfo::from_derive_input(&input).unwrap()
    {
        let (fd, ty) = if let Some(field) = field {
            match fields.iter().find(|f| f.ident.as_ref().unwrap() == &field) {
                Some(f) => (field, &f.ty),
                None => panic!("field attribute must be a field of the struct"),
            }
        } else {
            if fields.len() == 1 {
                let f = fields.iter().next().unwrap();
                (f.ident.as_ref().unwrap().clone(), &f.ty)
            } else {
                panic!("AutoDeref can only be derived for structs with a single field")
            }
        };

        let mut code = vec![quote! {
            impl #generics std::ops::Deref for #ident #generics {
                type Target = #ty;
                fn deref(&self) -> &Self::Target {
                    &self.#fd
                }
            }
        }];

        if mutable {
            code.push(quote! {
                impl #generics std::ops::DerefMut for #ident #generics {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.#fd
                    }
                }
            });
        }
        quote! {
            #( #code )*
        }
    } else {
        panic!("AutoDeref can only be derived for structs");
    }
}

pub(crate) fn process_auto_debug(_input: DeriveInput) -> TokenStream {
    quote! {}
}
