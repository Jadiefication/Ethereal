use Language::spanned_token::SpannedToken;
use Language::storebuf::StoreBuf;
use Language::token::{Extras, Token};
use lasso::Rodeo;
use logos::Logos;
use std::env;
use std::io::Error;

fn main() -> Result<(), Error> {
    let path = env::args().nth(1).unwrap();
    let buf = StoreBuf::open(&path);

    if let Err(err) = &buf {
        eprintln!("Error reading file '{}': {}", path, err)
    } else if let Ok(buffer) = buf {
        let mut tokens: Vec<SpannedToken> = Vec::new();
        let mut lexer = Token::lexer_with_extras(
            buffer.text(),
            Extras {
                rodeo: Rodeo::default(),
            },
        );

        while let Some(raw_token) = lexer.next() {
            let span = lexer.span();
            match raw_token {
                Ok(token) => tokens.push(SpannedToken { token, span }),
                Err(err) => eprintln!("Error {:?} at {}, {} for {path}", err, span.start, span.end),
            }
        }

        let eof_span = if let Some(last_token) = tokens.last() {
            last_token.span.end..last_token.span.end
        } else {
            0..0
        };

        tokens.push(SpannedToken {
            token: Token::Eof,
            span: eof_span,
        });
    }

    println!("Hello, world!");
    Ok(())
}
