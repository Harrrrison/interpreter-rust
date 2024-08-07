use std::f32::consts::E; // Kept for illustration; this might not be needed depending on context.
use crate::parser::{Expr, Object, ParseError}; // Assumed modules
use crate::scanner::{Literal, Token, TokenType};
use crate::scanner::TokenType::Plus; // Correctly used below

pub struct RunTimeError {
    pub token: Token,
    pub message: String,
}

impl RunTimeError {
    pub fn new(token: &Token, message: &str) -> RunTimeError {
        RunTimeError {
            token: token.clone(),
            message: message.to_string(),
        }
    }

}

pub struct Interpreter {} // Fixed typo in struct name

impl Interpreter {
    pub(crate) fn new() -> Interpreter {
        Interpreter {}
    }

    fn visit_literal_expr(&self, expr: Expr) -> Result<Literal, RunTimeError> {
        // Changed argument type from `Expr::Literal` to `Expr`
        if let Expr::Literal { value } = expr {
            Ok(value)
        } else {
            Err(RunTimeError::from(RunTimeError { token: Token {
                token_type: TokenType::LeftParen,
                lexeme: "".to_string(),
                literal: None,
                line: 0,
            }, message: "literal".to_string() }))
        }
    }

    fn visit_grouping_expr(&self, expr: Expr) -> Result<Literal, RunTimeError> {
        if let Expr::Grouping { expression } = expr.clone() {
            self.evaluate(*expression)
        } else {
            Err(RunTimeError::from(RunTimeError { token: Token {
                token_type: TokenType::LeftParen,
                lexeme: "".to_string(),
                literal: None,
                line: 0,
            }, message: "group".to_string() }))
        }
    }

    fn evaluate(&self, expr: Expr) -> Result<Literal, RunTimeError> {
        // Fixed handling of different expression types
        match expr {
            Expr::Binary {
                left,
                operator,
                right,
            } => self.visit_binary_expression(*left, operator, *right),
            Expr::Grouping { expression } => self.visit_grouping_expr(*expression),
            Expr::Literal { value } => Ok(value),
            Expr::Unary { operator, right } => self.visit_unary_expr(operator, *right),
        }
    }

    fn visit_unary_expr(&self, operator: Token, expr: Expr) -> Result<Literal, RunTimeError> {
        let right = self.evaluate(expr)?;

        match operator.token_type {
            TokenType::Minus => {
                let value = self.check_number_operand(&operator, Some(right.clone()))?;
                Ok(Literal::Number(-value)) // Correctly return a `Literal`
            }
            TokenType::Bang => Ok(Literal::Bool(!self.is_truthy(&right))),
            _ => Err(RunTimeError::new(&operator, "Invalid unary operator")),
        }
    }

    fn is_truthy(&self, literal: &Literal) -> bool {
        match literal {
            Literal::Bool(b) => *b, // Fixed handling for literal boolean
            Literal::Nil => false,  // Handle Nil explicitly
            _ => true,
        }
    }

    fn visit_binary_expression(
        &self,
        left_expr: Expr,
        operator: Token,
        right_expr: Expr,
    ) -> Result<Literal, RunTimeError> {
        // Reorganized arguments to handle Expr structs

        let left = self.evaluate(left_expr)?;
        let right = self.evaluate(right_expr)?;

        match operator.token_type {
            TokenType::Greater => {
                let (left_val, right_val) = self.check_number_operands(&operator, &left, &right)?;
                Ok(Literal::Bool(left_val > right_val))
            }
            TokenType::GreaterEqual => {
                let (left_val, right_val) = self.check_number_operands(&operator, &left, &right)?;
                Ok(Literal::Bool(left_val >= right_val))
            }
            TokenType::Less => {
                let (left_val, right_val) = self.check_number_operands(&operator, &left, &right)?;
                Ok(Literal::Bool(left_val < right_val))
            }
            TokenType::LessEqual => {
                let (left_val, right_val) = self.check_number_operands(&operator, &left, &right)?;
                Ok(Literal::Bool(left_val <= right_val))
            }
            TokenType::Minus => {
                let (left_val, right_val) = self.check_number_operands(&operator, &left, &right)?;
                Ok(Literal::Number(left_val - right_val))
            }
            TokenType::Slash => {
                let (left_val, right_val) = self.check_number_operands(&operator, &left, &right)?;
                if right_val == 0.0 {
                    return Err(RunTimeError::new(&operator,"Division by zero")); // Handle divide by zero
                }
                Ok(Literal::Number(left_val / right_val))
            }
            TokenType::Star => {
                let (left_val, right_val) = self.check_number_operands(&operator, &left, &right)?;
                Ok(Literal::Number(left_val * right_val))
            }
            TokenType::Plus => match (&left, &right) {
                (Literal::Number(l), Literal::Number(r)) => Ok(Literal::Number(l + r)),
                (Literal::String(l), Literal::String(r)) => {
                    Ok(Literal::String(format!("{}{}", l, r))) // Concatenate strings correctly
                }
                _ => Err(RunTimeError::new(
                    &operator,
                    "Operands must be two numbers or strings",
                )
                    ),
            },
            TokenType::BangEqual => Ok(Literal::Bool(!self.is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(Literal::Bool(self.is_equal(&left, &right))),
            _ => Err(RunTimeError::new(&operator, "Unknown binary operator")),
        }
    }

    fn is_equal(&self, left: &Literal, right: &Literal) -> bool {
        // Handle equality checking correctly
        left == right
    }

    fn check_number_operand(
        &self,
        operator: &Token,
        operand: Option<Literal>,
    ) -> Result<f64, RunTimeError> {
        match operand {
            Some(Literal::Number(x)) => Ok(x),
            _ => Err(RunTimeError::new(
                operator,
                "Operand must be a number",
            )),
        }
    }

    fn check_number_operands(
        &self,
        operator: &Token,
        left: &Literal,
        right: &Literal,
    ) -> Result<(f64, f64), RunTimeError> {
        match (left, right) {
            (Literal::Number(x), Literal::Number(y)) => Ok((*x, *y)),
            _ => Err(RunTimeError::new(
                operator,
                "Operands must be numbers",
            )),
        }
    }

    pub(crate) fn interpret(&self, expression: Expr) {
        match self.evaluate(expression) {
            Ok(result) => println!("{}", self.stringify(&Some(result))),
            Err(err) => println!("Runtime error: {}", err.message), // Print error message correctly
        }
    }

    fn stringify(&self, literal: &Option<Literal>) -> String {
        match literal {
            None => "nil".to_string(), // Use to_string for String conversion
            Some(Literal::Number(n)) => {
                let text = n.to_string();
                if text.ends_with(".0") {
                    text.strip_suffix(".0").unwrap().to_string()
                } else {
                    text
                }
            }
            Some(Literal::String(s)) => s.clone(),
            Some(Literal::Bool(b)) => b.to_string(),
            Some(Literal::Nil) => "nil".to_string(),
        }
    }
}