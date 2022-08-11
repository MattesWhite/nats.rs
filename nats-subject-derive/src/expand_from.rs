use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

use crate::subject_template::{subject_attr, SubjectTemplate, TemplateToken};

pub fn expand_derive_from_subject(input: &mut DeriveInput) -> Result<TokenStream> {
    let type_ident = &input.ident;
    let sub_attr = subject_attr(&input)?;
    let subject_template = sub_attr.parse_args::<SubjectTemplate>()?;

    let mut token_checks = TokenStream::new();
    let mut tokens = subject_template.tokens().iter();
    let mut current_token = tokens
        .next()
        .expect("ensured by SubjectTemplate constructor");
    let mut next_token = tokens.next();
    loop {
        let check = check_or_parse(current_token, next_token)?;
        token_checks.extend(check);
        if let Some(token) = next_token {
            current_token = token;
            next_token = tokens.next();
        } else {
            break;
        }
    }

    let fields = subject_template.fields();
    Ok(quote! {
        impl ::async_nats::subject::FromSubject for #type_ident {
            fn from_subject(subject: &::async_nats::Subject) -> Result<Self, ::async_nats::subject::FromSubjectError> {
                let mut idx = 0;

                #token_checks

                Ok(Self { #fields } )
            }
        }
    })
}

fn check_or_parse(token: &TemplateToken, next: Option<&TemplateToken>) -> Result<TokenStream> {
    let idx_and_sub = match (token, next) {
        (
            TemplateToken::MultiField(ident),
            Some(TemplateToken::MultiField(_) | TemplateToken::SingleField(_)),
        ) => {
            return Err(syn::Error::new(
                ident.span(),
                "Multi-field placeholders next to each other are indistinguishable",
            ));
        }
        (TemplateToken::MultiField(_), Some(TemplateToken::Token(token))) => {
            let pattern = format!(".{token}.");
            quote! {
                idx = subject.rfind(#pattern).ok_or_else(|| ::async_nats::subject::FromSubjectError::SubjectEndedUnexpected {
                    wanted: #token.to_string(),
                })?;
                let sub = &subject[..idx];
            }
        }
        (TemplateToken::SingleField(_) | TemplateToken::Token(_), Some(_)) => {
            quote! {
                idx = subject
                    .rfind('.')
                    .ok_or_else(|| ::async_nats::subject::FromSubjectError::SubjectEndedUnexpected {
                        wanted: ".".to_string(),
                    })?;
                let sub = &subject[..idx];
            }
        }
        (_, None) => {
            quote! {
                let sub = subject;
            }
        }
    };
    let parse_or_check = match token {
        TemplateToken::Token(t) => {
            quote! {
                if sub != #t {
                    return Err(::async_nats::subject::FromSubjectError::TokenMismatch {
                        expected: #t.to_string(),
                        got: sub.to_string(),
                    });
                }
            }
        }
        TemplateToken::MultiField(ident) | TemplateToken::SingleField(ident) => {
            quote! {
                let #ident = sub
                    .parse()
                    .map_err(|e| FromSubjectError::parser_err(e, stringify!(#ident), sub))?;
            }
        }
    };
    let forward_subject = if let Some(_) = next {
        quote! {
            let subject = &subject[idx + 1..];
        }
    } else {
        quote! {}
    };

    Ok(quote! {
        #idx_and_sub
        #parse_or_check
        #forward_subject
    })
}
