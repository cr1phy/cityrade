use proc_macro::TokenStream;
use syn::{spanned::Spanned, Attribute, Data, DeriveInput, Lit, Meta, MetaNameValue, Path};
use quote::quote;

#[proc_macro_derive(Building, attributes(building))]
pub fn building_derive(input: TokenStream) -> TokenStream {
    let ast = match syn::parse2::<DeriveInput>(input.into()) {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error().into(),
    };

    impl_building(&ast)
}

fn impl_building(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Найти атрибут #[building(...)]
    let building_attr = ast.attrs.iter().find(|attr| attr.path().is_ident("building"));

    let (title, description, effect) = match building_attr {
        Some(attr) => match parse_building_attr(attr) {
            Ok(values) => values,
            Err(err) => return err.to_compile_error().into(),
        },
        None => {
            return syn::Error::new_spanned(ast, "Expected #[building(...)] attribute")
                .to_compile_error()
                .into();
        }
    };

    // Генерация кода для реализации
    let gen = quote! {
        impl #name {
            pub fn title() -> &'static str {
                #title
            }

            pub fn description() -> &'static str {
                #description
            }

            pub fn effect() -> &'static str {
                #effect
            }
        }
    };

    gen.into()
}

fn parse_building_attr(attr: &Attribute) -> Result<(String, String, String), syn::Error> {
    let mut title = None;
    let mut description = None;
    let mut effect = None;

    attr.parse_nested_meta(|meta| {
        if meta.path.is_ident("title") {
            if let Lit::Str(lit_str) = meta.value()?.parse()? {
                title = Some(lit_str.value());
            } else {
                return Err(syn::Error::new_spanned(meta, "Expected a string literal for `title`"));
            }
        } else if meta.path.is_ident("description") {
            if let Lit::Str(lit_str) = meta.value()?.parse()? {
                description = Some(lit_str.value());
            } else {
                return Err(syn::Error::new_spanned(meta, "Expected a string literal for `description`"));
            }
        } else if meta.path.is_ident("effect") {
            if let Lit::Str(lit_str) = meta.value()?.parse()? {
                effect = Some(lit_str.value());
            } else {
                return Err(syn::Error::new_spanned(meta, "Expected a string literal for `effect`"));
            }
        }

        Ok(())
    })?;

    Ok((
        title.ok_or_else(|| syn::Error::new(attr.span(), "Missing `title` attribute"))?,
        description.ok_or_else(|| syn::Error::new(attr.span(), "Missing `description` attribute"))?,
        effect.ok_or_else(|| syn::Error::new(attr.span(), "Missing `effect` attribute"))?,
    ))
}