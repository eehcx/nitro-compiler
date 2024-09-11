pub mod tokens;
//pub mod token_utils;

use logos::Logos;
use tokens::Token;

pub fn tokenize(source: &str) -> Vec<Token> {
    Token::lexer(source).filter_map(Result::ok).collect()
}
