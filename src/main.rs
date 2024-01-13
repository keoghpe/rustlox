pub mod token;

use std::{
    env, fs,
    io::{self, Write},
};

static mut HAD_ERROR: bool = false;

fn main() {
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
    println!("{}", source);
    // Scanner scanner = new Scanner(source);
    // List<Token> tokens = scanner.scanTokens();
    // For now, just print the tokens.
    //     for (Token token : tokens) {
    //       System.out.println(token);
    // }
}

fn error(line_number: i32, message: &str) {
    report(line_number, "", message)
}

fn report(line_number: i32, location: &str, message: &str) {
    println!("[line {}] Error{}: {}", line_number, location, message);
    unsafe { HAD_ERROR = true };
}
