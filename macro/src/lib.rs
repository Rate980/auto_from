use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse::Parse, punctuated::Punctuated, spanned::Spanned, Expr, Generics, Ident, Item, ItemEnum,
    PathArguments, Result, Token, Type, TypeParam, TypeParamBound, TypePath,
};
#[macro_use]
mod utils;
#[derive(Debug, Clone)]
struct EnumFilled(Type, Ident);

impl Parse for EnumFilled {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let type_ = input.parse::<Type>()?;
        let ident = if input.peek(Token!(:)) {
            input.parse::<Token!(:)>()?;
            input.parse::<Ident>()?
        } else {
            to_enum_ident(&type_)?
        };
        Ok(Self(type_, ident))
    }
}
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
    let parser = Punctuated::<EnumFilled, Token![,]>::parse_terminated;
    let fields = parser.parse(args)?;
    let types = fields.iter().map(|x| x.0.clone()).collect::<Vec<Type>>();
    let names = fields.iter().map(|x| x.1.clone()).collect::<Vec<Ident>>();
    let res = quote!(
        enum #name{
            #(#names(#types)),*
        }
        #(
            impl From<#types> for #name{
                fn from(value: #types) -> Self{
                    Self::#names(value)
                }
            }
        )*
    );
    // println!("{}", res);
    Ok(res)
    // Ok(quote!(
    //     enum #name{
    //         #(#types(#names)),*
    //     }
    // ))
}

fn to_enum_ident(t: &Type) -> Result<Ident> {
    match t {
        Type::Array(t) => array_to_ident(t),
        Type::Paren(t) => to_enum_ident(&*t.elem),
        Type::Path(t) => path_to_ident(t),
        Type::Macro(_)
        | Type::ImplTrait(_)
        | Type::TraitObject(_)
        | Type::Tuple(_)
        | Type::BareFn(_) => {
            syn_err!(t.span(); "need name")
        }
        _ => syn_err!(t.span(); "unsnapped type"),
    }
}

fn array_to_ident(t: &syn::TypeArray) -> std::result::Result<Ident, syn::Error> {
    Ok(Ident::new(
        format!("{}_{}", to_enum_ident(&*t.elem)?, array_len_to_str(&t.len)?).as_str(),
        Span::call_site(),
    ))
}

fn array_len_to_str(t: &syn::Expr) -> Result<String> {
    match t {
        Expr::Lit(t) => match &t.lit {
            syn::Lit::Int(x) => Ok(x.to_string()),
            _ => syn_err!(t.span(); "need name"),
        },
        _ => syn_err!(t.span(); "need name"),
    }
}

// fn ref_to_ident(t: &syn::TypeReference) -> Result<Ident> {
//     Ok(Ident::new(
//         format!(
//             "Ref{}{}",
//             match t.mutability {
//                 Some(_) => "Mut",
//                 None => "",
//             },
//             to_enum_ident(&*t.elem)?
//         )
//         .as_str(),
//         Span::call_site(),
//     ))
// }

fn path_to_ident(t: &TypePath) -> Result<Ident> {
    let name = match t.path.segments.last() {
        Some(x) => x,
        None => syn_err!(t.span(); "need type"),
    };
    if PathArguments::None != name.arguments {
        syn_err!(t.span(); "need name")
    }
    let mut name = name.ident.to_string();
    match name.get_mut(0..=0) {
        Some(x) => x.make_ascii_uppercase(),
        None => syn_err!(t.span(); "err"),
    }
    Ok(Ident::new(&name, Span::call_site()))
}

// fn slice_to_ident(t: &TypeSlice) -> Result<Ident> {
//     Ok(Ident::new(
//         format!("Slice{}", to_enum_ident(&*t.elem)?).as_str(),
//         Span::call_site(),
//     ))
// }
