use crate::{
    ast::{BinaryOperator, Expression, Program, Statement, UnaryOperator},
    lexer::Lexer,
    token::Token,
};

type Precedence = u8;

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub current_token: Token,
    pub peek_token: Token,
    pub errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer::new(source);
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Self {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token()
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statements: Vec::new(),
        };

        while self.current_token != Token::EOF {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            } else {
                self.errors.push(format!(
                    "Could not parse statement starting with {:?}",
                    self.current_token
                ));
                self.next_token();
            }
        }
        program
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        self.parse_expression_statement()
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = match self.parse_expression(0) {
            Ok(e) => e,
            Err(msg) => {
                self.errors.push(msg);
                return None;
            }
        };

        Some(Statement::ExpressionStatement(expr))
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, String> {
        let token_for_nud = self.current_token.clone();
        self.next_token();

        let mut left = self.nud(token_for_nud)?;

        while self.current_token != Token::EOF
            && precedence < self.get_binding_power(&self.current_token).0
        {
            let op_token = self.current_token.clone();
            self.next_token();

            left = self.led(left, op_token)?;
        }

        Ok(left)
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), String> {
        if self.current_token == expected {
            self.next_token();
            Ok(())
        } else {
            Err(format!(
                "Expected {:?}, but got {:?}",
                expected, self.current_token
            ))
        }
    }

    fn nud(&mut self, token: Token) -> Result<Expression, String> {
        match token {
            Token::Integer(value) => Ok(Expression::Integer(value)),
            Token::Plus => {
                let r_bp = self.get_prefix_binding_power(&Token::Plus).unwrap();
                let expr = self.parse_expression(r_bp)?;
                Ok(Expression::UnaryOp {
                    op: UnaryOperator::Plus,
                    expr: Box::new(expr),
                })
            }
            Token::Minus => {
                let r_bp = self.get_prefix_binding_power(&Token::Minus).unwrap();
                let expr = self.parse_expression(r_bp)?;
                Ok(Expression::UnaryOp {
                    op: UnaryOperator::Minus,
                    expr: Box::new(expr),
                })
            }
            Token::LParen => {
                let expr = self.parse_expression(0)?;
                self.expect_token(Token::RParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token in NUD: {:?}", token)),
        }
    }

    fn led(&mut self, left: Expression, token: Token) -> Result<Expression, String> {
        let (_, r_bp) = self.get_binding_power(&token);
        let op = match token {
            Token::Plus => BinaryOperator::Plus,
            Token::Minus => BinaryOperator::Minus,
            Token::Asterisk => BinaryOperator::Multiply,
            Token::Slash => BinaryOperator::Divide,
            Token::Caret => BinaryOperator::Power,
            _ => {
                return Err(format!(
                    "Unexpected token in LED, not an infix operator: {:?}",
                    token
                ));
            }
        };

        let right = self.parse_expression(r_bp)?;

        Ok(Expression::BinaryOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        })
    }

    fn get_binding_power(&self, token: &Token) -> (Precedence, Precedence) {
        match token {
            Token::Plus | Token::Minus => (50, 50),
            Token::Asterisk | Token::Slash => (60, 60),
            Token::Caret => (71, 70),
            _ => (0, 0),
        }
    }

    fn get_prefix_binding_power(&self, token: &Token) -> Option<Precedence> {
        match token {
            Token::Plus | Token::Minus => Some(80),
            _ => None,
        }
    }
}

pub fn check_parser_error(p: &Parser) {
    if !p.errors.is_empty() {
        for err in p.errors.iter() {
            println!("  - {}", err);
        }
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let math_expressions = vec![
        ("(1 + 2) * 3", "((1 + 2) * 3)"),
        ("1 + 2 * 3", "(1 + (2 * 3))"),
        ("1 + -2 * 3", "(1 + ((-2) * 3))"),
        ("10 - 5 - 2", "((10 - 5) - 2)"),
        ("10 - -2", "(10 - (-2))"),
        ("2 ^ 3 ^ 2", "(2 ^ (3 ^ 2))"),
    ];

    for expr_str in math_expressions {
        let (source, expected) = expr_str;

        let mut parser = Parser::new(source);
        let program = parser.parse_program();
        check_parser_error(&parser);

        assert_eq!(parser.errors.len(), 0);
        assert_eq!(format!("{}", program), expected);
    }
}
