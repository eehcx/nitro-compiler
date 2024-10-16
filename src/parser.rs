use crate::ast::{Expr, Operator};
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn parse_number(&mut self) -> Expr {
        if let Token::Number(value) = *self.current_token() {  // Desreferenciamos el token
            self.advance();
            Expr::Number(value)  // Devolvemos el valor desreferenciado
        } else {
            panic!("Expected a number");
        }
    }

    fn parse_factor(&mut self) -> Expr {
        match self.current_token() {
            Token::Number(_) => self.parse_number(),
            Token::LParen => {
                self.advance(); // skip '('
                let expr = self.parse_expression();
                if let Token::RParen = self.current_token() {
                    self.advance(); // skip ')'
                } else {
                    panic!("Expected closing parenthesis");
                }
                expr
            }
            _ => panic!("Expected a factor"),
        }
    }

    fn parse_term(&mut self) -> Expr {
        let mut node = self.parse_factor();

        while matches!(self.current_token(), Token::Multiply | Token::Divide) {
            let operator = match self.current_token() {
                Token::Multiply => Operator::Multiply,
                Token::Divide => Operator::Divide,
                _ => unreachable!(),
            };
            self.advance();
            node = Expr::BinaryOp(Box::new(node), operator, Box::new(self.parse_factor()));
        }

        node
    }

    pub fn parse_expression(&mut self) -> Expr {
        let mut node = self.parse_term();

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let operator = match self.current_token() {
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                _ => unreachable!(),
            };
            self.advance();
            node = Expr::BinaryOp(Box::new(node), operator, Box::new(self.parse_term()));
        }

        node
    }

    pub fn parse(&mut self) -> Expr {
        self.parse_expression()
    }
}
