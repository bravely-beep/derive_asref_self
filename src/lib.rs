use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(AsRef)]
pub fn derive_as_ref(item: TokenStream) -> TokenStream {
    let item: DeriveInput = parse_macro_input!(item);
    let ident = item.ident;
    quote! {
        impl ::core::convert::AsRef<#ident> for #ident {
            fn as_ref(&self) -> &#ident {
                self
            }
        }
    }
    .into()
}
