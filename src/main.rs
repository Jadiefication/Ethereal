use std::fs;
use std::io::Error;
use crate::storebuf::StoreBuf;

pub mod token;
pub mod span;
pub mod storebuf;

fn main() -> Result<(), Error> {
    let path = "examples/example.et";
    let buf = StoreBuf::open(path);

    if let Err(err) = buf {
        eprintln!("Error reading file '{}': {}", path, err)
    }

    println!("Hello, world!");
    Ok(())
}
