use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;

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
                    while let Some(next_char) = chars.next() {
                        match next_char {
                            '\n' => {
                                eprintln!("[line {}] Error: Unterminated string", line_nb);
                                line_nb += 1;
                                break;
                            }
                            '"' => {
                                println!("STRING \"{}\" {}", string_tok, string_tok);
                                break;
                            }
                            a => string_tok.push(a),
                        }
                    }
                    if !chars.peek().is_some() && !string_tok.ends_with('"') {
                        eprintln!("[line {}] Error: Unterminated string", line_nb);
                    }
                }

                ' ' | '\t' | '\r' => continue,
                '\n' => line_nb += 1,
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
