

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    StringLiteral(String),
    Number(f64),

    Null,
    Undefined,

    // Keywords
    Let, Var,
    If, Else,
    While, For, Do,
    Continue, Break,
    Return,
    Function,
    Class,
    True,
    False,
    New,

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
    Percent,
    PercentEqual,
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
    Amp,
    AmpAmp,
    Pipe,
    PipePipe,
    PlusPlus,
    MinusMinus,

    EOF,
}

impl Token {
    pub fn is_assignment_operator(&self) -> bool {
        match self {
            Token::PlusEqual | Token::MinusEqual | Token::StarEqual | Token::SlashEqual => true,
            _ => false,
        }
    }
}