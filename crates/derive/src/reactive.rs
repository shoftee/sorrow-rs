use proc_macro::TokenStream;
use proc_macro2_diagnostics::Diagnostic;
use quote::quote;
use syn::{AttrStyle, Data, DeriveInput, Field, Fields, FieldsNamed, Ident, Type};

use crate::{error, parse_input};

#[derive(Clone, Copy, PartialEq)]
enum FieldKind {
    Dependent,
    Nested,
}

pub(crate) fn try_reactive(input: TokenStream) -> Result<TokenStream, Diagnostic> {
    let ast = parse_input(input)?;

    if !ast.generics.params.is_empty() {
        return Err(error(
            ast.generics,
            "derive(Reactive) does not support generics.",
        ));
    }

    let vis = ast.vis.clone();
    let ident = ast.ident.clone();

    let fields = named_fields(ast)?.named;

    let (nested_fields, dependent_fields) = partition_by_kind(&fields)?;

    //let reactive_crate = reactive_crate_name()?;
    let reactive_ident = reactive_ident(&ident);

    let nested_decls = nested_fields.iter().map(|&field| {
        let vis = field.vis.clone();
        let ident = field.ident.clone().expect("all fields should be named");
        let ty = field.ty.clone();
        quote! { #vis #ident: <#ty as ::sorrow_reactive::IntoReactive>::Target }
    });
    let state_decls = dependent_fields.iter().map(|&field| {
        let vis = field.vis.clone();
        let ident = field.ident.clone().expect("all fields should be named");
        let ty = field.ty.clone();
        quote! { #vis #ident: ::sorrow_reactive::State<#ty> }
    });

    let reactive_struct_decl = quote! {
        #vis struct #reactive_ident {
            #(#nested_decls,)*
            #(#state_decls,)*
        }
    };

    let field_idents = fields.iter().map(|field| {
        let ident = field.ident.clone().expect("all fields should be named");
        quote! { #ident }
    });
    let nested_initializers = nested_fields.iter().map(|&field| {
        let ident = field.ident.clone().expect("all fields should be named");
        let ty = field.ty.clone();
        quote! { #ident: <#ty as ::sorrow_reactive::IntoReactive>::into_reactive(#ident, __runtime) }
    });
    let dependent_initializers = dependent_fields.iter().map(|&field| {
        let ident = field.ident.clone().expect("all fields should be named");
        quote! { #ident: __runtime.create_state(#ident) }
    });

    let into_reactive_impl = quote! {
        #[automatically_derived]
        impl ::sorrow_reactive::IntoReactive for #ident {
            type Target = #reactive_ident;
            fn into_reactive(self, __runtime: &::sorrow_reactive::Runtime) -> Self::Target {
                let Self {
                    #(#field_idents,)*
                } = self;
                Self::Target {
                    #(#nested_initializers,)*
                    #(#dependent_initializers,)*
                }
            }
        }
    };

    let token_stream = quote! {
        #reactive_struct_decl
        #into_reactive_impl
    };

    Ok(token_stream.into())
}

fn partition_by_kind(
    fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>,
) -> Result<(Vec<&Field>, Vec<&Field>), Diagnostic> {
    let fields: Vec<_> = fields
        .iter()
        .map(|field| -> Result<_, Diagnostic> {
            let kind = if is_nested(field)? {
                FieldKind::Nested
            } else {
                FieldKind::Dependent
            };
            Ok((field, kind))
        })
        .try_collect()?;

    let nested: Vec<_> = fields
        .iter()
        .filter_map(|&(f, k)| match k {
            FieldKind::Nested => Some(f),
            FieldKind::Dependent => None,
        })
        .collect();
    let dependent: Vec<_> = fields
        .iter()
        .filter_map(|&(f, k)| match k {
            FieldKind::Dependent => Some(f),
            FieldKind::Nested => None,
        })
        .collect();
    Ok((nested, dependent))
}

fn named_fields(ast: DeriveInput) -> Result<FieldsNamed, Diagnostic> {
    let data = match ast.data {
        Data::Struct(data) => Ok(data),
        Data::Enum(e) => Err(error(
            e.enum_token,
            "derive(Reactive) is only supported on struct types.",
        )),
        Data::Union(u) => Err(error(
            u.union_token,
            "derive(Reactive) is only supported on struct types.",
        )),
    }?;

    let fields = match data.fields {
        Fields::Named(fields) => Ok(fields),
        _ => Err(error(
            data.fields,
            "derive(Reactive) is not supported on tuples.",
        )),
    }?;

    if fields.named.is_empty() {
        return Err(error(
            fields,
            "derive(Reactive) is only supported on structs with one or more named fields.",
        ));
    }

    let mut complex_field_types = fields.named.iter().filter_map(|f| match f.ty {
        Type::Path(_) => None,
        _ => Some(&f.ty),
    });
    if let Some(ty) = complex_field_types.next() {
        return Err(error(ty, "derive(Reactive) only supports basic types."));
    }

    Ok(fields)
}

fn is_nested(field: &Field) -> Result<bool, Diagnostic> {
    let reactive_attr = field
        .attrs
        .iter()
        .find(|attr| matches!(attr.style, AttrStyle::Outer) && attr.path().is_ident("reactive"));

    match reactive_attr {
        None => Ok(false),
        Some(reactive) => reactive
            .parse_nested_meta(|meta| {
                if meta.path.is_ident("nested") {
                    Ok(())
                } else {
                    Err(meta.error("invalid reactive attribute"))
                }
            })
            .and(Ok(true))
            .map_err(|e| error(e.span(), e.to_string())),
    }
}

fn reactive_ident(ident: &Ident) -> Ident {
    let name = format!("Reactive{}", ident);
    Ident::new(&name, ident.span())
}
