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
        match self {
            Tokens::Bold => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("'''")})),
            Tokens::BoldItalic => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("'''''")})),
            Tokens::Comment => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("##")})),
            Tokens::DeletedBar => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("--")})),
            Tokens::DeletedWave => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("~~")})),
            Tokens::Escape(char) => return Objects::RenderObject(RenderObject::Literal(Literal {literal:format!("\\{}", char)})),
            Tokens::FixedComment => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("@##")})),
            Tokens::Header(level) => return Objects::RenderObject(RenderObject::Literal(Literal {literal:"=".repeat(level.to_owned().into()).to_string()})),
            Tokens::Italic => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("''")})),
            Tokens::UnderLine => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("__")})),
            Tokens::Sup => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("^^")})),
            Tokens::Sub => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from(",,")})),
            Tokens::TripleClose => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("}}}")})),
            Tokens::TripleOpen => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("{{{")})),
            Tokens::LinkOpen => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("[[")})),
            Tokens::LinkClose => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("]]")})),
            Tokens::MacroOpen => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("[")})),
            Tokens::MacroClose => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("]")})),
            Tokens::StarList => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("*")})),
            Tokens::Reference => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("[*")})),
            Tokens::Quote(_) => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from(">")})),
            Tokens::Horizon => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("----")})),
            Tokens::PipeLine => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("|")})),
            Tokens::TablePipe => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("||")})),
            Tokens::Literal(string) => return Objects::RenderObject(RenderObject::Literal(Literal {literal:string.to_owned()})),
            Tokens::NewLine => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("\n")})),
            Tokens::Space => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from(" ")})),
            Tokens::Happy => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from(")")})),
            Tokens::Sad => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("(")})),
            Tokens::Nop => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("wtf")})),
            Tokens::ShBoom => return Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("#!")})), //Life could be suck~ Life could be suck~ do dodo do do shboom
        }
    }
}