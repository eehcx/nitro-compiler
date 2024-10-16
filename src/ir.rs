use crate::ast::Expr;

pub struct IRGenerator;

impl IRGenerator {
    pub fn generate(ast: &Expr) -> Expr {
        ast.clone() // Ahora Expr implementa Clone, por lo que podemos clonarlo
    }
}
