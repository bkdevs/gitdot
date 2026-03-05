use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, ImplItem, ItemImpl, ReturnType, Type, parse_macro_input, parse_quote};

fn return_type_is_result(ret: &ReturnType) -> bool {
    match ret {
        ReturnType::Default => false,
        ReturnType::Type(_, ty) => matches_result(ty),
    }
}

fn matches_result(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        type_path.path.segments.last().map_or(false, |s| s.ident == "Result")
    } else {
        false
    }
}

fn already_instrumented(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        let segs: Vec<String> = attr.path().segments.iter().map(|s| s.ident.to_string()).collect();
        let segs: Vec<&str> = segs.iter().map(String::as_str).collect();
        matches!(segs.as_slice(), ["tracing", "instrument"] | ["instrument"])
    })
}

#[proc_macro_attribute]
pub fn instrument_all(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut impl_block = parse_macro_input!(input as ItemImpl);

    for item in &mut impl_block.items {
        if let ImplItem::Fn(method) = item {
            if method.sig.asyncness.is_none() || already_instrumented(&method.attrs) {
                continue;
            }
            let attr: Attribute = if return_type_is_result(&method.sig.output) {
                parse_quote!(#[tracing::instrument(skip_all, err)])
            } else {
                parse_quote!(#[tracing::instrument(skip_all)])
            };
            method.attrs.insert(0, attr);
        }
    }

    quote! { #impl_block }.into()
}
