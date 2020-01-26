#[derive(Debug)]
pub struct Token {
    pub text: Option<String>,
    pub kind: TokenKind,
}

#[derive(Debug)]
pub enum TokenKind {
    NumLiteral(NumLiteralKind),
    StrLiteral(StrLiteralKind),

    Name,
    Keyword(KeywordKind),

    Operator(OperatorKind),
    Delimiter,
}

#[derive(Debug)]
pub enum NumLiteralKind {
    Dec,
    Bin,
    Oct,
    Hex,
}

#[derive(Debug)]
pub enum StrLiteralKind {
    Normal,
    Raw,
    Formatted,
    Multiline,
}

#[derive(Debug)]
pub enum KeywordKind {
    Def,
    Class,
    Lambda,
    Kind,

    If,
    Else,
    Elif,

    While,
    For,
    Break,
    Continue,

    Try,
    Except,
    Finally,

    With,
    As,

    Import,
    From,

    Not,
    Or,
    And,
    In,
    Is,

    Pass,
    Return,

    None,
    True,
    False,
}

#[derive(Debug)]
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

#[derive(Debug)]
enum DelimiterKind {
    OpParen,
    EdParen,
    OpBrack,
    EdBrack,
    OpCurly,
    EdCurly,
}