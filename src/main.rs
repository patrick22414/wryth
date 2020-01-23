mod lexer;
mod token;

use std::fs::File;
use std::path::Path;
use lexer::Lexer;

fn main() {
    let path = Path::new("examples/example-1.wy");

    // open file
    if let Ok(file) = File::open(&path) {
        let mut lex = Lexer::new(file);

        println!("\nIteration 1");
        lex.lex();
        for token in lex.tokens() {
            println!("{:?}", token);
        }

        println!("\nIteration 2");
        lex.lex();
        for token in lex.tokens() {
            println!("{:?}", token);
        }
    }
}
