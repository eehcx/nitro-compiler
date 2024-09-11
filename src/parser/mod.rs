pub mod grammar;
pub mod errors;

use crate::lexer::tokens::Token;
use grammar::Expr;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    pub fn parse_expr(&mut self) -> Expr {
        self.parse_term()
    }

    fn parse_term(&mut self) -> Expr {
        let mut node = self.parse_factor();

        while matches!(self.current_token(), Token::Plus | Token::Minus) {
            let op = self.current_token().clone();
            self.advance();
            let right = self.parse_factor();
            node = Expr::Binary(Box::new(node), op, Box::new(right));
        }

        node
    }

    fn parse_factor(&mut self) -> Expr {
        let mut node = self.parse_primary();

        while matches!(self.current_token(), Token::Star | Token::Slash) {
            let op = self.current_token().clone();
            self.advance();
            let right = self.parse_primary();
            node = Expr::Binary(Box::new(node), op, Box::new(right));
        }

        node
    }

    fn parse_primary(&mut self) -> Expr {
        match self.current_token() {
            Token::Number(n) => {
                self.advance();
                Expr::Number(n.parse().unwrap())
            }
            Token::Float(f) => {
                self.advance();
                Expr::Float(f.parse().unwrap())
            }
            Token::Identifier(id) => {
                self.advance();
                Expr::Identifier(id.clone())
            }
            Token::ParenL => {
                self.advance();
                let expr = self.parse_expr();
                self.advance(); // Consume ')'
                expr
            }
            _ => panic!("Unexpected token: {:?}", self.current_token()),
        }
    }
}
