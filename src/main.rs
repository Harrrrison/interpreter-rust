use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;
// use regex::Regex; this is abit studpif and annoying
use std::collections::HashMap;
use std::fmt;
use std::sync::Once;

pub enum Token {
    // need to capitalise
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

    String(String),

    Number(String),

    Identifier(String),

    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::LeftParen => write!(f, "LEFT_PAREN"),
            Token::RightParen => write!(f, "RIGHT_PAREN"),
            Token::LeftBrace => write!(f, "LEFT_BRACE"),
            Token::RightBrace => write!(f, "RIGHT_BRACE"),
            Token::Star => write!(f, "STAR"),
            Token::Dot => write!(f, "DOT"),
            Token::Comma => write!(f, "COMMA"),
            Token::Plus => write!(f, "PLUS"),
            Token::Minus => write!(f, "MINUS"),
            Token::EqualEqual => write!(f, "EQUAL_EQUAL"),
            Token::Equal => write!(f, "EQUAL"),
            Token::BangEqual => write!(f, "BANG_EQUAL"),
            Token::Bang => write!(f, "BANG"),
            Token::LessEqual => write!(f, "LESS_EQUAL"),
            Token::Less => write!(f, "LESS"),
            Token::GreaterEqual => write!(f, "GREATER_EQUAL"),
            Token::Greater => write!(f, "GREATER"),
            Token::SemiColon => write!(f, "SEMICOLON"),
            Token::Slash => write!(f, "SLASH"),
            Token::String(value) => write!(f, "STRING({})", value),
            Token::Number(value) => write!(f, "NUMBER({})", value),
            Token::Identifier(value) => write!(f, "IDENTIFIER({})", value),
            Token::AND => write!(f, "AND"),
            Token::CLASS => write!(f, "CLASS"),
            Token::ELSE => write!(f, "ELSE"),
            Token::FALSE => write!(f, "FALSE"),
            Token::FOR => write!(f, "FOR"),
            Token::FUN => write!(f, "FUN"),
            Token::IF => write!(f, "IF"),
            Token::NIL => write!(f, "NIL"),
            Token::OR => write!(f, "OR"),
            Token::PRINT => write!(f, "PRINT"),
            Token::RETURN => write!(f, "RETURN"),
            Token::SUPER => write!(f, "SUPER"),
            Token::THIS => write!(f, "THIS"),
            Token::TRUE => write!(f, "TRUE"),
            Token::VAR => write!(f, "VAR"),
            Token::WHILE => write!(f, "WHILE"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}

static INIT: Once = Once::new();
static mut KEYWORDS: Option<HashMap<&'static str, Token>> = None;

fn initialise_keywords() {
    let mut m = HashMap::new();
    m.insert("and", Token::AND);
    m.insert("class", Token::CLASS);
    m.insert("else", Token::ELSE);
    m.insert("false", Token::FALSE);
    m.insert("for", Token::FOR);
    m.insert("fun", Token::FUN);
    m.insert("if", Token::IF);
    m.insert("nil", Token::NIL);
    m.insert("or", Token::OR);
    m.insert("print", Token::PRINT);
    m.insert("return", Token::RETURN);
    m.insert("super", Token::SUPER);
    m.insert("this", Token::THIS);
    m.insert("true", Token::TRUE);
    m.insert("var", Token::VAR);
    m.insert("while", Token::WHILE);

    unsafe {
        KEYWORDS = Some(m);
    }
}

fn get_keywords() -> &'static HashMap<&'static str, Token> {
    unsafe {
        INIT.call_once(|| {
            initialise_keywords();
        });
        KEYWORDS.as_ref().unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });

            if !file_contents.is_empty() {
                tokenize(&file_contents);
            } else {
                println!("EOF  null")
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
    // bad practice need to use a seperate file for the lexr secion
    fn tokenize(input: &str) -> i32 {
        let mut return_code = 0;
        let mut line_nb = 1;

        let keywords = get_keywords();
        let mut chars = input.chars().peekable();
        while let Some(char_current) = chars.next() {
            match char_current {
                '(' => println!("LEFT_PAREN ( null"),
                ')' => println!("RIGHT_PAREN ) null"),
                '{' => println!("LEFT_BRACE {{ null"),
                '}' => println!("RIGHT_BRACE }} null"),
                ',' => println!("COMMA , null"),
                '.' => println!("DOT . null"),
                '-' => println!("MINUS - null"),
                '+' => println!("PLUS + null"),
                ';' => println!("SEMICOLON ; null"),
                '*' => println!("STAR * null"),
                '=' => match chars.peek() {
                    Some('=') => {
                        println!("EQUAL_EQUAL == null");
                        chars.nth(0);
                        continue;
                    }
                    _ => println!("EQUAL = null"),
                },
                '!' => match chars.peek() {
                    Some('=') => {
                        println!("BANG_EQUAL != null");
                        chars.nth(0);
                        continue;
                    }
                    _ => println!("BANG ! null"),
                },
                '<' => match chars.peek() {
                    Some('=') => {
                        println!("LESS_EQUAL <= null");
                        chars.nth(0);
                        continue;
                    }
                    _ => println!("LESS < null"),
                },
                '>' => match chars.peek() {
                    Some('=') => {
                        println!("GREATER_EQUAL >= null");
                        chars.nth(0);
                        continue;
                    }
                    _ => println!("GREATER > null"),
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
                    _ => println!("SLASH / null"),
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
                                println!("STRING \"{}\" {}", string_tok, string_tok);
                                ending = true;
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
                        println!("DOT . null"); // this is a stupid and quick fix and will need to be refactored
                    }
                    let out_numeber_float = out_number.parse::<f64>().unwrap();
                    println!("NUMBER {} {:?}", out_number, out_numeber_float);
                }
                a if a.is_alphanumeric() || a == '_' => {
                    // pretty sure this '_' could be
                    // implemented better
                    let mut toke_out = String::from(a);
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_alphanumeric() || next_char == '_' {
                            toke_out.push(next_char);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    if let Some(token_type) = keywords.get(&toke_out.as_str()) {
                        println!("{} {} null", token_type, toke_out)
                    } else {
                        println!("IDENTIFIER {} null", toke_out);
                    }
                }
                a => {
                    eprintln!("[line {}] Error: Unexpected character: {}", line_nb, a);
                    return_code = 65;
                }
            }
        }
        println!("EOF  null");
        if return_code != 0 {
            exit(return_code)
        }
        return return_code;
    }
}
