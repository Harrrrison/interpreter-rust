mod scanner;
mod parser;
mod evaluator;

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::exit;
use std::string::ParseError;
use scanner::Scanner;
use parser::Parser;
use crate::evaluator::Interpreter;
use crate::scanner::{Literal, Token, TokenType};

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
                let return_code = scanner.scan_and_tokenize(&file_contents);
                for token in scanner.tokens{
                    println!("{}", token)
                }
                println!("EOF  null");
                if return_code != 0 {
                    exit(return_code);
                }
                // println!("{}", scanner.tokens); // not needed I dont think
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
                let mut parsed_file = parser::Parser::new(scanner.tokens);
                let result = parsed_file.parse();
                if result.is_ok() {
                    for token in result{
                        println!("{}", token) // var and fun and that lot arnt actyaly parsed yet so wejsut need to regurgiatete the tokens

                    }

                }else{
                    exit(65)
                }
            } else {
                println!("EOF  null")
            }
        },
        "evaluate" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
                String::new()
            });
            if !file_contents.is_empty() {
                let mut scanner = Scanner::new();
                scanner.scan_and_tokenize(&file_contents);
                let mut parsed_file = parser::Parser::new(scanner.tokens);
                let result = parsed_file.parse();
                let interpreter = Interpreter::new();
                if result.is_ok() {
                    /*if let out = interpreter.interpret(result.clone().unwrap()){
                        match out {
                            Ok(T) => {
                                println!("{:?}", out);
                                exit(0)
                            },
                            Err(e)=> exit(70),
                        }
                    }*/
                    let out = match interpreter.interpret(result.unwrap()) {
                        Ok(T) => {
                            println!("{}", T);
                        },
                        Err(e)=> exit(70),
                    };
                    exit(0)

                }else{
                    exit(65)
                }
            } else {
                println!("EOF  null")
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }


}

/*
fn tokenize(filename: &String) -> Vec<Token> {
    writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        writeln!(io::stderr(), "Failed to read file {}", filename).unwrap();
        String::new()
    });
    if !file_contents.is_empty() {
        let mut scanner = Scanner::new();
        let return_code = scanner.scan_and_tokenize(&file_contents);
        return scanner.tokens
    }
    Err(return_code)
}*/
