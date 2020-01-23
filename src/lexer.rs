use std::fs::File;
use std::io::BufReader;

use super::token;

struct Lexer {}

impl Lexer {
    pub fn new(&self, file: &File) -> Lexer {
        let buf = BufReader::new(&file);
    }
}