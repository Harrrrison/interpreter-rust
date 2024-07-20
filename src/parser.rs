use std::f32::consts::E;
use crate::scanner;
use crate::scanner::{TokenType, Literal, Token};

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

#[derive(Debug, Clone)]
enum Expr {
    Binary(Box<Expr>, TokenType, Box<Expr>),
    Unary(TokenType, Box<Expr>),
    Literal(Literal),
    Grouping(Box<Expr>),
}

impl Expr {
    fn new_binary(left: Expr, operator: TokenType, right: Expr) -> Self {
        Expr::Binary(Box::new(left), operator, Box::new(right))
    }

    fn new_unary(operator: TokenType, right: Expr) -> Self {
        Expr::Unary(operator, Box::new(right))
    }

    fn new_literal(literal: Literal) -> Self {
        Expr::Literal(literal)
    }

    fn new_grouping(expression: Expr) -> Self {
        Expr::Grouping(Box::new(expression))
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens_vector: Vec<Token>) -> Self {
        Self {
            tokens: tokens_vector,
            current: 0,
        }
    }

    pub fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().token_type.clone();
            let right = self.comparison();
            expr = Expr::new_binary(expr, operator, right); // where right is the operand
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        // Implement comparison function based on grammar
        // Similar to equality, but with comparison operators
        let mut expr = self.term();
        while self.match_tokens(&[TokenType::Greater, TokenType::GreaterEqual,
            TokenType::Less, TokenType::LessEqual]){
            let operator = self.previous().token_type.clone();
            let right = self.comparison();
            expr = Expr::new_binary(expr, operator, right);
        }

        expr
    }

    fn term(&mut self) -> Expr{
        let mut expr = self.factor();

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]){
            let operator = self.previous().token_type.clone();
            let right = self.factor();
            expr  = Expr::new_binary(expr, operator, right)
        }

        expr
    }

    fn factor(&mut self) -> Expr{
        let mut expr = self.factor();

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
let operator = self.previous().token_type.clone();
            let right = self.factor();
            expr = Expr::new_binary(expr, operator, right);
        }

        expr
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        // Implement match function to check for token types
        for token_type  in  types{
if (self.check(token_type)){
    self.advance();
    return true
}
        }
    false
    }

    fn check(&self, token_type: &TokenType) -> bool{
        if (self.isAtEnd()){ return false;}
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn advance(&self) -> &Token{
        if(!self.isAtEnd()){Self.current +=1}
        return self.previous();
    }

    fn isAtEnd(&self) -> bool{
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn peek(&self) -> &Token{
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]    }


}
