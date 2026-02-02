#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

/// The overflow behavior mode for `forward()` and `backward()` methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Mode {
    /// Wrap around from the last variant to the first (and vice versa).
    #[default]
    Wrap,
    /// Clamp at the boundaries (stay at first/last variant).
    Clamp,
}

fn parse_mode(input: &DeriveInput) -> Result<Mode, syn::Error> {
    let mut mode = Mode::default();

    for attr in &input.attrs {
        if attr.path().is_ident("rotalubat") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("mode") {
                    let value: syn::LitStr = meta.value()?.parse()?;
                    match value.value().as_str() {
                        "wrap" => mode = Mode::Wrap,
                        "clamp" => mode = Mode::Clamp,
                        other => {
                            return Err(syn::Error::new_spanned(
                                &value,
                                format!("unknown mode `{other}`, expected `wrap` or `clamp`"),
                            ));
                        }
                    }
                    Ok(())
                } else {
                    Err(syn::Error::new_spanned(
                        meta.path,
                        "unknown rotalubat attribute, expected `mode`",
                    ))
                }
            })?;
        }
    }

    Ok(mode)
}

#[proc_macro_derive(Rotalubat, attributes(rotalubat))]
pub fn rotalubat_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let mode = match parse_mode(&input) {
        Ok(m) => m,
        Err(e) => return e.to_compile_error().into(),
    };

    let Data::Enum(data) = &input.data else {
        return syn::Error::new_spanned(&input, "Rotalubat can only be derived for enums")
            .to_compile_error()
            .into();
    };

    for variant in &data.variants {
        let Fields::Unit = variant.fields else {
            return syn::Error::new_spanned(
                variant,
                "Rotalubat does not support enum variants with fields",
            )
            .to_compile_error()
            .into();
        };
    }

    let variants: Vec<&syn::Ident> = data.variants.iter().map(|v| &v.ident).collect();

    if variants.is_empty() {
        return syn::Error::new_spanned(&input, "Rotalubat requires at least one variant")
            .to_compile_error()
            .into();
    }

    let expanded = match mode {
        Mode::Wrap => generate_wrap_impl(name, &variants),
        Mode::Clamp => generate_clamp_impl(name, &variants),
    };

    TokenStream::from(expanded)
}

fn generate_wrap_impl(name: &syn::Ident, variants: &[&syn::Ident]) -> proc_macro2::TokenStream {
    let next_variants = variants.iter().cycle().skip(1);
    let forward_arms = variants.iter().zip(next_variants).map(|(current, next)| {
        quote! { #name::#current => #name::#next }
    });

    let prev_variants = variants.iter().cycle().skip(variants.len() - 1);
    let backward_arms = variants.iter().zip(prev_variants).map(|(current, prev)| {
        quote! { #name::#current => #name::#prev }
    });

    quote! {
        impl #name {
            #[inline]
            pub fn forward(&mut self) {
                *self = match self {
                    #(#forward_arms),*
                };
            }

            #[inline]
            pub fn backward(&mut self) {
                *self = match self {
                    #(#backward_arms),*
                };
            }
        }
    }
}

fn generate_clamp_impl(name: &syn::Ident, variants: &[&syn::Ident]) -> proc_macro2::TokenStream {
    let first = variants.first().unwrap();
    let last = variants.last().unwrap();

    // For forward: all variants map to next, except the last which stays
    let forward_arms = variants.windows(2).map(|pair| {
        let current = pair[0];
        let next = pair[1];
        quote! { #name::#current => #name::#next }
    });
    let forward_last = quote! { #name::#last => #name::#last };

    // For backward: all variants map to previous, except the first which stays
    let backward_arms = variants.windows(2).map(|pair| {
        let prev = pair[0];
        let current = pair[1];
        quote! { #name::#current => #name::#prev }
    });
    let backward_first = quote! { #name::#first => #name::#first };

    quote! {
        impl #name {
            #[inline]
            pub fn forward(&mut self) {
                *self = match self {
                    #(#forward_arms,)*
                    #forward_last
                };
            }

            #[inline]
            pub fn backward(&mut self) {
                *self = match self {
                    #backward_first,
                    #(#backward_arms),*
                };
            }
        }
    }
}
