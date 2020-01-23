#[derive(Debug)]
pub struct Token {
    pub text: String,
    pub kind: TokenKind,
}

#[derive(Debug)]
pub enum TokenKind {
    NumLiteral,
    StrLiteral,

    Name,
    Keyword,

    Operator,
}

pub enum NumLiteralKind {
    Dec,
    Bin,
    Oct,
    Hex,
}

pub enum StrLiteralKind {
    Normal,
    Raw,
    Formatted,
    Multiline,
}

pub enum KeywordKind {
    Num,
    Str,
    Dyn,

    Def,
    Class,
    Lambda,

    Import,

    If,
    Else,
    Elif,

    For,
    While,
    Break,
    Continue,

    With,
    As,
    In,
    Is,
    From,

    NOT,
    OR,
    AND,

    Return,

    None,
    True,
    False,
}

pub enum OperatorKind {
    Dot,

    Assign,
    Add,
    Sub,
    Mul,
    Div,

    BitAND,
    BitOR,
    BitXOR,
    BitNOT,

    OpParen,
    EdParen,
    OpBrack,
    EdBrack,
    OpCurly,
    EdCurly,

    EQ,
    GT,
    LT,
    GE,
    LE,
    NE,

    Arrow,
    Colon,
    Comma,

    Hash,

    Semicolon,
}
