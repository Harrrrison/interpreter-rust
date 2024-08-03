mod scanner;
mod parser;

use std::env;
use std::fs;
use std::io::{self, Write};
use scanner::Scanner;
use parser::Parser;
use crate::scanner::TokenType;

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
            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
//TODO: implement the "parse" command
            if !file_contents.is_empty() {
                let mut scanner = Scanner::new();
                scanner.scan_and_tokenize(&file_contents);
                println!("{:?}", scanner.tokens); // not needed I dont think
            } else {
                println!("EOF  null")
            }
        }
        "parse" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            if !file_contents.is_empty() {
                let mut scanner = Scanner::new();
                scanner.scan_and_tokenize(&file_contents);
                //println!("{:?}", scanner.tokens); // not needed I dont think
                let mut parsed_file = parser::Parser::new(scanner.tokens);
                parsed_file.parse();
                for token in &parsed_file.tokens{
                    if token.token_type == TokenType::Class{
                        println!("{}", token);
                    }
                    println!("{}", token.lexeme)



                }
            } else {
                println!("EOF  null")
            }
        },
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
