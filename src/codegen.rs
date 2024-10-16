// codegen.rs
use crate::ast::{Expr, Operator};

pub struct CodeGenerator;

impl CodeGenerator {
    pub fn generate(ir: Expr) -> i32 {
        match ir {
            Expr::Number(value) => value,
            Expr::BinaryOp(left, operator, right) => {
                let left_value = Self::generate(*left);
                let right_value = Self::generate(*right);

                match operator {
                    Operator::Plus => left_value + right_value,
                    Operator::Minus => left_value - right_value,
                    Operator::Multiply => left_value * right_value,
                    Operator::Divide => left_value / right_value,
                }
            }
        }
    }
}

