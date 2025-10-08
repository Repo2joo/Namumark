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
  Lower(Lower)
}
#[derive(Debug, PartialEq, Clone)]
pub struct Bold {
 pub content:Vec<Objects>
}
#[derive(Debug, PartialEq, Clone)]
pub struct Itelic {
  pub content:Vec<Objects>
}
#[derive(Debug, PartialEq, Clone)]
pub struct DelTidal {
  pub content:Vec<Objects>
}
#[derive(Debug, PartialEq, Clone)]
pub struct DelBar {
  pub content:Vec<Objects>
}
#[derive(Debug, PartialEq, Clone)]
pub struct UnderLine {
  pub content:Vec<Objects>
}
#[derive(Debug, PartialEq, Clone)]
pub struct Upper {
  pub content:Vec<Objects>
}
#[derive(Debug, PartialEq, Clone)]
pub struct Lower {
  pub content:Vec<Objects>
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
impl InnerToString for RenderObject {
  fn to_string(&self) -> String {
    let mut result = String::new();
    match self {
      //두글자로 줄이기 달인😎😎😎
      RenderObject::Link(lk) => {
        result.push_str("[[");
        result.push_str(&lk.to);
        if !lk.show.is_empty() {
          result.push('|');
          result.push_str(&lk.show.to_string());
        }
        result.push_str("]]");
      }
      RenderObject::NamuTriple(nt) => {
        let nt = nt.clone();
        result.push_str("{{{#!");
        result.push_str(&nt.triplename);
        result.push(' ');
        if nt.attr.is_some() {
          result.push_str(&nt.attr.unwrap());
        }
        result.push('\n');
        if nt.content.is_some() {
          result.push_str(&nt.content.unwrap().to_string());
        }
        result.push_str("}}}");
      }
      //이러면 경고는 줄이고 나중에 찾아와서 다 하겠지?
      //씨발놈
      RenderObject::Literal(lt) => {
        result.push_str("{{{");
        result.push_str(lt);
        result.push_str("}}}");
      }
      RenderObject::NamumarkMacro(nm) => {
        result.push('[');
        if nm.macroarg.is_none() {
          result.push_str(&nm.macroname);
          result.push(']');
        } else {
          result.push_str(&nm.macroname);
          result.push('(');
          result.push_str(nm.macroarg.as_ref().unwrap());
          result.push_str(")]");
        }
      }
      RenderObject::List(lt) => {
        let mut a = true;
        if lt.from.is_some() {
          a = false;
        }
        for i in lt.content.clone() {
          result.push_str(&" ".repeat(i.lvl));
          match lt.listtype {
            ListType::Hangul => {
              result.push_str("가.");
            }
            ListType::AlphaSmall => {
              result.push_str("a.");
            }
            ListType::AlphaBig => {
              result.push_str("A.");
            }
            ListType::RomanBig => {
              result.push_str("I.");
            }
            ListType::RomanSmall => {
              result.push_str("i.");
            }
            ListType::Arabia => {
              result.push_str("1.");
            }
            ListType::List => {
              result.push_str("*.");
            }
          }
          if !a {
            result.push('#');
            result.push_str(lt.from.unwrap().to_string().as_str());
            a = true;
          }
          result.push(' ');
          result.push_str(&i.content.to_string());
          result.push('\n');
        }
      }
      RenderObject::Quote(qt) => {}
      #[allow(unused)]
      RenderObject::Heading(hd) => {}
      #[allow(unused)]
      RenderObject::Color(cl) => {}
      #[allow(unused)]
      RenderObject::Plus(pl) => {}
      #[allow(unused)]
      RenderObject::Minus(mn) => {}
      #[allow(unused)]
      RenderObject::Reference(rf) => {}
      _ => {
        panic!("how?");
      }
    }
    result
  }
}
#[allow(dead_code)]
trait InnerToString {
  fn to_string(&self) -> String;
}
impl InnerToString for Vec<Objects> {
  fn to_string(&self) -> String {
    let mut result = String::new();
    let mut index = 0;
    loop {
      match self.get(index) {
        Some(Objects::Char(ch)) => {
          result.push(*ch);
        }
        Some(Objects::RenderObject(rdobj)) => {
          result.push_str(&rdobj.to_string());
        }
        None => {
          break;
        }
      }
      index += 1;
    }
    result
  }
}
