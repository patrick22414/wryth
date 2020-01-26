#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod lexer;
mod token;

use std::fs::File;
use std::path::Path;
use lexer::Lexer;

fn main() {
    let path = Path::new("examples/example-1.wy");

    if let Ok(mut file) = File::open(&path) {
        let mut lex = Lexer::new(&mut file);

        lex.lex();

        for token in lex.tokens() {
            println!("{:?}", token);
        }
    }
}
