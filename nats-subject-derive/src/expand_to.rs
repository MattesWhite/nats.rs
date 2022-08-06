use proc_macro2::{TokenStream, Span};
use quote::{quote, ToTokens};
use syn::{DeriveInput, LitStr, Attribute, Result, Ident, parse::{Parse, ParseStream}, punctuated::Punctuated, Token};

struct SubjectTemplate {
    format_template: LitStr,
    arguments: Option<Punctuated<TokenStream, Token![,]>>,
}

impl Parse for SubjectTemplate {
    fn parse(input: ParseStream) -> Result<Self> {
        let format_template = input.parse()?;
        let arguments = 
        if let Ok(_) = input.parse::<Token![,]>() {
            let args = Punctuated::<Ident, Token![,]>::parse_separated_nonempty(input)?;
            Some(args.iter().map(|id| quote!{ self.#id }).collect())
        } else {
            None
        };
        Ok(Self {format_template, arguments})
    }
}

impl ToTokens for SubjectTemplate {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let format_template = &self.format_template;
        let subject_tokens = if let Some(args) = &self.arguments {
            quote! { #format_template , #args }
        } else {
            quote! { #format_template }
        };
        tokens.extend(subject_tokens);
    }
}

pub fn expand_derive_to_subject(input: &mut DeriveInput) -> Result<TokenStream> {
    let type_ident = &input.ident;
    let sub_attr = subject_attr(&input)?;
    let subject_template = sub_attr
        .parse_args::<SubjectTemplate>()?;

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
    .ok_or_else(|| syn::Error::new(Span::call_site(), "deriving ToSubject requires the #[subject(...)] attribute"))
}
