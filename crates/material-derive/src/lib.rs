#![warn(
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style,
    clippy::pedantic
)]

use std::fmt::Write;

use heck::ToShoutySnakeCase as _;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;
use syn::token::Comma;
use syn::{
    Data, DeriveInput, Ident, Path, Token, Variant, bracketed, parenthesized, parse_macro_input,
};

#[proc_macro_derive(GenerateMaterials, attributes(material))]
pub fn material(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = input.ident.clone();

    let Data::Enum(data) = input.data else {
        return syn::Error::new(input.span(), "macro can only be derived for `enum`s")
            .into_compile_error()
            .into();
    };

    match Materials::from_variants(enum_name, data.variants) {
        Ok(m) => quote! { #m }.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

struct Materials {
    enum_name: Ident,
    entries: Vec<Entry>,
}

impl Materials {
    fn from_variants(enum_name: Ident, variants: Punctuated<Variant, Comma>) -> syn::Result<Self> {
        let mut entries = Vec::<Entry>::new();

        for variant in variants {
            let family = variant.ident;

            let mut items = Vec::<Item>::new();

            for attr in &variant.attrs {
                if !attr.path().is_ident("material") {
                    // Silently skip unknown attributes as they may belong to other libraries.
                    continue;
                }

                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("items") {
                        let value = meta.value()?;

                        let content;
                        bracketed!(content in value);

                        items = content
                            .parse_terminated(Item::parse, Token![,])?
                            .into_iter()
                            .collect();

                        Ok(())
                    } else if meta.path.is_ident("category") {
                        // TODO: Only allow category if items is not defined.
                        // Also, require if items is not defined.
                        Ok(())
                    } else {
                        Err(meta.error("unsupported attribute"))
                    }
                })?;
            }

            entries.push(Entry { family, items });
        }

        Ok(Self { enum_name, entries })
    }
}

impl ToTokens for Materials {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let entries = self
            .entries
            .iter()
            .flat_map(|entry| {
                let enum_name = &self.enum_name;
                let family = &entry.family;

                entry.items.iter().map(move |item| {
                    let category = &item.category;
                    let shape = &item.shape;
                    let modifiers = &item.modifiers;

                    let modifier_id = item
                        .modifiers
                        .iter()
                        .map(|m| {
                            m.segments
                                .last()
                                .unwrap()
                                .ident
                                .to_string()
                                .to_shouty_snake_case()
                        })
                        .collect::<Vec<String>>()
                        .join("_");
                    let family_id = family.to_string().to_shouty_snake_case();
                    let mut shape_id = item
                        .shape
                        .segments
                        .last()
                        .unwrap()
                        .ident
                        .to_string()
                        .to_shouty_snake_case();

                    // We cannot use numbers for enum variants, so we wrote them in English
                    // and need to overwrite them here. These are music discs with numeric names.
                    if shape_id == "THIRTEEN" {
                        shape_id = "13".to_string();
                    } else if shape_id == "ELEVEN" {
                        shape_id = "11".to_string();
                    } else if shape_id == "FIVE" {
                        shape_id = "5".to_string();
                    }

                    let mut material_id = String::new();

                    if !modifier_id.is_empty() {
                        write!(material_id, "{modifier_id}_")
                            .expect("failed to prepend modifier id");
                    }

                    material_id.push_str(&family_id);

                    if shape_id != "BASE" {
                        // Some families are plural when alone, but when attached to a shape
                        // become singular. The shape is plural instead. (E.g., `DEEPSLATE_BRICKS`
                        // and `DEEPSLATE_BRICK_SLAB`.)
                        if material_id.ends_with("BRICKS") {
                            material_id = material_id.replace("BRICKS", "BRICK");
                        } else if material_id.ends_with("TILES") {
                            material_id = material_id.replace("TILES", "TILE");
                        }

                        write!(material_id, "_{shape_id}").expect("failed to append shape id");
                    }

                    quote! {
                        (#material_id, crate::material::Material {
                            category: #category,
                            family: #enum_name::#family,
                            shape: #shape,
                            modifiers: &[#(#modifiers),*],
                        })
                    }
                })
            })
            .collect::<Vec<_>>();

        tokens.extend(quote! {
            pub static MATERIALS: std::sync::LazyLock<std::collections::HashMap<&'static str, crate::material::Material>> = std::sync::LazyLock::new(|| {
                std::collections::HashMap::from_iter([
                    #(#entries),*
                ])
            });
        });
    }
}

struct Item {
    category: Path,
    shape: Path,
    modifiers: Vec<Path>,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

        let category = content.parse()?;
        _ = content.parse::<Token![,]>()?;
        let shape = content.parse()?;
        _ = content.parse::<Token![,]>()?;

        Ok(Self {
            category,
            shape,
            modifiers: {
                let c;
                bracketed!(c in content);

                c.parse_terminated(Path::parse, Token![,])?
                    .into_iter()
                    .collect()
            },
        })
    }
}

struct Entry {
    family: Ident,
    items: Vec<Item>,
}
