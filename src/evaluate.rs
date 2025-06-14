use crate::ast::{BinaryOperator, Expression, UnaryOperator};

pub fn evaluate(expr: &Expression) -> Result<i64, String> {
    match expr {
        Expression::Integer(val) => Ok(*val),
        Expression::UnaryOp { op, expr } => {
            let val = evaluate(expr)?;
            match op {
                UnaryOperator::Plus => Ok(val),
                UnaryOperator::Minus => Ok(-val),
            }
        }
        Expression::BinaryOp { left, op, right } => {
            let left_val = evaluate(left)?;
            let right_val = evaluate(right)?;

            match op {
                BinaryOperator::Plus => Ok(left_val + right_val),
                BinaryOperator::Minus => Ok(left_val - right_val),
                BinaryOperator::Multiply => Ok(left_val * right_val),
                BinaryOperator::Divide => {
                    if right_val == 0 {
                        Err("Division by zero!".to_string())
                    } else {
                        Ok(left_val / right_val)
                    }
                }
                BinaryOperator::Power => {
                    if right_val < 0 {
                        Err("Negative exponents not supported for integers".to_string())
                    } else {
                        Ok(left_val.pow(right_val as u32))
                    }
                }
            }
        }
    }
}
