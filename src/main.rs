#![feature(slice_split_once)] // fuck
mod toskens;
mod lexer;
mod parser;
mod render;
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
    let asdf = String::from("{{{#!");
    let mut lexer = Compiler::new(asdf);
    let start = Instant::now();
    lexer.lex();
    println!("{:#?}\n
    Lexed IN: {:#?}", lexer.tokens, start.elapsed());
    let start2 = Instant::now();
    lexer.parse();
    println!("{:#?}\n
    Lexed IN: {:#?}", lexer.parsetemp, start2.elapsed());
    stdout().flush();
}

