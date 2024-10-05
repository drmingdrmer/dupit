use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(Duplicate)]
pub fn impl_duplicate(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Implement your macro logic here
    let name = input.ident;

    let expanded = quote! {
        impl ::dupit::Duplicate for #name {
            fn dup(&self) -> Self {
                use ::dupit::impls::DuplicateImpl;
                (&&::dupit::impls::Wrapper(self)).duplicate_impl()
            }
        }
    };

    TokenStream::from(expanded)
}
