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
    Sharp, //#
    Happy, // )
    Sad, // (
    Nop
} //리다이렉트는 따로 처리할 예정
impl Default for Tokens {
    fn default() -> Self {
        return Tokens::Nop
    }
}
fn sarade () {
    println!("이거 보면 사라다빵 사줘. (뭔지 모름)")
}