#![feature(iterator_try_collect)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2_diagnostics::{Diagnostic, Level};
use syn::{spanned::Spanned, DeriveInput};

mod reactive;

#[proc_macro_derive(Reactive, attributes(reactive))]
pub fn derive_reactive(input: TokenStream) -> TokenStream {
    match reactive::try_reactive(input) {
        Ok(token_stream) => token_stream,
        Err(diagnostic) => diagnostic.emit_as_expr_tokens().into(),
    }
}

pub(crate) fn error(spanned: impl Spanned, message: impl Into<String>) -> Diagnostic {
    Diagnostic::spanned(spanned.span(), Level::Error, message)
}

pub(crate) fn parse_input(input: TokenStream) -> Result<DeriveInput, Diagnostic> {
    syn::parse::<DeriveInput>(input).map_err(|e| error(e.span(), e.to_string()))
}

pub(crate) fn _reactive_crate_name() -> Result<TokenStream, Diagnostic> {
    use proc_macro_crate::{crate_name, FoundCrate};
    use quote::quote;
    use syn::Ident;

    let found_crate =
        crate_name("sorrow-reactive").map_err(|e| error(Span::call_site(), e.to_string()))?;

    match found_crate {
        FoundCrate::Itself => Ok((quote! { crate }).into()),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            Ok((quote! { #ident }).into())
        }
    }
}
