use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Attribute, ImplItem, ItemImpl, LitStr, MetaNameValue, ReturnType, Type, parse_macro_input,
    parse_quote,
    parse::{Parse, ParseStream},
};

struct InstrumentArgs {
    level: Option<LitStr>,
}

impl Parse for InstrumentArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(InstrumentArgs { level: None });
        }
        let meta: MetaNameValue = input.parse()?;
        if !meta.path.is_ident("level") {
            return Err(syn::Error::new_spanned(&meta.path, "expected `level`"));
        }
        let level = match &meta.value {
            syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(s), .. }) => Some(s.clone()),
            other => return Err(syn::Error::new_spanned(other, "expected string literal")),
        };
        Ok(InstrumentArgs { level })
    }
}

#[proc_macro_attribute]
pub fn instrument_all(args: TokenStream, input: TokenStream) -> TokenStream {
    let InstrumentArgs { level } = parse_macro_input!(args as InstrumentArgs);
    let mut impl_block = parse_macro_input!(input as ItemImpl);

    for item in &mut impl_block.items {
        if let ImplItem::Fn(method) = item {
            if method.sig.asyncness.is_none() || already_instrumented(&method.attrs) {
                continue;
            }
            let attr: Attribute = match (&level, return_type_is_result(&method.sig.output)) {
                (Some(lvl), true) => parse_quote!(#[tracing::instrument(level = #lvl, skip_all, err)]),
                (Some(lvl), false) => parse_quote!(#[tracing::instrument(level = #lvl, skip_all)]),
                (None, true) => parse_quote!(#[tracing::instrument(skip_all, err)]),
                (None, false) => parse_quote!(#[tracing::instrument(skip_all)]),
            };
            method.attrs.insert(0, attr);
        }
    }

    quote! { #impl_block }.into()
}

fn return_type_is_result(ret: &ReturnType) -> bool {
    match ret {
        ReturnType::Default => false,
        ReturnType::Type(_, ty) => matches_result(ty),
    }
}

fn matches_result(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        type_path
            .path
            .segments
            .last()
            .map_or(false, |s| s.ident == "Result")
    } else {
        false
    }
}

fn already_instrumented(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        let segs: Vec<String> = attr
            .path()
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect();
        let segs: Vec<&str> = segs.iter().map(String::as_str).collect();
        matches!(segs.as_slice(), ["tracing", "instrument"] | ["instrument"])
    })
}
