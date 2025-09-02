use std::collections::HashMap;
use crate::token::Token;

pub struct Lexer {
    source: String,
    pos: usize
}

impl Lexer {
    pub fn new(source: impl AsRef<str>) -> Self {
        Self {
            source: source.as_ref().to_string(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.peek_ahead(0)
    }

    fn peek_ahead(&self, by: usize) -> Option<char> {
        if self.pos + by >= self.source.len() {
            None
        } else {
            self.source.chars().nth(self.pos + by)
        }
    }

    fn consume(&mut self) -> Option<char> {
        let res = self.peek();
        self.pos += 1;
        res
    }

    fn consume_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(c) = self.peek() {
            if predicate(c) {
                self.consume();
            } else {
                break;
            }
        }
    }

    fn lex_identifier(&mut self) -> Token {
        let mut word = String::new();

        let keyword_map = HashMap::from([
            ("let", Token::Let),
            ("var", Token::Var),
            ("if", Token::If),
            ("else", Token::Else),
            ("while", Token::While),
            ("for", Token::For),
            ("do", Token::Do),
            ("continue", Token::Continue),
            ("break", Token::Break),
            ("return", Token::Return),
            ("function", Token::Function),
            ("true", Token::True),
            ("false", Token::False)
        ]);

        // Parse until whitespace or punctuation.
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' || c == '$' {
                word.push(self.consume().unwrap());
            } else {
                break;
            }
        }

        if let Some(token) = keyword_map.get(word.as_str()) {
            token.clone()
        } else {
            Token::Identifier(word)
        }
    }

    fn lex_numeric(&mut self) -> Token {
        let mut number = String::new();

        while self.peek().is_some() && self.peek().unwrap().is_digit(10) {
            number.push(self.consume().unwrap());
        }

        Token::Number(number.parse::<f64>().expect("Invalid number"))
    }

    fn lex_literal(&mut self) -> Token {
        self.consume(); // Consume opening quote.
        let mut literal = String::new();

        while self.peek() != Some('"') {
            literal.push(self.consume().unwrap());
        }
        self.consume(); // And closing quote.

        Token::StringLiteral(literal)
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(c) = self.peek() {

            let token: Option<Token> = match c {
                'a'..='z' | 'A'..='Z' | '_' => Some(self.lex_identifier()),
                '0'..='9' => Some(self.lex_numeric()),
                '"' => Some(self.lex_literal()),
                ' ' | '\n' | '\t' => { self.consume(); None },
                '(' => { self.consume(); Some(Token::LeftParen) },
                ')' => { self.consume(); Some(Token::RightParen) },
                '{' => { self.consume(); Some(Token::LeftBrace) },
                '}' => { self.consume(); Some(Token::RightBrace) },
                ';' => { self.consume(); Some(Token::Semicolon) },
                ':' => { self.consume(); Some(Token::Colon) },
                ',' => { self.consume(); Some(Token::Comma) },
                '.' => { self.consume(); Some(Token::Dot) },
                '[' => { self.consume(); Some(Token::LeftBracket) },
                ']' => { self.consume(); Some(Token::RightBracket) },
                '/' => {  // TODO: Implement multiline comments.
                    self.consume();
                    if self.peek() == Some('/') {
                        self.consume_while(|c| c != '\n');
                        None
                    } else if self.peek() == Some('=') {
                        Some(Token::SlashEqual)
                    } else {
                        Some(Token::Slash)
                    }
                },
                '*' => {
                    self.consume();
                    if self.peek() == Some('=') {
                        self.consume();
                        Some(Token::StarEqual)
                    } else {
                        Some(Token::Star)
                    }
                },
                '+' => {
                    self.consume();
                    if self.peek() == Some('=') {
                        self.consume();
                        Some(Token::PlusEqual)
                    } else {
                        Some(Token::Plus)
                    }
                },
                '-' => {
                    self.consume();
                    if self.peek() == Some('=') {
                        self.consume();
                        Some(Token::MinusEqual)
                    } else {
                        Some(Token::Minus)
                    }
                },
                '=' => {
                    self.consume();
                    if self.peek() == Some('=') {
                        self.consume();
                        Some(Token::EqualEqual)
                    } else {
                        Some(Token::Equal)
                    }
                },
                '!' => {
                    self.consume();
                    if self.peek() == Some('=') {
                        self.consume();
                        Some(Token::BangEqual)
                    } else {
                        Some(Token::Bang)
                    }
                },
                '<' => {
                    self.consume();
                    if self.peek() == Some('=') {
                        self.consume();
                        Some(Token::LessEqual)
                    } else {
                        Some(Token::Less)
                    }
                },
                '>' => {
                    self.consume();
                    if self.peek() == Some('=') {
                        self.consume();
                        Some(Token::GreaterEqual)
                    } else {
                        Some(Token::Greater)
                    }
                },
                _ => { self.consume(); None }
            };

            token.map(|t| tokens.push(t));
        }
        
        tokens.push(Token::EOF);
        tokens
    }
}