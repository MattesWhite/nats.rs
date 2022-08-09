use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod expand_from;
mod expand_to;
pub(crate) mod subject_template;

#[proc_macro_derive(ToSubject, attributes(subject))]
pub fn derive_to_subject(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    expand_to::expand_derive_to_subject(&mut input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[proc_macro_derive(FromSubject, attributes(subject))]
pub fn derive_from_subject(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    expand_from::expand_derive_from_subject(&mut input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}
