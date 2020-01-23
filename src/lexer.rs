use std::fs::File;
use std::io::BufReader;

use super::token::*;
use std::slice::Iter;

pub struct Lexer {
    _buf: BufReader<File>,
    _tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(file: File) -> Lexer {
        Lexer {
            _buf: BufReader::new(file),
            _tokens: Vec::new(),
        }
    }

    pub fn lex(&mut self) {
        for i in 1..5 {
            let tk = Token { text: i.to_string(), kind: TokenKind::NumLiteral };
            self._tokens.push(tk);
        }
    }

    pub fn tokens(&self) -> Iter<Token> {
        (&self._tokens).into_iter()
    }
}
