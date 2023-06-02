mod lexer;
mod token;
mod repl;
mod ast;

fn main() {
    println!("Welcome to the Monke(y) language REPL!");
    repl::start();
}
