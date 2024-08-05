use std::cmp::PartialEq;
use std::panic::{self, UnwindSafe};
use crate::scanner::{Literal, Token, TokenType};

/*
Grammar:
expression     → equality ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
               | primary ;
primary        → NUMBER | STRING | "true" | "false" | "nil"
               | "(" expression ")" ;
 */

#[derive(Debug)]
pub struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error")
    }
}

impl std::error::Error for ParseError {
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {

        left: Box<Expr>,

        operator: Token,

        right: Box<Expr>,

    },

    Grouping {

        expression: Box<Expr>,

    },

    Literal {

        value: Literal,

    },

    Unary {

        operator: Token,

        right: Box<Expr>,

    },
}

#[derive(Clone, Debug)]
pub enum Object {

    Number(f32),

    String(String),

    Boolean(bool),

    Nil,

}

impl Expr {
    fn new_binary(left: Expr, operator: Token, right: Expr) -> Self {
        Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    fn new_unary(operator: Token, right: Expr) -> Self {
        Expr::Unary{
            operator,
            right: Box::new(right),
        }

    }

    fn new_literal(literal: Literal) -> Self {
        Expr::Literal{
            value: literal,
        }
    }

    fn new_grouping(expression: Expr) -> Self {
        Expr::Grouping{
            expression: Box::new(expression),
        }
    }
}

impl<'a> std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                write!(f, "({} {} {})", operator, left, right)
            }

            Expr::Grouping { expression } => {
                write!(f, "(group {})", expression)
            }

            Expr::Literal { value } => {
                write!(f, "{}",
                match value {
                    Literal::Number(literal)  => format!("{:?}", literal),
                    literal=> format!("{}", literal),
                })
            }

            Expr::Unary { operator, right } => {
                write!(f, "({:?} {})", operator.lexeme, right)
            }

        }
    }
}

pub struct Parser {
    pub(crate) tokens: Vec<Token>,
    current: usize,
}

impl Parser { // TODO: Return exit code on null error :) test 3 expected exit code 65, got 0
    pub fn new(tokens_vector: Vec<Token>) -> Self {
        Self {
            tokens: tokens_vector,
            current: 0,
        }
    }

    pub fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.match_tokens(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::new_binary(expr, operator, right)
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::new_binary(expr, operator, right);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_tokens(&[TokenType::Minus, TokenType::Bang]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::new_unary(operator, right));
        }

        self.primary()
    }

    pub(crate) fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_tokens(&[TokenType::True]) {
            return Ok(Expr::new_literal(Literal::Bool(true)));
        }

        if self.match_tokens(&[TokenType::False]) {
            return Ok(Expr::new_literal(Literal::Bool(false)));
        }

        if self.match_tokens(&[TokenType::Nil]) {
            return Ok(Expr::new_literal(Literal::Nil));
        }

        if self.match_tokens(&[TokenType::Number, TokenType::String]) {
            let token = self.previous().clone();
            return Ok(Expr::new_literal(token.literal.clone().unwrap()));
        }

        if self.match_tokens(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::new_grouping(expr));
        }

        Err(ParseError)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }
        Err(ParseError)
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        ( self.tokens.len() == self.current|| self.peek().token_type == TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    pub(crate) fn parse(&mut self) -> Result<Expr, ParseError>{
        match self.expression() {
            Ok(result) => Ok(result),
            Err(ParseError) => Err(ParseError),
        }
    }

    }