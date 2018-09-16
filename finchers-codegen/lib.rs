//!

#![doc(html_root_url = "https://docs.rs/finchers-codegen/0.12.0-alpha.1")]

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn endpoint(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item: syn::ItemFn = syn::parse(item).expect("failed to parse item");

    let (output, rarrow) = match item.decl.output {
        syn::ReturnType::Default => (
            syn::Type::Tuple(syn::TypeTuple {
                paren_token: Default::default(),
                elems: Default::default(),
            }),
            Default::default(),
        ),
        syn::ReturnType::Type(ref rarrow, ref ty) => ((**ty).clone(), rarrow.clone()),
    };
    let output: syn::Type = syn::parse(
        (quote::quote! {
            ::finchers::endpoint::IntoLocal<
                impl for<'__a> ::finchers::endpoint::SendEndpoint<'__a, Output = #output>
            >
        }).into(),
    ).unwrap();
    item.decl.output = syn::ReturnType::Type(rarrow, Box::new(output));

    let orig_block = &item.block;
    let block: syn::Block = syn::parse(
        quote::quote! {
            {
                ::finchers::endpoint::SendEndpoint::into_local(
                    #orig_block
                )
            }
        }.into(),
    ).unwrap();
    item.block = Box::new(block);

    quote::quote!(#item).into()
}
