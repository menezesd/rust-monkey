use lexer::Lexer;
use parser::Parser;
use std::io::stdin;

pub fn main() {
    println!("Welcome to monkey parser by dmenezes");
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim_end().is_empty() {
            println!("bye");
            std::process::exit(0)
        }

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        println!("{}", program);
    }
}
