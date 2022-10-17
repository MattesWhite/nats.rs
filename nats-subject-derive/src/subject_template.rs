use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    Attribute, DeriveInput, Expr, Ident, LitStr, Result, Token,
};

const WHITESPACE: [char; 4] = [' ', '\t', '\n', '\r'];

#[derive(Debug, PartialEq, Eq)]
pub enum TemplateToken {
    Token(String),
    MultiField(Ident),
    SingleField(Ident),
}

pub struct SubjectTemplate {
    span: Span,
    tokens: Vec<TemplateToken>,
}

pub fn subject_attr(input: &DeriveInput) -> Result<&Attribute> {
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

impl SubjectTemplate {
    pub fn tokens(&self) -> &[TemplateToken] {
        &self.tokens
    }
    /// Construct a string literal for the format macro.
    pub fn format_template(&self) -> LitStr {
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
                TemplateToken::MultiField(_) | TemplateToken::SingleField(_) => {
                    format_template.push_str("{}")
                }
            }
        }
        LitStr::new(&format_template, self.span)
    }
    pub fn format_args(&self) -> Punctuated<Expr, Token![,]> {
        self.tokens()
            .iter()
            .filter_map(|t| match t {
                TemplateToken::Token(_) => None,
                TemplateToken::MultiField(ident) | TemplateToken::SingleField(ident) => {
                    let expr: Expr = parse_quote!( self.#ident );
                    Some(expr)
                }
            })
            .collect()
    }
    pub fn fields(&self) -> Punctuated<Ident, Token![,]> {
        self.tokens()
            .iter()
            .filter_map(|t| match t {
                TemplateToken::Token(_) => None,
                TemplateToken::MultiField(ident) | TemplateToken::SingleField(ident) => {
                    Some(ident.clone())
                }
            })
            .collect()
    }
}

impl Parse for SubjectTemplate {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        let subject_template: LitStr = input.parse()?;

        let tokens =
            parse_subject_template_literal(&subject_template.value(), subject_template.span())?;

        Ok(Self { span, tokens })
    }
}

fn valid_token(token: &str, span: Span) -> Result<()> {
    if token.contains(WHITESPACE) {
        Err(syn::Error::new(
            span,
            "Identifiers may not include whitespace",
        ))
    } else {
        Ok(())
    }
}

fn parse_subject_template_literal(template: &str, span: Span) -> Result<Vec<TemplateToken>> {
    let mut tokens = Vec::new();
    if template.is_empty() {
        return Err(syn::Error::new(span, "Empty subjects are not valid"));
    }
    if template.starts_with('.') || template.ends_with('.') {
        return Err(syn::Error::new(
            span,
            "The subject template does not represent a valid subject",
        ));
    }

    for token in template.split_terminator('.') {
        let token = match token {
            ident if ident.starts_with("[ ") && ident.ends_with(" ]") => {
                let ident = ident[1..ident.len() - 2].trim();
                match ident {
                    ident if ident.starts_with('>') => {
                        let ident = ident[1..].trim();
                        valid_token(ident, span)?;
                        TemplateToken::MultiField(Ident::new(ident, span))
                    }
                    ident if ident.starts_with('*') => {
                        let ident = ident[1..].trim();
                        valid_token(ident, span)?;
                        TemplateToken::SingleField(Ident::new(ident, span))
                    }
                    ident => {
                        valid_token(ident, span)?;
                        TemplateToken::SingleField(Ident::new(ident, span))
                    }
                }
            }
            token => {
                valid_token(token, span)?;
                TemplateToken::Token(token.to_string())
            }
        };
        tokens.push(token);
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_leading_multi_wildcard_token() {
        let template = "[ > prefix ].api.[ number ].[ > rest ]";
        let tokens = parse_subject_template_literal(template, Span::call_site()).unwrap();
        assert_eq!(tokens.len(), 4);
        assert!(
            matches!(&tokens[0], TemplateToken::MultiField(ident) if ident.to_string() == "prefix")
        );
        assert!(matches!(&tokens[1], TemplateToken::Token(token) if token == "api"));
        assert!(
            matches!(&tokens[2], TemplateToken::SingleField(ident) if ident.to_string() == "number")
        );
        assert!(
            matches!(&tokens[3], TemplateToken::MultiField(ident) if ident.to_string() == "rest")
        );
    }
}
