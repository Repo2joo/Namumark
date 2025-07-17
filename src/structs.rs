use std::{fmt, vec};

use crate::{parser::parse_first, renderobjs::RenderObject};
#[derive(Debug)]
pub struct Compiler {
    pub index: usize,
    pub array: Vec<Objects>,
    pub expected: Vec<Expect>,
    pub lastrollbackindex: Vec<usize>,
    pub fixed_comments:Vec<String>,
    pub redirect:Option<String>
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
    NamuMacro(NamuMacroType)
}
#[derive(Debug, PartialEq, Clone)]
pub enum NamuMacroType {
    YouTube,
    KakaoTV,
    NicoVideo,
    Vimeo,
    NaverTV,
    Include,
    Age,
    DDay,
    PageCount,
    Ruby
}
impl NamuMacroType {
    pub fn to_string(&self) -> String {
        match self {
            NamuMacroType::YouTube => String::from("youtube"),
            NamuMacroType::KakaoTV => String::from("kakaotv"),
            NamuMacroType::NicoVideo => String::from("nicovideo"),
            NamuMacroType::Vimeo => String::from("vimeo"),
            NamuMacroType::NaverTV => String::from("navertv"),
            NamuMacroType::Include => String::from("include"),
            NamuMacroType::Age => String::from("age"),
            NamuMacroType::DDay => String::from("dday"),
            NamuMacroType::PageCount => String::from("pagecount"),
            NamuMacroType::Ruby => String::from("ruby"),
        }
    }
}
impl Compiler {
    pub fn from(string: String) -> Compiler {
        let mut compiler = Compiler {
            index: 0,
            array: Vec::new(),
            expected: Vec::new(),
            lastrollbackindex: Vec::new(),
            fixed_comments: vec![String::new()],
            redirect:None
        };
        for char in string.chars() {
            compiler.array.push(Objects::Char(char));
        }
        return compiler;
    }
    pub fn parse(&mut self) {
        parse_first(self, Expect::None);
        self.fixed_comments.pop();
    }
    pub fn get(&mut self, idx: usize) -> Option<&Objects> {
        self.array.get(idx)
    }
    pub fn current(&self) -> Option<Objects> {
        self.array.get(self.index).cloned()
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
    pub fn peak_line(&mut self, str: &str) -> bool {
        let mut idx = 0;
        if self.index == 0 || self.get(self.index-1) == Some(&Objects::Char('\n')) {
            idx +=1;
        } else {
            return false;
        }
        for ch in str.chars() {
            if let Some(Objects::Char(cha)) = self.get(self.index + idx -1) {
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
