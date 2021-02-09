use std::todo;

use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    *,
};

#[proc_macro_attribute]
pub fn transform_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_func(attr, item)
}

fn parse_func(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemFn = syn::parse_macro_input!(item as ItemFn);

    let inputs = input.sig.inputs.iter().filter_map(|c| match c {
        syn::FnArg::Receiver(_) => return None,
        syn::FnArg::Typed(syn::PatType { pat, ty, .. }) => match &**ty {
            syn::Type::Path(path) => {
                if matches!(path, TypePath{path,..} if path.is_ident( "i32" )) {
                    return Some(quote::quote! { #pat: MyI32 });
                }
                None
            }
            _ => None,
        },
    });

    let name = &input.sig.ident;
    let _ret = &input.sig.output; // TODO map this
    let block = &input.block;

    let ast = quote::quote! {
        fn #name(#(#inputs,)*) -> MyI32 #block
    };
    ast.into()
}
