#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, spanned::Spanned};

#[proc_macro_derive(Rotalubat)]
pub fn rotalubat_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Enum(data) = &input.data else {
        return syn::Error::new(
            input.ident.span(),
            "Rotalubat can only be derived for enums",
        )
        .to_compile_error()
        .into();
    };

    for variant in &data.variants {
        let Fields::Unit = variant.fields else {
            return syn::Error::new(
                variant.span(),
                "Rotalubat does not support enum variants with fields",
            )
            .to_compile_error()
            .into();
        };
    }

    let variants: Vec<&syn::Ident> = data.variants.iter().map(|v| &v.ident).collect();

    if variants.is_empty() {
        return syn::Error::new(
            input.ident.span(),
            "Rotalubat requires at least one variant",
        )
        .to_compile_error()
        .into();
    }

    let forward_arms =
        variants
            .iter()
            .zip(variants.iter().cycle().skip(1))
            .map(|(current, next)| {
                quote! { #name::#current => #name::#next }
            });

    let backward_arms = variants
        .iter()
        .zip(variants.iter().cycle().skip(variants.len() - 1))
        .map(|(current, prev)| {
            quote! { #name::#current => #name::#prev }
        });

    let expanded = quote! {
        impl #name {
            fn forward(&mut self) {
                *self = match self {
                    #(#forward_arms),*
                };
            }

            fn backward(&mut self) {
                *self = match self {
                    #(#backward_arms),*
                };
            }
        }
    };

    TokenStream::from(expanded)
}
