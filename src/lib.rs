use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn dismay(attr: TokenStream, func: TokenStream) -> TokenStream {
    let mut output = TokenStream2::new();

    let tree = parse_macro_input!(func as ItemFn);
    let mut new_sig = tree.sig.clone();

    output.extend(tree.vis.clone().into_token_stream());
    output.extend(tree.sig.clone().into_token_stream());

    let _pub = tree.vis;
    let _const = tree.sig.constness;
    let _async = tree.sig.asyncness;
    let _unsafe = tree.sig.unsafety;

    let _ident = tree.sig.ident;
    let _inputs = tree.sig.inputs;
    let _generics = tree.sig.generics;
    let _output = tree.sig.output;

    new_sig.ident = syn::Ident::new(
        &format!("__dismayed__{}", new_sig.ident),
        new_sig.ident.span(),
    );

    output.extend(new_sig.clone().into_token_stream());
    output.extend(Paren)
    for input_name in _inputs.iter() {
        match input_name {
            syn::FnArg::Typed(pat_type) => {
                let pat = &pat_type.pat;
                let ty = &pat_type.ty;
                output.extend(quote! { let #pat: #ty = #pat; });
            }
            syn::FnArg::Receiver(_) => {}
        }
    }
    output.extend(new_sig.into_token_stream());
    output.extend(tree.block.into_token_stream());

    output.into()
}
