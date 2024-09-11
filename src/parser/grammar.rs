use crate::lexer::tokens::Token;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Float(f64),
    Identifier(String),
    Binary(Box<Expr>, Token, Box<Expr>),
}

/*
use yap::{Tokens, IntoTokens};

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i64),
    Binary(Box<Expr>, Token, Box<Expr>),
}

pub fn parse_expr(tokens: &mut impl Tokens<Item = Token>) -> Option<Expr> {
    parse_term(tokens)
}

fn parse_term(tokens: &mut impl Tokens<Item = Token>) -> Option<Expr> {
    let mut node = parse_factor(tokens)?;

    while let Some(token) = tokens.peek() {
        match token {
            Token::Plus | Token::Minus => {
                let op = tokens.next().unwrap();
                let right = parse_factor(tokens)?;
                node = Expr::Binary(Box::new(node), op, Box::new(right));
            }
            _ => break,
        }
    }

    Some(node)
}

fn parse_factor(tokens: &mut impl Tokens<Item = Token>) -> Option<Expr> {
    let mut node = parse_primary(tokens)?;

    while let Some(token) = tokens.peek() {
        match token {
            Token::Star | Token::Slash => {
                let op = tokens.next().unwrap();
                let right = parse_primary(tokens)?;
                node = Expr::Binary(Box::new(node), op, Box::new(right));
            }
            _ => break,
        }
    }

    Some(node)
}

fn parse_primary(tokens: &mut impl Tokens<Item = Token>) -> Option<Expr> {
    match tokens.next()? {
        Token::Number(n) => Some(Expr::Number(n)),
        Token::LParen => {
            let expr = parse_expr(tokens)?;
            tokens.next(); // Consume ')'
            Some(expr)
        }
        _ => None,
    }
}

use nom::{
    IResult,
    branch::alt,
    combinator::map,
    sequence::tuple,
    error::{Error, ErrorKind},
};

use crate::lexer::tokens::Token;
//use crate::parser::errors::ParseError;

use crate::lexer::token_utils::{integer_value, float_value};
use logos::Lexer;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i64),
    Float(f64),
    //Add(Box<Expr>, Box<Expr>),
}

fn parse_number(lexer: &mut Lexer<Token>) -> Option<Expr> {
    if let Some(token) = lexer.next() {
        let slice = lexer.slice();
        match token {
            Token::Integer => {
                if let Some(value) = integer_value(&token, slice) {
                    return Some(Expr::Number(value));
                }
            }
            Token::Float => {
                if let Some(value) = float_value(&token, slice) {
                    return Some(Expr::Float(value));
                }
            }
            _ => {}
        }
    }
    None
}


fn parse_number(input: &[Token]) -> IResult<&[Token], Expr> {
    match input.split_first() {
        Some((Token::Integer(n), rest)) => Ok((rest, Expr::Number(*n))),
        _ => Err(nom::Err::Error(Error::new(input, ErrorKind::Tag))),
    }
}

fn parse_plus(input: &[Token]) -> IResult<&[Token], ()> {
    if let Some((Token::Plus, rest)) = input.split_first() {
        Ok((rest, ()))
    } else {
        Err(nom::Err::Error(Error::new(input, ErrorKind::Tag)))
    }
}

fn parse_add(input: &[Token]) -> IResult<&[Token], Expr> {
    map(
        tuple((parse_number, parse_plus, parse_number)),
        |(left, _, right)| Expr::Add(Box::new(left), Box::new(right))
    )(input)
}

fn parse_expr(input: &[Token]) -> IResult<&[Token], Expr> {
    alt((parse_add, parse_number))(input)
}

pub fn parse_expression(input: &[Token]) -> Result<Expr, ParseError> {
    match parse_expr(input) {
        Ok((_, expr)) => Ok(expr),
        Err(_) => Err(ParseError::InvalidInput),
    }
}*/
