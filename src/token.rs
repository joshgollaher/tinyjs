

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Literal(String),
    Number(f64),

    // Keywords
    Let, Var,
    If, Else,
    While, For, Do,
    Continue, Break,
    Return,
    Function,
    True,
    False,

    // Punctuation
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    LeftBracket, RightBracket,
    Comma,
    Dot,
    Colon,
    Semicolon,

    // Operators
    Slash,
    Plus,
    Minus,
    Star,
    SlashEqual,
    PlusEqual,
    MinusEqual,
    StarEqual,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}