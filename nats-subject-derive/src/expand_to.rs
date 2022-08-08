use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

use crate::subject_template::{subject_attr, SubjectTemplate};

pub fn expand_derive_to_subject(input: &mut DeriveInput) -> Result<TokenStream> {
    let type_ident = &input.ident;
    let sub_attr = subject_attr(&input)?;
    let subject_template = sub_attr.parse_args::<SubjectTemplate>()?;
    let format_string = subject_template.format_template();
    let format_args = subject_template.format_args();

    Ok(quote! {
        impl ::async_nats::ToSubject for #type_ident {
            fn to_subject(&self) -> Result<::async_nats::SubjectBuf, ::async_nats::subject::Error> {
                ::async_nats::subj!(#format_string, #format_args)
            }
        }
    })
}
