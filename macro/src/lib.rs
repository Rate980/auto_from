use proc_macro::{TokenStream, TokenTree};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{ParseBuffer, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Ident, Item, ItemEnum, PathSegment, Result, Token, Type,
};
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

fn entry(input: proc_macro::TokenStream) -> Result<TokenStream2> {
    match syn::parse::<Item>(input)? {
        Item::Enum(e) => impl_enum(&e),
        x => syn_err!(x.span(); "unsupported item"),
    }
}

fn impl_enum(_ast: &ItemEnum) -> Result<TokenStream2> {
    todo!()
}

#[proc_macro_attribute]
pub fn uppercase(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match entry(item) {
        Ok(result) => result,
        Err(err) => err.to_compile_error(),
    }
    .into()
}

#[proc_macro]
pub fn union_enum(input: TokenStream) -> TokenStream {
    match enum_entry(input) {
        Ok(res) => res,
        Err(err) => err.to_compile_error(),
    }
    .into()
}

fn enum_entry(input: TokenStream) -> Result<TokenStream2> {
    let trees: Vec<_> = input.into_iter().collect();
    let name = syn::parse::<Ident>(trees[0].clone().into())?;
    match &trees[1] {
        TokenTree::Punct(x) => {
            if x.as_char() != ';' {
                syn_err!(x.span().into(); "need ;")
            }
        }
        x => syn_err!(x.span().into(); "need ;"),
    }
    let args = TokenStream::from_iter(trees[2..].into_iter().map(|x| x.clone()));
    let path: Punctuated<PathSegment, Token![::]> = syn::parse(input)?;
    // let res:Punctuated::<Type, Token![,]> = ::parse_terminated(syn::parse(args)?)?;
    println!("{:?}", res);
    Ok(quote!(
        enum #name{

        }
    ))
}
