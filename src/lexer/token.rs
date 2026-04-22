use crate::parser::BinaryOperator;

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

    pub fn as_binary_operator(&self) -> Option<BinaryOperator> {
        match self {
            Token::Plus => Some(BinaryOperator::Add),
            Token::Minus => Some(BinaryOperator::Sub),
            Token::Star => Some(BinaryOperator::Mul),
            Token::Slash => Some(BinaryOperator::Div),
            Token::Percent => Some(BinaryOperator::Mod),
            Token::Amp => Some(BinaryOperator::BinaryAnd),
            Token::AmpAmp => Some(BinaryOperator::LogicalAnd),
            Token::PipePipe => Some(BinaryOperator::LogicalOr),
            Token::Pipe => Some(BinaryOperator::BinaryOr),
            Token::EqualEqual => Some(BinaryOperator::Equal),
            Token::BangEqual => Some(BinaryOperator::NotEqual),
            Token::Greater => Some(BinaryOperator::GreaterThan),
            Token::GreaterEqual => Some(BinaryOperator::GreaterThanOrEqual),
            Token::Less => Some(BinaryOperator::LessThan),
            Token::LessEqual => Some(BinaryOperator::LessThanOrEqual),
            _ => None,
        }
    }
}