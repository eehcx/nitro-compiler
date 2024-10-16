// semantic.rs
use crate::ast::Expr;

pub struct SemanticAnalyzer;

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer
    }

    pub fn analyze(&mut self, _ast: &Expr) {
        // En este caso simple, no realizamos ningún análisis semántico adicional
    }
}

