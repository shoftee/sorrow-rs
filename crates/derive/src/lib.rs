#![feature(iterator_try_collect)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2_diagnostics::{Diagnostic, Level};
use proc_macro_crate::{crate_name, FoundCrate};
use syn::{DeriveInput, Ident};

mod reactive;

#[proc_macro_derive(Reactive, attributes(reactive))]
pub fn derive_reactive(input: TokenStream) -> TokenStream {
    match reactive::try_derive(input) {
        Ok(token_stream) => token_stream,
        Err(error) => error.into_diagnostic().emit_as_expr_tokens().into(),
    }
}

pub(crate) fn parse_input(input: TokenStream) -> Result<DeriveInput, Error> {
    syn::parse::<DeriveInput>(input).map_err(Error::Syn)
}

pub(crate) fn core_crate_name() -> Result<FoundCrate, Error> {
    crate_name("sorrow-core").map_err(Error::ProcMacroCrate)
}

pub(crate) fn found_crate_ident(found_crate: FoundCrate) -> Ident {
    match found_crate {
        FoundCrate::Itself => Ident::new("crate", Span::call_site()),
        FoundCrate::Name(name) => Ident::new(name.as_str(), Span::call_site()),
    }
}

pub(crate) enum Error {
    Syn(syn::Error),
    ProcMacroCrate(proc_macro_crate::Error),
    Other(Span, String),
}

impl Error {
    fn spanned(spanned: impl syn::spanned::Spanned, message: impl Into<String>) -> Error {
        Error::Other(spanned.span(), message.into())
    }

    pub(crate) fn into_diagnostic(self) -> Diagnostic {
        match self {
            Error::Syn(e) => Diagnostic::spanned(e.span(), Level::Error, e.to_string()),
            Error::ProcMacroCrate(e) => {
                Diagnostic::spanned(Span::call_site(), Level::Error, e.to_string())
            }
            Error::Other(span, message) => Diagnostic::spanned(span, Level::Error, message),
        }
    }
}
