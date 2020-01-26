use std::fs::File;
use std::io::Read;
use std::slice::Iter;

use regex::Regex;

use super::token::*;

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
        lazy_static! {
            static ref RE_SYMBOL: Regex = Regex::new(r"\b[a-zA-Z_][a-zA-Z0-9_]*\b").unwrap();
            static ref RE_PUCTUATION: Regex = Regex::new(r"").unwrap();
        }

        for line in self._raw_text.lines() {
            println!("--- '{}'", line);

            let mut tokens: Vec<Token> = vec![];

            for find in RE_SYMBOL.find_iter(line) {
                let text = &line[find.start()..find.end()];

                match text.as_ref() {
                    "def" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Def), text: None }
                    ),
                    "class" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Class), text: None }
                    ),
                    "lambda" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Lambda), text: None }
                    ),
                    "if" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::If), text: None }
                    ),
                    "else" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Else), text: None }
                    ),
                    "elif" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Elif), text: None }
                    ),
                    "while" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::While), text: None }
                    ),
                    "for" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::For), text: None }
                    ),
                    "break" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Break), text: None }
                    ),
                    "continue" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Continue), text: None }
                    ),
                    "try" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Try), text: None }
                    ),
                    "except" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Except), text: None }
                    ),
                    "finally" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Finally), text: None }
                    ),
                    "with" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::With), text: None }
                    ),
                    "as" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::As), text: None }
                    ),
                    "import" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Import), text: None }
                    ),
                    "from" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::From), text: None }
                    ),
                    "not" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Not), text: None }
                    ),
                    "and" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::And), text: None }
                    ),
                    "or" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Or), text: None }
                    ),
                    "in" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::In), text: None }
                    ),
                    "is" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Is), text: None }
                    ),
                    "pass" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Pass), text: None }
                    ),
                    "return" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::Return), text: None }
                    ),
                    "None" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::None), text: None }
                    ),
                    "True" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::True), text: None }
                    ),
                    "False" => tokens.push(
                        Token { kind: TokenKind::Keyword(KeywordKind::False), text: None }
                    ),
                    _ => tokens.push(
                        Token { kind: TokenKind::Name, text: Some(String::from(text)) }
                    ),
                };
            }

            self._tokens.extend(tokens);
        }
    }

    pub fn tokens(&self) -> Iter<Token> {
        (&self._tokens).into_iter()
    }
}
