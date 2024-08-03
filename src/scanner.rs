use std::collections::HashMap;
use std::fmt;
use std::fmt::{Formatter, write};
use std::process::exit;
use std::sync::Once;
use std::cmp::{PartialEq,Eq};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Star,
    Dot,
    Comma,
    Plus,
    Minus,
    EqualEqual,
    Equal,
    BangEqual,
    Bang,
    LessEqual,
    Less,
    GreaterEqual,
    Greater,
    SemiColon,
    Slash,
    String,
    Number,
    Identifier,
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}
// TODO: Need to change this from for TokenType to for Token so as to output the literal values
// for the literals

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.token_type,
            self.lexeme,
            match &self.literal {
                Some(literal) => format!("{}", literal),
                None => "None".to_string(),
            }
        )
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Number(value) => write!(f, "{}", value),
            Literal::String(value) => write!(f, "{}", value),
            Literal::Bool(value) => write!(f, "{}", value),
            Literal::Nil => write!(f, "nil"),
        }
    }
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::LeftParen => write!(f, "LEFT_PAREN"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE"),
            TokenType::Star => write!(f, "STAR"),
            TokenType::Dot => write!(f, "DOT"),
            TokenType::Comma => write!(f, "COMMA"),
            TokenType::Plus => write!(f, "PLUS"),
            TokenType::Minus => write!(f, "MINUS"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL"),
            TokenType::Equal => write!(f, "EQUAL"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL"),
            TokenType::Bang => write!(f, "BANG"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL"),
            TokenType::Less => write!(f, "LESS"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL"),
            TokenType::Greater => write!(f, "GREATER"),
            TokenType::SemiColon => write!(f, "SEMICOLON"),
            TokenType::Slash => write!(f, "SLASH"),
            TokenType::String => write!(f, "STRING"),
            TokenType::Number => write!(f, "NUMBER"),
            TokenType::Identifier => write!(f, "IDENTIFIER"),
            TokenType::And => write!(f, "AND"),
            TokenType::Class => write!(f, "CLASS"),
            TokenType::Else => write!(f, "ELSE"),
            TokenType::False => write!(f, "FALSE"),
            TokenType::For => write!(f, "FOR"),
            TokenType::Fun => write!(f, "FUN"),
            TokenType::If => write!(f, "IF"),
            TokenType::Nil => write!(f, "NIL"),
            TokenType::Or => write!(f, "OR"),
            TokenType::Print => write!(f, "PRINT"),
            TokenType::Return => write!(f, "RETURN"),
            TokenType::Super => write!(f, "SUPER"),
            TokenType::This => write!(f, "THIS"),
            TokenType::True => write!(f, "TRUE"),
            TokenType::Var => write!(f, "VAR"),
            TokenType::While => write!(f, "WHILE"),
            TokenType::Eof => write!(f, "EOF"),
        }
    }
}

// Used Once and Option for thread-safe initialization of keywords
static INIT: Once = Once::new();
static mut KEYWORDS: Option<HashMap<&'static str, TokenType>> = None;

pub struct Scanner {
    pub tokens: Vec<Token>,
    line: usize,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            line: 1,
        }
    }

    pub fn scan_and_tokenize(&mut self, input: &str) -> i32 {
        let mut return_code = 0;
        let mut line_nb = 1;

        let keywords = get_keywords();
        let mut chars = input.chars().peekable();
        while let Some(char_current) = chars.next() {
            match char_current {
                '(' | ')' | '{' | '}' | ',' | '.' | '-' | '+' | ';' | '*' => {
                    let char_str = &char_current.to_string();
                    if let Some(token_type) = keywords.get(char_str.as_str()) {
                        println!("{} {} null", token_type, char_current);
                        self.tokens.push(Token{
                            token_type: token_type.clone(),
                            lexeme: char_str.clone(),
                            literal: None,
                            line: line_nb,
                        });
                    } else {
                        eprintln!(
                            "[line {}] Error: Unexpected character: {}",
                            line_nb, char_current
                        );
                        return_code = 65;
                    }
                }
                '=' => match chars.peek() {
                    Some('=') => {
                       // println!("EQUAL_EQUAL == null");
                        chars.nth(0);
                        self.tokens.push(Token {
                            token_type: TokenType::EqualEqual,
                            lexeme: "==".to_string(),
                            literal: None,
                            line: line_nb,
                        });
                        continue;
                    }
                    _ => {
                       // println!("EQUAL = null");
                        self.tokens.push(Token {
                            token_type: TokenType::Equal,
                            lexeme: "=".to_string(),
                            literal: None,
                            line: line_nb,
                        });
                    }
                },
                '!' => match chars.peek() {
                    Some('=') => {
                        //println!("BANG_EQUAL != null");
                        chars.nth(0);
                        self.tokens.push(Token {
                            token_type: TokenType::BangEqual,
                            lexeme: "!=".to_string(),
                            literal: None,
                            line: line_nb,
                        });
                        continue;
                    }
                    _ => {
                        //println!("BANG ! null");
                        self.tokens.push(Token {
                            token_type: TokenType::Bang,
                            lexeme: "!".to_string(),
                            literal: None,
                            line: line_nb,
                        });
                    }
                },
                '<' => match chars.peek() {
                    Some('=') => {
                        //println!("LESS_EQUAL <= null");
                        chars.nth(0);
                        self.tokens.push(Token {
                            token_type: TokenType::LessEqual,
                            lexeme: "<=".to_string(),
                            literal: None,
                            line: line_nb,
                        });
                        continue;
                    }
                    _ => {
                       // println!("LESS < null");
                        self.tokens.push(Token {
                            token_type: TokenType::Less,
                            lexeme: "<".to_string(),
                            literal: None,
                            line: line_nb,
                        });
                    }
                },
                '>' => match chars.peek() {
                    Some('=') => {
                       // println!("GREATER_EQUAL >= null");
                        chars.nth(0);
                        self.tokens.push(Token {
                            token_type: TokenType::GreaterEqual,
                            lexeme: ">=".to_string(),
                            literal: None,
                            line: line_nb,
                        });
                        continue;
                    }
                    _ => {
                        //println!("GREATER > null");
                        self.tokens.push(Token {
                            token_type: TokenType::Greater,
                            lexeme: ">".to_string(),
                            literal: None,
                            line: line_nb,
                        });
                    }
                },
                '/' => match chars.peek() {
                    Some('/') => {
                        chars.next(); // Consume the '/' character
                        while let Some(&next_char) = chars.peek() {
                            if next_char == '\n' {
                                break;
                            }
                            chars.next(); // Consume characters until end of line
                        }
                    }
                    _ => {
                        //println!("SLASH / null");
                        self.tokens.push(Token {
                            token_type: TokenType::Slash,
                            lexeme: "/".to_string(),
                            literal: None,
                            line: line_nb,
                        });
                    }
                },
                '"' => {
                    let mut string_tok = String::from("");
                    let mut ending = false;
                    while let Some(next_char) = chars.next() {
                        match next_char {
                            '\n' => {
                                eprintln!("[line {}] Error: Unterminated string.", line_nb);
                                line_nb += 1;
                                break;
                            }
                            '"' => {
                                //println!("STRING \"{}\" {}", string_tok, string_tok);
                                ending = true;
                                self.tokens.push(Token {
                                    token_type: TokenType::String,
                                    lexeme: string_tok.clone(),
                                    literal: Some(Literal::String(string_tok.clone())),
                                    line: line_nb,
                                });
                                break;
                            }
                            a => string_tok.push(a),
                        }
                    }
                    if !chars.peek().is_some() && !ending {
                        eprintln!("[line {}] Error: Unterminated string.", line_nb);
                        return_code = 65;
                    }
                }
                ' ' | '\t' | '\r' => continue,
                '\n' => line_nb += 1,
                a if a.is_digit(10) => {
                    let mut out_number = a.to_string();
                    let mut point = false;
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_digit(10) {
                            out_number.push(next_char);
                            chars.next();
                        } else if next_char == '.' && !point {
                            let mut chars_clone = chars.clone();
                            chars_clone.next();
                            if let Some(&next_next_char) = chars_clone.peek() {
                                point = true;
                                out_number.push(next_char);
                                chars.next();
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    if out_number.ends_with('.') {
                        out_number.pop();
                       // println!("DOT . null"); // this is a quick fix and needs refactoring
                    }
                    let out_number_float = out_number.parse::<f64>().unwrap();
                    //println!("NUMBER {} {:?}", out_number, out_number_float);
                    self.tokens.push(Token {
                        token_type: TokenType::Number,
                        lexeme: out_number.clone(),
                        literal: Some(Literal::Number(out_number_float)),
                        line: line_nb,
                    });
                }
                a if a.is_alphanumeric() || a == '_' => {
                    let mut token_out = String::from(a);
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_alphanumeric() || next_char == '_' {
                            token_out.push(next_char);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    if let Some(token_type) = keywords.get(&token_out.as_str()) {
                        //println!("{} {} null", token_type, token_out);
                        self.tokens.push(Token {
                            token_type: token_type.clone(),
                            lexeme: token_out.clone(),
                            literal: None,
                            line: line_nb,
                        });
                    } else {
                        //println!("IDENTIFIER {} null", token_out);
                        self.tokens.push(Token {
                            token_type: TokenType::Identifier,
                            lexeme: token_out,
                            literal: None,
                            line: line_nb,
                        });
                    }
                }
                a => {
                    eprintln!("[line {}] Error: Unexpected character: {}", line_nb, a);
                    return_code = 65;
                }
            }
        }
        //println!("EOF  null");
        if return_code != 0 {
            exit(return_code);
        }
        return return_code;
    }
}

fn initialise_keywords() { // prety skitz way of doing it could be simplified for sure -- for another day
    let mut m = HashMap::new();

    m.insert("(", TokenType::LeftParen);
    m.insert(")", TokenType::RightParen);
    m.insert("{", TokenType::LeftBrace);
    m.insert("}", TokenType::RightBrace);
    m.insert("*", TokenType::Star);
    m.insert(".", TokenType::Dot);
    m.insert(",", TokenType::Comma);
    m.insert("+", TokenType::Plus);
    m.insert("-", TokenType::Minus);
    m.insert("==", TokenType::EqualEqual);
    m.insert("=", TokenType::Equal);
    m.insert("!=", TokenType::BangEqual);
    m.insert("!", TokenType::Bang);
    m.insert("<=", TokenType::LessEqual);
    m.insert("<", TokenType::Less);
    m.insert(">=", TokenType::GreaterEqual);
    m.insert(">", TokenType::Greater);
    m.insert(";", TokenType::SemiColon);
    m.insert("/", TokenType::Slash);
    m.insert("and", TokenType::And);
    m.insert("class", TokenType::Class);
    m.insert("else", TokenType::Else);
    m.insert("false", TokenType::False);
    m.insert("for", TokenType::For);
    m.insert("fun", TokenType::Fun);
    m.insert("if", TokenType::If);
    m.insert("nil", TokenType::Nil);
    m.insert("or", TokenType::Or);
    m.insert("print", TokenType::Print);
    m.insert("return", TokenType::Return);
    m.insert("super", TokenType::Super);
    m.insert("this", TokenType::This);
    m.insert("true", TokenType::True);
    m.insert("var", TokenType::Var);
    m.insert("while", TokenType::While);
    m.insert("EOF", TokenType::Eof);
    unsafe {
        KEYWORDS = Some(m);
    }
}

fn get_keywords() -> &'static HashMap<&'static str, TokenType> {
    unsafe {
        INIT.call_once(|| {
            initialise_keywords();
        });
        KEYWORDS.as_ref().unwrap()
    }
}
