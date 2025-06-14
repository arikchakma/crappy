use crate::{ast::Statement, evaluate::evaluate, parser::Parser};

mod ast;
mod evaluate;
mod lexer;
mod parser;
mod token;

fn main() {
    let math_expressions = vec![
        "1 + 2 * 3",
        "(1 + 2) * 3",
        "1 + -2 * 3",
        "10 - 5 - 2",
        "10 - -2",
        "2 ^ 3 ^ 2",
    ];

    for expr_str in math_expressions {
        println!("Evaluating: {}", expr_str);
        println!("{}", "-".repeat(20));

        let source = expr_str;
        let mut parser = Parser::new(source);
        let program = parser.parse_program();

        if !parser.errors.is_empty() {
            println!("Parser Errors:");
            for err in parser.errors.iter() {
                println!("  - {}", err);
            }
        } else {
            println!("Program AST: {:#?}", program);
            println!("Program Expression: {}", program.to_string());

            for statement in program.statements {
                match statement {
                    Statement::ExpressionStatement(expr) => match evaluate(&expr) {
                        Ok(result) => println!("Evaluation Result: {}", result),
                        Err(e) => println!("Evaluation Error: {}", e),
                    },
                    // _ => println!("Unhandled statement type: {:?}", statement),
                }
            }
        }
        println!("---\n");
    }
}
