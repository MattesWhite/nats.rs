use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, LitStr, Result};

use crate::subject_template::{subject_attr, SubjectTemplate, TemplateToken};

pub fn expand_derive_from_subject(input: &mut DeriveInput) -> Result<TokenStream> {
    let type_ident = &input.ident;
    let sub_attr = subject_attr(&input)?;
    let subject_template = sub_attr.parse_args::<SubjectTemplate>()?;
    let token_cnt = subject_template.tokens().len();
    let fields = subject_template.fields();

    let mut token_checks = TokenStream::new();
    let mut first = true;
    for token in subject_template.tokens().iter() {
        let check_stream = token_check(first, token, token_cnt)?;
        token_checks.extend(check_stream);
        first = false;
    }

    Ok(quote! {
        impl ::async_nats::subject::FromSubject for #type_ident {
            fn from_subject(subject: &::async_nats::Subject) -> Result<Self, ::async_nats::subject::FromSubjectError> {
                let mut tokens = subject.tokens();
                let mut token_cnt = 0;

                #token_checks

                Ok(Self { #fields } )
            }
        }
    })
}

fn token_check(first: bool, token: &TemplateToken, token_cnt: usize) -> Result<TokenStream> {
    let mut check = TokenStream::new();

    if !first {
        // increase token counter
        check.extend(quote! { token_cnt += 1; });
    }
    // Get the current token
    check.extend(quote! {
        let cur_token = tokens.next().ok_or(::async_nats::subject::FromSubjectError::ExpectedMoreTokens {
            expected: #token_cnt,
            got: token_cnt,
        })?;
    });
    // Parse or check token
    match token {
        TemplateToken::Token(check_token) => {
            let check_token = LitStr::new(&check_token, Span::call_site());
            check.extend(quote! {
                if cur_token != #check_token {
                    return Err(::async_nats::subject::FromSubjectError::TokenMismatch {
                        expected: #check_token.to_string(),
                        got: cur_token.to_string(),
                    });
                }
            });
        }
        TemplateToken::Field(field) => {
            check.extend(quote! {
                let #field = cur_token
                    .parse()
                    .map_err(|e| ::async_nats::subject::FromSubjectError::parser_err(e, stringify!(#field), cur_token))?;
            })
        },
    }

    Ok(check)
}
