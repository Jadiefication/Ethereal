use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Skip whitespace automatically
#[logos(skip(r"//[^\n]*", allow_greedy = true))]   // Skip single-line comments
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
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Ident,

    #[regex(r"[0-9]+(?:[ui](?:8|16|32|64))?")]
    IntLit,

    #[regex(r"[0-9]+\.[0-9]+(?:f(?:32|64))?")]
    FloatLit,

    #[regex(r#""[^"\\]*(\\.[^"\\]*)*""#)]
    StringLit,
}