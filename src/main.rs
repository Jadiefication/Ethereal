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
        let tokens: Vec<Token> = Token::lexer_with_extras(
            buffer.text(),
            Extras {
                rodeo: Rodeo::default(),
            },
        )
        .collect::<Result<_, _>>()
        .unwrap();
        dbg!(tokens);
    }

    println!("Hello, world!");
    Ok(())
}
