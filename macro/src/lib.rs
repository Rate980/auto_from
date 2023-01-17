use proc_macro2::TokenStream;
use syn::{spanned::Spanned, Item, ItemEnum, Result};
#[macro_use]
mod utils;
#[proc_macro_derive(From)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match entry(input) {
        Ok(result) => result,
        Err(err) => err.to_compile_error(),
    }
    .into()
}

fn entry(input: proc_macro::TokenStream) -> Result<TokenStream> {
    match syn::parse::<Item>(input)? {
        Item::Enum(e) => impl_enum(&e),
        x => syn_err!(x.span(); "unsupported item"),
    }
}

fn impl_enum(ast: &ItemEnum) -> Result<TokenStream> {
    if ast.generics.params.len() > 0 {
        syn_err!(ast.span(); "not allowed generics");
    }
    let name = ast.ident.clone();
    todo!();
}
