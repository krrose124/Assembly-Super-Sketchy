mod lexer;
use std::env;
use lexer::lex;

fn main(){
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    lex(filename.to_string());
}
