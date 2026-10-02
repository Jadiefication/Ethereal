use lasso::Rodeo;
use logos::Logos;
use Language::num::float::{FloatLiteral, FloatSuffix};
use Language::num::int::{IntLiteral, IntSuffix};
use Language::token::{Extras, Token};

fn lex(input: &str) -> (Vec<Token>, Rodeo) {
    let mut lex = Token::lexer_with_extras(input, Extras { rodeo: Rodeo::default() });
    let mut tokens = Vec::new();
    while let Some(res) = lex.next() {
        tokens.push(res.expect("unexpected token lexing error"));
    }
    (tokens, lex.extras.rodeo)
}

fn lex_raw(input: &str) -> Vec<Result<Token, ()>> {
    let lexer = Token::lexer_with_extras(input, Extras { rodeo: Rodeo::default() });
    lexer.collect()
}

#[test]
fn test_keywords_tokenization() {
    let input = "val var kernel fn impl struct if else while for in import pub internal true false";
    let (tokens, _) = lex(input);

    assert_eq!(
        tokens,
        vec![
            Token::Val,
            Token::Var,
            Token::Kernel,
            Token::Fn,
            Token::Impl,
            Token::Struct,
            Token::If,
            Token::Else,
            Token::While,
            Token::For,
            Token::In,
            Token::Import,
            Token::Pub,
            Token::Internal,
            Token::True,
            Token::False,
        ]
    );
}

#[test]
fn test_prefixes_and_operators() {
    let input = "# @ :: |> | = + - * / % ..";
    let (tokens, _) = lex(input);

    assert_eq!(
        tokens,
        vec![
            Token::Hash,
            Token::At,
            Token::PathSep,
            Token::PipeAssign,
            Token::SinglePipe,
            Token::Assign,
            Token::Plus,
            Token::Minus,
            Token::Star,
            Token::Slash,
            Token::Percent,
            Token::Range,
        ]
    );
}

#[test]
fn test_comparison_and_logic() {
    let input = "== != < > && || !";
    let (tokens, _) = lex(input);

    assert_eq!(
        tokens,
        vec![
            Token::Eq,
            Token::Neq,
            Token::Lt,
            Token::Gt,
            Token::And,
            Token::Or,
            Token::Bang,
        ]
    );
}

#[test]
fn test_delimiters() {
    let input = "( ) { } [ ] . , : ;";
    let (tokens, _) = lex(input);

    assert_eq!(
        tokens,
        vec![
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::LBracket,
            Token::RBracket,
            Token::Dot,
            Token::Comma,
            Token::Colon,
            Token::Semicolon,
        ]
    );
}

#[test]
fn test_identifier_interning() {
    let input = "hello world _foo bar123 hello";
    let (tokens, rodeo) = lex(input);

    assert_eq!(tokens.len(), 5);

    let idents: Vec<&str> = tokens
        .iter()
        .map(|tok| match tok {
            Token::Ident(spur) => rodeo.resolve(spur),
            _ => panic!("expected Ident token"),
        })
        .collect();

    assert_eq!(idents, vec!["hello", "world", "_foo", "bar123", "hello"]);
    // Check that repeated identifier "hello" resolves to the exact same Spur
    assert_eq!(tokens[0], tokens[4]);
}

#[test]
fn test_integer_literals_with_all_suffixes() {
    let input = "42 255u8 65535u16 4294967295u32 1000u64 127i8 32767i16 2147483647i32 9999i64";
    let (tokens, _) = lex(input);

    assert_eq!(
        tokens,
        vec![
            Token::IntLit(IntLiteral { value: 42, suffix: IntSuffix::None }),
            Token::IntLit(IntLiteral { value: 255, suffix: IntSuffix::U8 }),
            Token::IntLit(IntLiteral { value: 65535, suffix: IntSuffix::U16 }),
            Token::IntLit(IntLiteral { value: 4294967295, suffix: IntSuffix::U32 }),
            Token::IntLit(IntLiteral { value: 1000, suffix: IntSuffix::U64 }),
            Token::IntLit(IntLiteral { value: 127, suffix: IntSuffix::I8 }),
            Token::IntLit(IntLiteral { value: 32767, suffix: IntSuffix::I16 }),
            Token::IntLit(IntLiteral { value: 2147483647, suffix: IntSuffix::I32 }),
            Token::IntLit(IntLiteral { value: 9999, suffix: IntSuffix::I64 }),
        ]
    );
}

