use std::env;
use crate::storebuf::StoreBuf;
use std::io::Error;
use lasso::Rodeo;
use logos::Logos;
use crate::token::{Extras, Token};

pub mod token;
pub mod span;
pub mod storebuf;
pub mod num;

fn main() -> Result<(), Error> {
    let path = env::args().nth(1).unwrap();
    let buf = StoreBuf::open(&path);

    if let Err(err) = &buf {
        eprintln!("Error reading file '{}': {}", path, err)
    } else if let Ok(buffer) = buf {
        let tokens: Vec<Token> = Token::lexer_with_extras(
            buffer.text(), Extras { rodeo: Rodeo::default() }
        )
            .collect::<Result<_, _>>().unwrap();
        dbg!(tokens);
    }


    println!("Hello, world!");
    Ok(())
}
