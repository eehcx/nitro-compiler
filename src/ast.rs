#[derive(Debug, Clone)] // Implementamos Clone para poder clonar Expr
pub enum Expr {
    Number(i32),
    BinaryOp(Box<Expr>, Operator, Box<Expr>),
}

#[derive(Debug, Clone)] // También implementamos Clone para Operator
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}
