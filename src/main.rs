use std::{
    env, fs,
    io::{self, Write},
};

use crate::interpreter::Interpreter;
use env_logger::Env;

mod environment;
mod expression;
mod interpreter;
mod parser;
mod token;

static mut HAD_ERROR: bool = false;

fn main() {
    env_logger::init();

    if env::args().len() > 2 {
        println!("Usage: rustlox [script]");
    } else if env::args().len() == 2 {
        let path = env::args().nth(1).unwrap();
        run(&fs::read_to_string(path).unwrap());
        if unsafe { HAD_ERROR } {
            return;
        }
    } else {
        prompt();
    }

    // let expr = Expr::Binary {
    //     left: Box::new(Expr::Unary {
    //         operator: token::Token {
    //             ttype: token::TokenType::MINUS,
    //             lexeme: "-".to_owned(),
    //             literal: "".to_owned(),
    //             line: 0,
    //         },
    //         right: Box::new(Expr::Literal {
    //             value: "123".to_owned(),
    //         }),
    //     }),
    //     operator: token::Token {
    //         ttype: token::TokenType::EOF,
    //         lexeme: "*".to_string(),
    //         literal: "".to_string(),
    //         line: 0,
    //     },
    //     right: Box::new(Expr::Grouping {
    //         expression: Box::new(Expr::Literal {
    //             value: "45.67".to_owned(),
    //         }),
    //     }),
    // };
}

fn prompt() {
    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.is_empty() {
            break;
        }

        run(&input);
        unsafe { HAD_ERROR = false };
    }
}

fn run(source: &str) {
    let mut scanner = token::Scanner::new(source);
    let tokens = scanner.scan_tokens();

    // for token in tokens.clone().into_iter() {
    //     println!("{}", token);
    // }

    let mut parser = parser::Parser::new(&tokens);
    let statements = parser.parse();

    // println!("{}", AstPrinter {}.print(&expression));
    Interpreter::new().interpret(&statements);
}

// fn error(line_number: i32, message: &str) {
//     report(line_number, "", message)
// }

// fn report(line_number: i32, location: &str, message: &str) {
//     println!("[line {}] Error{}: {}", line_number, location, message);
//     unsafe { HAD_ERROR = true };
// }
