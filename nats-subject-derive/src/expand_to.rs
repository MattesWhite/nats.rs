use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    Attribute, DeriveInput, Expr, Ident, LitStr, Result, Token,
};

const WHITESPACE: [char; 4] = [' ', '\t', '\n', '\r'];

enum TemplateToken {
    Token(String),
    Field(Ident),
}

struct SubjectTemplate {
    span: Span,
    tokens: Vec<TemplateToken>,
}

impl SubjectTemplate {
    /// Construct a string literal for a format token.
    fn format_template(&self) -> LitStr {
        let mut format_template = String::new();
        let mut push_point = false;
        for token in self.tokens.iter() {
            if push_point {
                format_template.push('.');
            } else {
                push_point = true;
            }
            match token {
                TemplateToken::Token(token) => format_template.push_str(token),
                TemplateToken::Field(_) => format_template.push_str("{}"),
            }
        }
        LitStr::new(&format_template, self.span.clone())
    }
    fn format_args(&self) -> Punctuated<Expr, Token![,]> {
        let mut args = Punctuated::new();
        for token in self.tokens.iter() {
            if let TemplateToken::Field(ident) = token {
                args.push(parse_quote! { self.#ident });
            }
        }
        args
    }
}

impl Parse for SubjectTemplate {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        let subject_template: LitStr = input.parse()?;

        let mut tokens = Vec::new();
        let template = subject_template.value();
        if template.starts_with('.') || template.ends_with('.') {
            return Err(syn::Error::new(
                subject_template.span(),
                "The subject template does not represent a valid subject",
            ));
        }

        for token in template.split_terminator('.') {
            let token = match token {
                ident if ident.starts_with("[ ") && ident.ends_with(" ]") => {
                    let ident = ident[1..ident.len() - 2].trim();
                    if ident.contains(WHITESPACE) {
                        return Err(syn::Error::new(
                            subject_template.span(),
                            "Identifiers may not include whitespace",
                        ));
                    }
                    TemplateToken::Field(Ident::new(ident, subject_template.span()))
                }
                token => {
                    if token.contains(WHITESPACE) {
                        if token.starts_with('[') || token.ends_with(']') {
                            return Err(syn::Error::new(
                                subject_template.span(),
                                "Tokens may not include whitespace, did you intend to use a placeholder here?",
                            ));
                        }
                        return Err(syn::Error::new(
                            subject_template.span(),
                            "Tokens may not include whitespace",
                        ));
                    }
                    TemplateToken::Token(token.to_string())
                }
            };
            tokens.push(token);
        }

        Ok(Self { span, tokens })
    }
}

impl ToTokens for SubjectTemplate {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let format_template = self.format_template();
        let args = self.format_args();
        let subject_tokens = quote! { #format_template , #args };
        tokens.extend(subject_tokens);
    }
}

pub fn expand_derive_to_subject(input: &mut DeriveInput) -> Result<TokenStream> {
    let type_ident = &input.ident;
    let sub_attr = subject_attr(&input)?;
    let subject_template = sub_attr.parse_args::<SubjectTemplate>()?;

    Ok(quote! {
        impl ::async_nats::ToSubject for #type_ident {
            fn to_subject(&self) -> Result<::async_nats::SubjectBuf, ::async_nats::subject::Error> {
                ::async_nats::subj!(#subject_template)
            }
        }
    })
}

fn subject_attr(input: &DeriveInput) -> Result<&Attribute> {
    input
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("subject"))
        .ok_or_else(|| {
            syn::Error::new(
                Span::call_site(),
                "deriving ToSubject requires the #[subject(...)] attribute",
            )
        })
}
