mod toskens;
mod lexer;
mod parser;
use std::{io::{stdout, Write}, time::Instant};

use parser::{Objects, RenderObject};

use crate::toskens::Tokens;
#[derive(Debug)]
struct Compiler {
    string:String,
    idx:usize,
    tokens:Vec<Tokens>,
    chars:Vec<char>,
    parsed:Vec<RenderObject>,
    parsetemp:Vec<Objects>
}

impl Compiler {
    pub fn new (string:String) -> Compiler {
        let string = string;
        return Compiler {
            string:string.clone(),
            idx:0,
            tokens: Vec::new(),
            chars: Vec::new(),
            parsed: Vec::new(),
            parsetemp: Vec::new(),
        }
    }
}

fn main() {
    let asdf = String::from("빵사줘\\사\\라ㄹㄹ (다 빵)");
    let mut lexer = Compiler::new(asdf);
    let start = Instant::now();
    lexer.lex();
    lexer.parse();
    println!("    {:#?}\n
    Lexed IN: {:#?}", lexer.tokens, start.elapsed());
    stdout().flush();
}

