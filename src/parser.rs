#[path = "./lexer.rs"] pub mod lexer;
pub use crate::lexer::Tokens;


pub fn parse(tokens: Vec<Tokens>) -> bool{
    let instructions = [Tokens::Add, Tokens::Sub, Tokens::Mult, Tokens::Div, Tokens::Mod, Tokens::Jal, Tokens::Gtr,
    Tokens::Less, Tokens::Equal, Tokens::Mv, Tokens::Cpy, Tokens::Set, Tokens::Jmp, Tokens::Nop, Tokens::Load,
    Tokens::Store, Tokens::Syscall, Tokens::And, Tokens::Or, Tokens::Not];
    let mut currentop:Tokens;
    let index: usize = 0;
    let lexeme:String;
    let mut test = vec![Tokens::Code, Tokens::Add, Tokens::Prnt, Tokens::Prnt, Tokens::Prnt];
    for each in &tokens {
        // while tokens is not empty
        // match tokens[ the other vector .len() -1]

        // code token => set other vector to accept an instruction or Flag
        // remove tokens[other.len()-1]

        // create slice from 0 to other.len()
        // match the 2 vectors
        // remove tokens from 0 to other.len()
        //

        // there has to be a better way
        // check if slice has tokens at 0 1 2 and 3
        // but would have to write this for each instruction

        // use slices instead then at the end advance the slice
        if tokens[0] == Tokens::Code {
            for each in instructions {
                if tokens[1] == each{
                    match each{
                        Tokens::Add => {}
                    }
                }
            }
        }

    match each {
        Tokens::Code => {println!("It works"); test = vec![Tokens::Add];},
        Tokens::Num(lexeme) => {println!("pls work {}", lexeme); test = vec![Tokens::Prnt];},
        _ => println!("IDK"),
    }

}
return true;
    //while index < tokens.len(){

    //}
}
