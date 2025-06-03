use crate::structs::{Expect, Objects};
#[derive(Debug, PartialEq, Clone)]
pub enum RenderObject {
    Link(Link),
    Nop(Vec<Objects>),
    NopString(Expect), //
    NopNopNop,
    EarlyParse((Expect, Vec<Objects>)), //우선순위 처리용
    Syntax(Syntax),
    NamuTriple(NamuTriple),
    Literal(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum LinkType {
    File,
    Hyper,
    Cat,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Link {
    pub to: String,
    pub show: Option<Vec<Objects>>,
    pub link_type: LinkType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Syntax {
    pub language: Languages,
    pub content: String,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Languages {
    Namumark,   //음 이건 쉽게 만들 수 있는게 ast를 뽑아내서 색칠하면 되니까.
    Corriander, //크레이트 나눠서 개발하니까 저쪽은... 그 크레만 뽑아오면 될듯? (아직 파서 개발을 안했데~ 그래서 내가 신통나게 까는중)
    Dolce,      //이건 그냥 플젝 전채를 가져오면 될듯
    NotSupported,
    //UmLang,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamuTriple {
    pub attr: Option<String>,
    pub content: Option<Vec<Objects>>,
    pub triplename: String,
}
