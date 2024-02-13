//! CRust, Rust utilities for when dealing with C is inevitable

#![deny(missing_debug_implementations, missing_docs)]
#![warn(unused)]

use proc_macro::TokenStream as T;

/// A macro to generate Rust bindings for C libraries
#[proc_macro]
pub fn include(input: T) -> T {
    let input_files = syn::parse_macro_input!(input as Files);
    let mut builder = bindgen::Builder::default()
        .default_visibility(bindgen::FieldVisibilityKind::Private)
        .use_core();
    for input_file in input_files.0 {
        builder = builder.header(input_file);
    }
    let bindings = match builder.generate() {
        Err(e) => return syn::Error::new(proc_macro2::Span::call_site(), e)
            .to_compile_error().into(),
        Ok(b) => b,
    };
    bindings.to_string().parse().unwrap()
}

struct Files(Vec<String>);

impl syn::parse::Parse for Files {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut files = Vec::new();
        while !input.is_empty() {
            files.push(input.parse::<syn::LitStr>()?.value());
            if input.peek(syn::Token![,]) {
                input.parse::<syn::Token![,]>()?;
            }
        }
        Ok(Self(files))
    }
}
