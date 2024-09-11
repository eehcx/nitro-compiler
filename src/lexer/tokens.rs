use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("let")]
    Let,

    #[token("fn")]
    Fn,

    #[token("(")]
    ParenL,

    #[token(")")]
    ParenR,

    #[token("{")]
    BraceL,

    #[token("}")]
    BraceR,

    #[token(";")]
    Semicolon,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[regex("-?[0-9]+")]
    Number,

    #[regex("-?[0-9]+\\.[0-9]+")]
    Float,

    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]*")]
    Identifier,

    #[token("=")]
    Equals,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    // Ignora los espacios en blanco
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Ignored,
}

// Pruebas
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_token() {
        let mut lexer = Token::lexer("42");
        let token = lexer.next().unwrap();

        match token {
            Ok(Token::Integer) => assert_eq!(lexer.slice(), "42"),
            _ => panic!("Expected Integer token"),
        }
    }

    #[test]
    fn test_float_token() {
        let mut lexer = Token::lexer("3.14");
        let token = lexer.next().unwrap();

        match token {
            Ok(Token::Float) => assert_eq!(lexer.slice(), "3.14"),
            _ => panic!("Expected Float token"),
        }
    }

    #[test]
    fn test_let_token() {
        let mut lexer = Token::lexer("let");
        let token = lexer.next().unwrap();

        match token {
            Ok(Token::Let) => {}
            _ => panic!("Expected Let token"),
        }
    }

    #[test]
    fn test_identifier_token() {
        let mut lexer = Token::lexer("myVar");
        let token = lexer.next().unwrap();

        match token {
            Ok(Token::Identifier) => assert_eq!(lexer.slice(), "myVar"),
            _ => panic!("Expected Identifier token"),
        }
    }
}
