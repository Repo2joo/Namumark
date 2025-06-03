use std::clone;

use crate::{
    parser::parse_first,
    renderobjs::{Link, RenderObject, Syntax},
};

#[derive(Debug)]
pub struct Compiler {
    pub index: usize,
    pub array: Vec<Objects>,
    pub expected: Vec<Expect>,
    pub lastrollbackindex: Vec<usize>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Objects {
    Char(char),
    RenderObject(RenderObject),
}
#[derive(Debug, PartialEq, Clone)]
pub enum Expect {
    None,
    Link,
    Link2,
    SyntaxTriple,
    TripleWithNamuMark,
    TripleWithNamuMark2,
    TripleWithNamuMark3,
    JustTriple,
}
impl Compiler {
    pub fn from(string: String) -> Compiler {
        let mut compiler = Compiler {
            index: 0,
            array: Vec::new(),
            expected: Vec::new(),
            lastrollbackindex: Vec::new(),
        };
        for char in string.chars() {
            compiler.array.push(Objects::Char(char));
        }
        return compiler;
    }
    pub fn parse(&mut self) {
        parse_first(self, Expect::None);
    }
    fn get(&mut self, idx: usize) -> Option<&Objects> {
        self.array.get(idx)
    }
    pub fn current(&mut self) -> Option<&Objects> {
        self.array.get(self.index)
    }
    pub fn peak(&mut self, str: &str) -> bool {
        let mut idx = 0;
        for ch in str.chars() {
            if let Some(Objects::Char(cha)) = self.get(self.index + idx) {
                if ch.to_lowercase().to_string() != *cha.to_lowercase().to_string() {
                    return false;
                }
            } else {
                return false;
            }
            idx += 1;
        }
        return true;
    }
}
