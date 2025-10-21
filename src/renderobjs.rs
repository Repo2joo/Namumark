

use crate::structs::{Expect, ListType, NamuMacroType, Objects};
#[derive(Debug, PartialEq, Clone)]

pub enum RenderObject {
  AddBefore(Vec<Objects>),
  Link(Link),
  
  Nop(Vec<Objects>),
  LastRollBack,
  
  NopNopNop,
  
  EarlyParse((Expect, Vec<Objects>)),
  
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
  Left, 
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
  
  pub lvl: usize,
  
  pub folded: bool,
  
  pub content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]

pub struct QuoteLine {
  
  pub lvl: usize,
  
  pub content: Vec<Objects>,
}
#[derive(Debug, PartialEq, Clone)]

pub struct Quote {
  
  pub content: Vec<QuoteLine>,
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

pub enum Languages {
  NotSupported, 
                
}

#[derive(Debug, PartialEq, Clone)]
pub struct NamuTriple {
  pub attr: Option<String>,
  pub content: Option<Vec<Objects>>,
  pub triplename: String,
}
#[derive(Debug, PartialEq, Clone)]
pub struct NamumarkMacro {
  
  pub macroname: String,
  pub macroarg: Option<String>,
  pub macrotype: NamuMacroType,
}
