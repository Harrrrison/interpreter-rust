use std::f32::consts::E;
use std::ptr::{null, read};
use crate::scanner;
use crate::scanner::{TokenType, Literal, Token};
use crate::scanner::Literal::Number;
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

    fn unary(&mut self) -> Expr{
        if self.match_tokens(&[TokenType::Minus, TokenType::Bang]){
            let operator = self.previous().token_type.clone();
            let right = self.unary();
            return Expr::new_unary(operator, right);
        }

        self.primary()
    }

    fn primary(&mut self) -> Expr{
        if self.match_tokens(&[TokenType::False]){return Expr::new_literal(Literal::Bool(false))}
        if self.match_tokens(&[TokenType::True]){return Expr::new_literal(Literal::Bool(true))}
        if self.match_tokens(&[TokenType::Nil]){return Expr::new_literal(Literal::Nil)}

        if self.match_tokens(&[TokenType::Number, TokenType::String]){
            let token =  self.previous().clone();
            return Expr::new_literal(token.literal.clone().unwrap());
        }

        if self.match_tokens(&[TokenType::LeftParen]){
            let mut expr = self.expression();
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Expr::Grouping(Box::from(expr))
        }

        panic!(self.peek(), "Unexpected token.");
    }

    fn consume(&self, token_type: TokenType, message: &str) -> &Token{
        if(self.check(&token_type)){return self.advance();}

        panic!(self.peek(), message)
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
