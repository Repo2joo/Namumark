

use crate::structs::{Expect, ListType, NamuMacroType, Objects};
#[derive(Debug, PartialEq, Clone)]
///링크, 삼중괄 등의 변종을 가지고 있습니다.
pub enum RenderObject {
  AddBefore(Vec<Objects>),
  Link(Link),
  ///파싱 과정중에 쓰이는 것으로 신경은 안쓰셔도 됩니다.
  Nop(Vec<Objects>),
  LastRollBack,
  ///파싱 과정중에 쓰이는 것으로 신경은 안쓰셔도 됩니다.
  NopNopNop,
  ///파싱 과정중에 쓰이는 것으로 신경은 안쓰셔도 됩니다.
  EarlyParse((Expect, Vec<Objects>)),
  ///파싱 과정중에 쓰이는 것으로 신경은 안쓰셔도 됩니다.
  EarlyParseRollBack(Expect),
  NamuTriple(NamuTriple),
  Literal(String),
  NamumarkMacro(NamumarkMacro),
  List(List),
  ListLine(ListLine),
  Quote(Quote),
  QuoteLine(QuoteLine),
  Heading(Heading),
  Color(Color),
  Plus(Plus),
  Minus(Minus),
  Reference(Reference),
  Bold(Bold),
  Itelic(Itelic),
  DelTidal(DelTidal),
  DelBar(DelBar),
  UnderLine(UnderLine),
  Upper(Upper),
  Lower(Lower),
  Table(Table),
  TableRow(Vec<Objects>),
}
#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
  Left, //어허 나는 왼솑더가지ㅓ아니러ㅏㅣ
  Center,
  Right,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Table {
  pub table_row: Vec<TableRow>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct TableRow {
  pub table_cell: Vec<TableCell>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct TableCell {
  pub attribute: CellAttribute,
  pub content: Vec<Objects>,
  pub allign: Option<Direction>,
  pub height_align: Option<Altitude>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Altitude {
  High,
  Middle,
  Low,
}
#[derive(Debug, PartialEq, Clone, Default)]
pub struct CellAttribute {
  pub nopad: bool,
  pub rowspan: Option<String>,
  pub height: Option<String>,
  pub width: Option<String>,
  pub keepall: bool,
  pub colkeepall: bool,
  pub rowkeepall: bool,
  pub bgcolor: LightNightColor,
  pub table_bordercolor:LightNightColor,
  pub bordercolor:LightNightColor,
  pub color:LightNightColor,
  pub table_bgcolor:LightNightColor,
  pub row_color:LightNightColor,
  pub col_color:LightNightColor,
  pub row_bgcolor:LightNightColor,
  pub col_bgcolor:LightNightColor,
  pub table_color:LightNightColor
}
type LightNightColor = Option<(Option<String>, Option<String>)>;
#[derive(Debug, PartialEq, Clone)]
pub struct Bold {
  pub content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Itelic {
  pub content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct DelTidal {
  pub content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct DelBar {
  pub content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct UnderLine {
  pub content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Upper {
  pub content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Lower {
  pub content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Reference {
  pub(crate) name: Option<String>,
  pub(crate) content: Option<Vec<Objects>>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Plus {
  pub(crate) how: u8,
  pub(crate) content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Minus {
  pub(crate) how: u8,
  pub(crate) content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Color {
  pub first: String,
  pub second: Option<String>,
  pub content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Heading {
  ///=의 개수
  pub lvl: usize,
  ///접힘 여부
  pub folded: bool,
  ///=와 =사이의 내용. 나무마크가 들어갈 수 있습니다.
  pub content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]
///인용문의 라인
pub struct QuoteLine {
  ///인용문의 중첩 레벨
  pub lvl: usize,
  ///인용문의 컨텐츠
  pub content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]
///인용문의 객체. 이어진 라인끼리 하나로 묶습니다.
pub struct Quote {
  ///QuoteLine들
  pub content: Vec<QuoteLine>,
}
#[derive(Debug, PartialEq, Clone)]
///리스트의 라인
pub struct ListLine {
  pub lvl: usize,
  pub content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum LinkType {
  ///파일
  File,
  ///hyperLink의 줄임말으로, 일반적인 링크입니다.
  Hyper,
  ///Category의 줄임말입니다.
  Cat,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Link {
  ///링크 | 왼쪽에 있는 내용
  pub to: String,
  ///|오른쪽에 있는 내용
  pub show: Vec<Objects>,
  ///링크의 타입 [`LinkType`] 참조
  pub link_type: LinkType,
}
#[derive(Debug, PartialEq, Clone)]
pub struct List {
  /// 1.#1이거였나
  pub from: Option<usize>,
  pub listtype: ListType,
  pub content: Vec<ListLine>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Syntax {
  pub language: Languages,
  pub content: String,
}
#[derive(Debug, PartialEq, Clone)]
///아직 지원 하는 언어가 없어요
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
  pub macrotype: NamuMacroType,
}
