use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(Duplicate)]
pub fn impl_duplicate(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Create the where clause for the Duplicate trait
    let duplicate_where_clause = {
        let predicates = where_clause
            .map(|wc| wc.predicates.iter().collect::<Vec<_>>())
            .unwrap_or_default();

        if !predicates.is_empty() {
            quote! { where #(#predicates,)* }
        } else {
            quote! {}
        }
    };

    // Generate the implementation
    let expanded = quote! {
        impl #impl_generics ::dupit::Duplicate for #name #ty_generics #duplicate_where_clause {
            fn dup(&self) -> Self {
                use ::dupit::impls::DuplicateImpl;
                (&&::dupit::impls::Wrapper(self)).duplicate_impl()
            }
        }
    };

    TokenStream::from(expanded)
}
