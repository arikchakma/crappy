use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in &self.statements {
            writeln!(f, "{}", statement)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    ExpressionStatement(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::ExpressionStatement(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Integer(i64),
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Integer(value) => write!(f, "{value}"),
            Expression::UnaryOp { op, expr } => write!(f, "({}{})", op, expr),
            Expression::BinaryOp { left, op, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            UnaryOperator::Plus => "+",
            UnaryOperator::Minus => "-",
        };
        write!(f, "{}", symbol)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            BinaryOperator::Plus => "+",
            BinaryOperator::Minus => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::Power => "^",
        };
        write!(f, "{}", symbol)
    }
}
