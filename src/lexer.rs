use std::fs::File;
use std::io::{BufReader, BufRead, Read};

use super::token::*;
use std::slice::Iter;

pub struct Lexer {
    _raw_text: String,
    _tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(file: &mut File) -> Lexer {
        let mut raw_text = String::new();
        file.read_to_string(&mut raw_text).expect("Lexer::new: cannot read file");

        Lexer {
            _raw_text: raw_text,
            _tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) {
        for line in self._raw_text.lines() {
            println!("'{}'", line.trim_end());
        }

        for i in 1..5 {
            let tk = Token { text: i.to_string(), kind: TokenKind::NumLiteral };
            self._tokens.push(tk);
        }
    }

    pub fn tokens(&self) -> Iter<Token> {
        (&self._tokens).into_iter()
    }
}
