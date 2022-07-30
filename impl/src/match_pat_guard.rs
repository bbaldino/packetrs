use proc_macro2::TokenStream;
use syn::Token;

/// MatchPatGuard represents the tokens of a match arm excluding the body.  (syn only defines
/// 'Arm', which includes an entire match arm).
#[derive(Debug, Clone)]
pub(crate) struct MatchPatGuard {
    pub(crate) pat: syn::Pat,
    pub(crate) guard: Option<(syn::token::If, Box<syn::Expr>)>,
}

impl syn::parse::Parse for MatchPatGuard {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(MatchPatGuard {
            pat: input.parse()?,
            guard: {
                if input.peek(Token![if]) {
                    let if_token: Token![if] = input.parse()?;
                    let guard: syn::Expr = input.parse()?;
                    Some((if_token, Box::new(guard)))
                } else {
                    None
                }
            },
        })
    }
}

impl quote::ToTokens for MatchPatGuard {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pat.to_tokens(tokens);
        if let Some((if_token, guard)) = &self.guard {
            if_token.to_tokens(tokens);
            guard.to_tokens(tokens);
        }
    }
}
