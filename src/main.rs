use crate::{
    ast::Statement,
    evaluate::evaluate,
    parser::{Parser, check_parser_error},
};
use std::{io, io::Write};

mod ast;
mod evaluate;
mod lexer;
mod parser;
mod token;

fn main() {
    println!("{}", "-".repeat(20));
    println!("Crappy Calculator!");
    println!("{}", "-".repeat(20));
    println!("Enter an expression to evaluate:");
    println!("Type '/bye' to quit.");

    start_repl();
}

fn start_repl() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "/bye" {
            println!("Bye!");
            break;
        }

        let mut parser = Parser::new(&input);
        let program = parser.parse_program();
        check_parser_error(&parser);

        for statement in program.statements {
            match statement {
                Statement::ExpressionStatement(expr) => match evaluate(&expr) {
                    Ok(result) => println!("{}", result),
                    Err(e) => println!("{}", e),
                },
            }
        }
    }
}
