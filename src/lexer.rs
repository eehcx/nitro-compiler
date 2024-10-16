// lexer.rs
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    Number(i32),
    LParen,
    RParen,
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn number(&mut self) -> i32 {
        let mut result = String::new();
        while let Some(c) = self.current_char() {
            if c.is_digit(10) {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        result.parse::<i32>().unwrap()
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current_char() {
            match c {
                ' ' | '\t' | '\n' => {
                    self.skip_whitespace();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Divide);
                    self.advance();
                }
                '(' => {
                    tokens.push(Token::LParen);
                    self.advance();
                }
                ')' => {
                    tokens.push(Token::RParen);
                    self.advance();
                }
                '0'..='9' => {
                    let number = self.number();
                    tokens.push(Token::Number(number));
                }
                _ => {
                    panic!("Unknown character: {}", c);
                }
            }
        }

        tokens.push(Token::EOF);
        tokens
    }
}

