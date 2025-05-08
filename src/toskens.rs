use crate::parser::{Literal, Objects, RenderObject};

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Tokens {
    Bold, // '''
    BoldItalic, // '''''
    Comment, // ## 
    DeletedBar, // --
    DeletedWave, // ~~
    Escape(char), // \
    FixedComment, // @##
    Header(u8), // =
    Italic, // ''
    UnderLine, // __
    Sup, // &&
    Sub, //,,
    TripleClose, // }}}
    TripleOpen, // {{{
    LinkOpen, // ]]
    LinkClose, // [[
    MacroOpen, // [
    MacroClose, // ]
    StarList, // * 
    Reference, // [* ]
    Quote(u8), // >
    Horizon, //수평선 문법이 기억이 안남
    PipeLine, // |
    TablePipe, // ||
    Literal(String), //String
    NewLine, // \n
    Space, // ' '
    Happy, // )
    Sad, // (
    Nop,
    ShBoom,
    Sharp,
} //리다이렉트는 따로 처리할 예정
impl Default for Tokens {
    fn default() -> Self {
        return Tokens::Nop
    }
}
fn sarade () {
    println!("이거 보면 사라다빵 사줘. (뭔지 모름)")
}
impl Tokens {
    pub fn to_literal(&self) -> Objects {
        return Objects::RenderObject(RenderObject::Literal(Literal {literal:self.to_string()}));
    }
    pub fn to_string(&self) -> String {
        match self {
            Tokens::Bold => return String::from("'''"),
            Tokens::BoldItalic => return String::from("'''''"),
            Tokens::Comment => return String::from("##"),
            Tokens::DeletedBar => return String::from("--"),
            Tokens::DeletedWave => return String::from("~~"),
            Tokens::Escape(char) => return format!("\\{}", char),
            Tokens::FixedComment => return String::from("@##"),
            Tokens::Header(level) => return "=".repeat(level.to_owned().into()).to_string(),
            Tokens::Italic => return String::from("''"),
            Tokens::UnderLine => return String::from("__"),
            Tokens::Sup => return String::from("^^"),
            Tokens::Sub => return String::from(",,"),
            Tokens::TripleClose => return String::from("}}}"),
            Tokens::TripleOpen => return String::from("{{{"),
            Tokens::LinkOpen => return String::from("[["),
            Tokens::LinkClose => return String::from("]]"),
            Tokens::MacroOpen => return String::from("["),
            Tokens::MacroClose => return String::from("]"),
            Tokens::StarList => return String::from("*"),
            Tokens::Reference => return String::from("[*"),
            Tokens::Quote(_) => return String::from(">"),
            Tokens::Horizon => return String::from("----"),
            Tokens::PipeLine => return String::from("|"),
            Tokens::TablePipe => return String::from("||"),
            Tokens::Literal(string) => return string.to_owned(),
            Tokens::NewLine => return String::from("\n"),
            Tokens::Space => return String::from(" "),
            Tokens::Happy => return String::from(")"),
            Tokens::Sad => return String::from("("),
            Tokens::Nop => return String::from("wtf"),
            Tokens::ShBoom => return String::from("{{{#!"), //Life could be suck~ Life could be suck~ do dodo do do shboom
            Tokens::Sharp => return String::from("#")
        }
    }
}