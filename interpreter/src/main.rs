use lexer::Lexer;
use parser::Parser;
use std::io::stdin;

use interpreter::eval;
use interpreter::environment::Env;
use std::cell::RefCell;
use std::rc::Rc;

pub fn main() {
    println!("Welcome to monkey interpreter by dmenezes");
    let env: Env = Rc::new(RefCell::new(Default::default()));
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim_end().is_empty() {
            println!("bye");
            std::process::exit(0)
        }

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        match program {
            Ok(node) => {
                match eval(parser::ast::Node::Program(node), &env) { 
                    Ok(res) => println!("{}", res),
                    Err(e) => eprintln!("{}", e),
                }
            },
            Err(e) => eprintln!("parse error: {}", e[0]),
        }
    }
}
