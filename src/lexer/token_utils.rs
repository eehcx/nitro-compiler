use crate::lexer::tokens::Token;

pub fn integer_value(token: &Token, slice: &str) -> Option<i64> {
    if let Token::Integer = token {
        slice.parse::<i64>().ok()
    } else {
        None
    }
}

pub fn float_value(token: &Token, slice: &str) -> Option<f64> {
    if let Token::Float = token {
        slice.parse::<f64>().ok()
    } else {
        None
    }
}
