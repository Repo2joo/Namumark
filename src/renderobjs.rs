use crate::structs::{Expect, ListType, Objects};
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
  NamumarkMacro(NamumarkMacro),
  List(List),
  ListLine(ListLine),
}
#[derive(Debug, PartialEq, Clone)]
pub struct ListLine {
  pub lvl: usize,
  pub content: Vec<Objects>,
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
  pub show: Vec<Objects>,
  pub link_type: LinkType,
}
#[derive(Debug, PartialEq, Clone)]
pub struct List {
  pub from: usize,
  pub listtype: ListType,
  pub content: Vec<ListLine>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Syntax {
  pub language: Languages,
  pub content: String,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Languages {
  NotSupported, //지원하는 언어: 현제 없음
                //UmLang,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamuTriple {
  pub attr: Option<String>,
  pub content: Option<Vec<Objects>>,
  pub triplename: String,
}
#[derive(Debug, PartialEq, Clone)]
pub struct NamumarkMacro {
  //웬지 예약어랑 곂칠듯
  pub macroname: String,
  pub macroarg: Option<String>,
}
