mod lexer;

//use lexer::tokenize;
//use lexer::tokens::Token;
//use crate::lexer::parser::parse_number;
//use logos::Logos;
use crate::lexer::tokenize;
//use crate::parser::parse_expression;

/*
fn main() {
    let text = "42 3.14";
    let mut lexer = Token::lexer(text);

    while let Some(expr) = parse_number(&mut lexer) {
        println!("{:?}", expr);
    }
    match parse_expression(&tokens) {
        Ok(expr) => println!("{:?}", expr),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}*/

fn main() {
    //let source = "let x = 5;";
    let source = "let x = 10 + 2;";
    // let source = "3 + 5 * (10 - 4)";
    let tokens = tokenize(source);
    //let mut tokens = source.into_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    //let ast = parse_expr(&mut tokens).unwrap();
    //println!("{:?}", ast);
}
