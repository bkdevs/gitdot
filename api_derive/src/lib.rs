use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(ApiResource)]
pub fn derive_api_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics crate::ApiResource for #name #ty_generics #where_clause {}
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(EndpointRequest)]
pub fn derive_endpoint_request(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics crate::EndpointRequest for #name #ty_generics #where_clause {}
    };
    TokenStream::from(expanded)
}
