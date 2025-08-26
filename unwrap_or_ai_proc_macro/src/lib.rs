use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn unwrap_or_ai_func(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let fn_name = &input.sig.ident;
    let helper_fn_name = syn::Ident::new(&format!("print_source_of_{}", fn_name), fn_name.span());

    // Collect all attributes (this includes doc comments)
    let attrs = &input.attrs;
    let sig = &input.sig;
    let block = &input.block;

    // Reconstruct with attributes so docs are preserved
    let src_string = quote! {
        #(#attrs)*
        #sig #block
    }
    .to_string();

    let expanded = quote! {
        #input

        pub fn #helper_fn_name() -> &'static str {
            #src_string
        }
    };

    expanded.into()
}
