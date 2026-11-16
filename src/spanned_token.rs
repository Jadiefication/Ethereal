use crate::token::Token;
use logos::Span;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SpannedToken {
    pub token: Token,
    pub span: Span,
}
