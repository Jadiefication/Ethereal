use crate::num::float::{FloatLiteral, FloatSuffix};
use crate::num::int::IntLiteral;
use crate::num::int::IntSuffix;
use lasso::{Rodeo, Spur};
use logos::Logos;

pub struct Extras {
    pub rodeo: Rodeo,
}

#[derive(Logos, Debug, Clone, PartialEq, Default)]
#[logos(skip r"[ \t\n\f]+")] // Skip whitespace automatically
#[logos(skip(r"//[^\n]*", allow_greedy = true))] // Skip single-line comments
#[logos(extras = Extras)]
pub enum Token {
    // Keywords
    #[token("val")]
    Val,
    #[token("var")]
    Var,
    #[token("kernel")]
    Kernel,
    #[token("fn")]
    Fn,
    #[token("impl")]
    Impl,
    #[token("struct")]
    Struct,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("import")]
    Import,
    #[token("pub")]
    Pub,
    #[token("internal")]
    Internal,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("as")]
    As,
    #[token("this")]
    This,
    #[token("static")]
    Static,

    // Prefixes & Operators
    #[token("#")]
    Hash,
    #[token("@")]
    At,
    #[token("::")]
    PathSep,
    #[token("|>")]
    PipeAssign,
    #[token("|")]
    SinglePipe,
    #[token("=")]
    Assign,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("..")]
    Range,

    // Comparison & Logic
    #[token("==")]
    Eq,
    #[token("!=")]
    Neq,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("<=")]
    Le,
    #[token(">=")]
    Ge,
    #[token("&&")]
    And,
    #[token("||")]
    Or,
    #[token("!")]
    Bang,

    // Delimiters
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    Semicolon,

    // Identifiers & Literals
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| {
        lex.extras.rodeo.get_or_intern(lex.slice())
    })]
    Ident(Spur),

    #[regex(r"[0-9]+(?:[ui](?:8|16|32|64))?", |lex| {
        let slice = lex.slice();
        let end = slice.find(|c: char| !c.is_ascii_digit()).unwrap_or(slice.len());

        let value = slice[..end].parse::<i64>().unwrap();

        let suffix = match &slice[end..] {
            "u8"  => IntSuffix::U8,
            "u16" => IntSuffix::U16,
            "u32" => IntSuffix::U32,
            "u64" => IntSuffix::U64,
            "i8"  => IntSuffix::I8,
            "i16" => IntSuffix::I16,
            "i32" => IntSuffix::I32,
            "i64" => IntSuffix::I64,
            _     => IntSuffix::None,
        };

        Some(IntLiteral { value, suffix })
    })]
    IntLit(IntLiteral),

    #[regex(r"[0-9]+\.[0-9]+(?:f(?:32|64))?", |lex| {
        let slice = lex.slice();
        let end = slice.find('f').unwrap_or(slice.len());

        let value = slice[..end].parse::<f64>().unwrap();

        let suffix = match &slice[end..] {
            "f32" => FloatSuffix::F32,
            "f64" => FloatSuffix::F64,
            _     => FloatSuffix::None,
        };

        Some(FloatLiteral { value, suffix })
    })]
    FloatLit(FloatLiteral),

    #[regex(r#""[^"\\]*(\\.[^"\\]*)*""#, |lex| {
        lex.extras.rodeo.get_or_intern(lex.slice())
    })]
    StringLit(Spur),
    
    #[default]
    Eof
}
