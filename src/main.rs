mod lexer;
mod parser;
mod ast;
mod semantic;
mod ir;
mod codegen;

use lexer::Lexer;
use parser::Parser;
use semantic::SemanticAnalyzer;
use ir::IRGenerator;
use codegen::CodeGenerator;

fn main() {
    //let input = "3 + 5 * (10 - 4)";
    let input ="30 / 10";

    // Etapa 1: Lexer (Análisis Léxico)
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    println!("Tokens: {:?}", tokens);

    // Etapa 2: Parser (Análisis Sintáctico)
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("AST: {:?}", ast);

    // Etapa 3: Análisis Semántico
    let mut semantic_analyzer = SemanticAnalyzer::new();
    semantic_analyzer.analyze(&ast);
    
    // Etapa 4: Generación de Código Intermedio (IR)
    let ir = IRGenerator::generate(&ast);
    println!("IR: {:?}", ir);

    // Etapa 5: Generación de Código
    let result = CodeGenerator::generate(ir);
    println!("Resultado: {}", result);
}