#[test]
fn test_float_literals_with_suffixes() {
    let input = "3.14 1.5f32 2.0f64 0.0";
    let (tokens, _) = lex(input);

    assert_eq!(
        tokens,
        vec![
            Token::FloatLit(FloatLiteral { value: 3.14, suffix: FloatSuffix::None }),
            Token::FloatLit(FloatLiteral { value: 1.5, suffix: FloatSuffix::F32 }),
            Token::FloatLit(FloatLiteral { value: 2.0, suffix: FloatSuffix::F64 }),
            Token::FloatLit(FloatLiteral { value: 0.0, suffix: FloatSuffix::None }),
        ]
    );
}

#[test]
fn test_string_literals() {
    let input = r#""hello" "with \"escaped\" quote" "line 1\nline 2" """#;
    let (tokens, rodeo) = lex(input);

    assert_eq!(tokens.len(), 4);

    let strings: Vec<&str> = tokens
        .iter()
        .map(|tok| match tok {
            Token::StringLit(spur) => rodeo.resolve(spur),
            _ => panic!("expected StringLit token"),
        })
        .collect();

    assert_eq!(strings[0], r#""hello""#);
    assert_eq!(strings[1], r#""with \"escaped\" quote""#);
    assert_eq!(strings[2], r#""line 1\nline 2""#);
    assert_eq!(strings[3], r#""""#);
}

#[test]
fn test_skipping_whitespace_and_comments() {
    let input = "  \t\n  // this is a single line comment\n  val // another comment\n x = 10; \n // trailing comment";
    let (tokens, rodeo) = lex(input);

    assert_eq!(tokens.len(), 5);
    assert_eq!(tokens[0], Token::Val);
    match tokens[1] {
        Token::Ident(spur) => assert_eq!(rodeo.resolve(&spur), "x"),
        _ => panic!("expected Ident"),
    }
    assert_eq!(tokens[2], Token::Assign);
    assert_eq!(tokens[3], Token::IntLit(IntLiteral { value: 10, suffix: IntSuffix::None }));
    assert_eq!(tokens[4], Token::Semicolon);
}

#[test]
fn test_empty_input() {
    let input = "";
    let (tokens, _) = lex(input);
    assert!(tokens.is_empty());

    let whitespace_only = "   \t\n\n\t  ";
    let (tokens_ws, _) = lex(whitespace_only);
    assert!(tokens_ws.is_empty());
}

#[test]
fn test_invalid_tokens_error_handling() {
    let input = "val $invalid = `test`;";
    let raw_tokens = lex_raw(input);

    let has_error = raw_tokens.iter().any(|res| res.is_err());
    assert!(has_error, "lexer should report errors for invalid characters like $ or `");
}

#[test]
fn test_complex_expression_and_piping() {
    let input = "pair |> pair.multiply_elements()";
    let (tokens, rodeo) = lex(input);

    assert_eq!(tokens.len(), 7);
    match &tokens[0] {
        Token::Ident(s) => assert_eq!(rodeo.resolve(s), "pair"),
        _ => panic!("expected ident"),
    }
    assert_eq!(tokens[1], Token::PipeAssign);
    match &tokens[2] {
        Token::Ident(s) => assert_eq!(rodeo.resolve(s), "pair"),
        _ => panic!("expected ident"),
    }
    assert_eq!(tokens[3], Token::Dot);
    match &tokens[4] {
        Token::Ident(s) => assert_eq!(rodeo.resolve(s), "multiply_elements"),
        _ => panic!("expected ident"),
    }
    assert_eq!(tokens[5], Token::LParen);
    assert_eq!(tokens[6], Token::RParen);
}

#[test]
fn test_struct_and_function_definition_lexing() {
    let input = r#"
        struct #VectorPair {
            a: @gpu::f64,
            b: @gpu::f64,
        }
        pub fn main() {
            val N = 1048576u32;
        }
    "#;
    let (tokens, _) = lex(input);
    assert!(!tokens.is_empty());
    assert_eq!(tokens[0], Token::Struct);
    assert_eq!(tokens[1], Token::Hash);
    // Ends with RBrace
    assert_eq!(tokens[tokens.len() - 1], Token::RBrace);
}
