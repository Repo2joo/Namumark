use crate::parser::parse_first;

#[warn(irrefutable_let_patterns)] //ㅖㅖ..
#[derive(Debug)]
pub struct Compiler {
    index: usize,
    pub array: Vec<Objects>,
    expected: Vec<Expect>,
    pub lastrollbackindex:Vec<usize>
}
#[derive(Debug, PartialEq)]
pub enum Objects {
    Char(char),
    RenderObject(RenderObject),
}
#[derive(Debug, PartialEq)]
pub enum RenderObject {
    Link(Link),
    Nop(Vec<Objects>),
    NopForLink,
    NopNopNop
}
#[derive(Debug, PartialEq)]
pub enum LinkType {
    File,
    Hyper,
    Cat,
}
#[derive(Debug, PartialEq)]
pub struct Link {
    pub to: String,
    pub show: Option<Vec<RenderObject>>,
    pub link_type: LinkType,
}

#[derive(Debug, PartialEq)]
pub enum Expect {
    None,
    Link,
    Link2,
}
impl Compiler {
    pub fn from(string: String) -> Compiler {
        let mut compiler = Compiler {
            index: 0,
            array: Vec::new(),
            expected: Vec::new(),
            lastrollbackindex:Vec::new()
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
    fn current(&mut self) -> Option<&Objects> {
        self.array.get(self.index)
    }
}