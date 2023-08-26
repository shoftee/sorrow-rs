use proc_macro::TokenStream;
use quote::quote;
use syn::{AttrStyle, Data, DeriveInput, Field, Fields, FieldsNamed, Ident, Type};

use crate::{core_crate_name, found_crate_ident, parse_input, Error};

#[derive(Clone, Copy, PartialEq)]
enum FieldKind {
    Dependent,
    Nested,
}

pub(crate) fn try_derive(input: TokenStream) -> Result<TokenStream, Error> {
    let found_core_crate = core_crate_name()?;
    let core_crate = found_crate_ident(found_core_crate);
    let into_reactive_type = quote!(#core_crate::reactive::IntoReactive);
    let runtime_type = quote!(#core_crate::reactive::Runtime);
    let state_type = quote!(#core_crate::reactive::State);

    let ast = parse_input(input)?;

    if !ast.generics.params.is_empty() {
        return Err(Error::spanned(
            ast.generics,
            "derive(Reactive) does not support generics.",
        ));
    }

    let vis = ast.vis.clone();
    let ident = ast.ident.clone();

    let fields = named_fields(ast)?.named;

    let (nested_fields, dependent_fields) = partition_by_kind(&fields)?;

    let reactive_ident = Ident::new(format!("Reactive{}", ident).as_str(), ident.span());

    let nested_decls = nested_fields.iter().map(|&field| {
        let vis = field.vis.clone();
        let ident = field.ident.clone().expect("all fields should be named");
        let ty = field.ty.clone();

        let into_reactive_type = into_reactive_type.clone();
        quote! { #vis #ident: <#ty as #into_reactive_type>::Target }
    });
    let state_decls = dependent_fields.iter().map(|&field| {
        let vis = field.vis.clone();
        let ident = field.ident.clone().expect("all fields should be named");
        let ty = field.ty.clone();
        quote! { #vis #ident: #state_type<#ty> }
    });

    let reactive_struct_decl = quote! {
        #[derive(Clone)]
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
        let into_reactive_type = into_reactive_type.clone();
        quote! { #ident: <#ty as #into_reactive_type>::into_reactive(#ident, __runtime) }
    });
    let dependent_initializers = dependent_fields.iter().map(|&field| {
        let ident = field.ident.clone().expect("all fields should be named");
        quote! { #ident: __runtime.create_state(#ident) }
    });

    let into_reactive_impl = quote! {
        #[automatically_derived]
        impl #into_reactive_type for #ident {
            type Target = #reactive_ident;
            fn into_reactive(self, __runtime: &#runtime_type) -> Self::Target {
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
) -> Result<(Vec<&Field>, Vec<&Field>), Error> {
    let fields: Vec<_> = fields
        .iter()
        .map(|field| -> Result<_, Error> {
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

fn named_fields(ast: DeriveInput) -> Result<FieldsNamed, Error> {
    let data = match ast.data {
        Data::Struct(data) => Ok(data),
        Data::Enum(e) => Err(Error::spanned(
            e.enum_token,
            "derive(Reactive) is only supported on struct types.",
        )),
        Data::Union(u) => Err(Error::spanned(
            u.union_token,
            "derive(Reactive) is only supported on struct types.",
        )),
    }?;

    let fields = match data.fields {
        Fields::Named(fields) => Ok(fields),
        _ => Err(Error::spanned(
            data.fields,
            "derive(Reactive) is not supported on tuples.",
        )),
    }?;

    if fields.named.is_empty() {
        return Err(Error::spanned(
            fields,
            "derive(Reactive) is only supported on structs with one or more named fields.",
        ));
    }

    let mut reserved_idents = fields.named.iter().filter_map(|field| match &field.ident {
        Some(ident) if ident == "__runtime" => Some(ident),
        Some(_) => None,
        None => unreachable!("all fields should be named"),
    });
    if let Some(ident) = reserved_idents.next() {
        return Err(Error::spanned(
            ident,
            "derive(Reactive) uses this identifier internally, please use another one.",
        ));
    }

    let mut complex_field_types = fields.named.iter().filter_map(|field| match field.ty {
        Type::Path(_) => None,
        _ => Some(&field.ty),
    });
    if let Some(ty) = complex_field_types.next() {
        return Err(Error::spanned(
            ty,
            "derive(Reactive) only supports basic types.",
        ));
    }

    Ok(fields)
}

fn is_nested(field: &Field) -> Result<bool, Error> {
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
            .map_err(Error::Syn),
    }
}
