
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::RegexSet;
use regex::Regex;
use std::fmt;
mod tables;

pub enum Tokens {
    Add,Sub,Mult,Div,Mod,
    Jal,Hi,Lo,
    Hptr,Sptr,Link,
    Zero,Prnt,
    Gtr,Less,Equal,
    Register(String),Syscall,
    Mv,Cpy,Set,
    Data,Code,Flag(String),Hex,Num(String),
    And,Or,Not,
    Jmp,Nop, Decl, Var,
    Load,Store, Str,
    Lbkt, Rbkt, Comma
}
impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Tokens::Add => write!(f, "add"),
            Tokens::Sub => write!(f, "sub"),
            Tokens::Mult => write!(f, "mult"),
            Tokens::Div => write!(f, "div"),
            Tokens::Mod => write!(f, "mod"),
            Tokens::Jal => write!(f, "jal"),
            Tokens::Hi => write!(f, "$hi"),
            Tokens::Lo => write!(f, "$lo"),
            Tokens::Hptr => write!(f, "$hptr"),
            Tokens::Sptr => write!(f, "$sptr"),
            Tokens::Link => write!(f, "Link"),
            Tokens::Zero => write!(f, "$zero"),
            Tokens::Prnt => write!(f, "$p"),
            Tokens::Gtr => write!(f, "gtr"),
            Tokens::Less => write!(f, "less"),
            Tokens::Equal => write!(f, "eql"),
            Tokens::Register(val) => write!(f, "Custom Register {}", val),
            Tokens::Syscall => write!(f, "Syscall"),
            Tokens::Mv => write!(f, "mv"),
            Tokens::Cpy => write!(f, "cpy"),
            Tokens::Set => write!(f, "set"),
            Tokens::Data => write!(f, "data"),
            Tokens::Code => write!(f, "code"),
            Tokens::Flag(val) => write!(f, "Custom Flag {}", val),
            Tokens::Hex => write!(f, "Custom Hex"),
            Tokens::Num(val) => write!(f, "Custom Number {}", val),
            Tokens::And => write!(f, "and"),
            Tokens::Or => write!(f, "or"),
            Tokens::Not => write!(f, "not"),
            Tokens::Jmp => write!(f, "jmp"),
            Tokens::Nop => write!(f, "No Op"),
            Tokens::Load => write!(f, "load"),
            Tokens::Store => write!(f, "store"),
            Tokens::Lbkt => write!(f, "["),
            Tokens::Rbkt => write!(f, "]"),
            Tokens::Comma => write!(f, ","),
            Tokens::Decl => write!(f, "="),
            Tokens::Str => write!(f, "String"),
            Tokens::Var => write!(f, "Var"),
        }
    }
}


pub fn lex(filename: String) -> Vec<Tokens> {
    let mut token:Vec<Tokens> = Vec::new();
    let operations = RegexSet::new(&[
        r"add", r"sub", r"mult", r"div", r"mod",
        r"jal", r"gtr", r"less", r"eql", r"mv", r"cp",
        r"set", r"jmp", r"nop", r"load", r"store", r"syscall",
        r"and", r"or", r"not",
        ]).unwrap();
    let sreg = RegexSet::new(&[
        r"\$hi", r"\&lo", r"\$zero", r"\$p",
        r"\$hptr", r"\$sptr", r"\$link",
        ]).unwrap();
    //let register = Regex::new(r"\$[a-z]+[0-9]+").unwrap();
    let lang = RegexSet::new(&[
        r"data:", r"code:",
        r"\$[[:alpha:]]+[[:digit:]]+", r"[[:alpha:]]+:", r"\[", r"\]", r"=",
        r"'[[[:alnum:]]|[[:space:]]|[[:punct:]]]+'", r"![[:alpha:]]+",]).unwrap();
    let number = RegexSet::new(&[
        r"[[:digit:]]+",
        ]).unwrap();

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate(){

        let line = line.unwrap();
/*
Add,Sub,Mult,Div,Mod,
Jal,Hi,Lo,
Hptr,Sptr,Link,
Zero,Prnt,
Gtr,Less,Equal,
Register,Syscall,
Mv,Cpy,Set,
Data,Code,Flag,Hex,Num,
And,Or,Not,
Jmp,Nop,
Load,Store,
Lbkt, Rbkt,
Colon, Comma
*/
        for word in line.split_whitespace(){
            let opmatches:Vec<_> = operations.matches(word).into_iter().collect();
            let sregmatch:Vec<_> = sreg.matches(word).into_iter().collect();
            let langmatch:Vec<_> = lang.matches(word).into_iter().collect();
            let numbermatch:Vec<_> = number.matches(word).into_iter().collect();
            //let test = "Test value " + word ;
            //println!("{}",word);
            if opmatches.len() == 0 && sregmatch.len() == 0 && langmatch.len() == 0 && numbermatch.len() ==0 {
                let test = "Lexer Error with ".to_string() + word;
                println!("{}",test);
                let build = tables::ErrorMessage{message:test.to_string(), isError:true};
            }


            if opmatches.len() > 0 {
                match opmatches[0] {
                    0 => token.push(Tokens::Add),
                    1 => token.push(Tokens::Sub),
                    2 => token.push(Tokens::Mult),
                    3 => token.push(Tokens::Div),
                    4 => token.push(Tokens::Mod),
                    5 => token.push(Tokens::Jal),
                    6 => token.push(Tokens::Gtr),
                    7 => token.push(Tokens::Less),
                    8 => token.push(Tokens::Equal),
                    9 => token.push(Tokens::Mv),
                    10 => token.push(Tokens::Cpy),
                    11 => token.push(Tokens::Set),
                    12 => token.push(Tokens::Jmp),
                    13 => token.push(Tokens::Nop),
                    14 => token.push(Tokens::Load),
                    15 => token.push(Tokens::Store),
                    16 => token.push(Tokens::Syscall),
                    17 => token.push(Tokens::And),
                    18 => token.push(Tokens::Or),
                    19 => token.push(Tokens::Not),

                    _ => println!("Error"),
                }
            }
            else if sregmatch.len() > 0{
                match sregmatch[0]{
                    0 => token.push(Tokens::Hi),
                    1 => token.push(Tokens::Lo),
                    2 => token.push(Tokens::Zero),
                    3 => token.push(Tokens::Prnt),
                    4 => token.push(Tokens::Sptr),
                    5 => token.push(Tokens::Hptr),
                    6 => token.push(Tokens::Link),
                    _ => println!("Error"),
                }
            }
            else if langmatch.len() > 0 {
                match langmatch[0]{
                    //0 => token.push(Tokens::Hex),
                    //0 => token.push(Tokens::Num),
                    0 => token.push(Tokens::Data),
                    1 => token.push(Tokens::Code),
                    2 => token.push(Tokens::Register(word.to_string())),
                    3 => token.push(Tokens::Flag(word.to_string())),
                    4 => token.push(Tokens::Lbkt),
                    5 => token.push(Tokens::Rbkt),
                    6 => token.push(Tokens::Decl),
                    7 => token.push(Tokens::Str),
                    8 => token.push(Tokens::Var),
                    _ => println!("Error"),
                }
            }
            else {
                token.push(Tokens::Num(word.to_string()));
            }

            //println!("{}. {}", index+1, word);
        }


    }
    for each in token.iter() {
        println!("{}",each);
    }
    return token;
}
