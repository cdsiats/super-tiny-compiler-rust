use lexer::Lexer;

mod lexer;
mod parser;
mod tokens;

fn main() {
    let mut lexer = Lexer::new("(add 2 4)".to_string());
    lexer.tokenize();
}
