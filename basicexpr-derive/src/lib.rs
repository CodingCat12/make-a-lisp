use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(BasicExpr)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl Expr<#ident> for #ident {
            fn eval(&self) -> #ident {
                *self
            }
        }
    };
    output.into()
}
