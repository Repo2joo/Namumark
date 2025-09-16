use std::vec;

use crate::{parser_first::parse_first, renderobjs::RenderObject};
#[derive(Debug)]
///사실 여기 있는거중에 fixed_comments랑 redirect정도만 pub (crate)lic으로 하면 되었습니다.
///그니까 그거 두 개만 알아두시면 됩니다.
pub struct Compiler {
  pub (crate) index: usize,
  pub  array: Vec<Objects>,
  pub (crate) expected: Vec<Expect>,
  pub (crate) lastrollbackindex: Vec<usize>,
  ///고정주석이 있으면 벡터에 추가됩니다. is_empty로 확인하시는 것을 추천드립니다.
  pub fixed_comments: Vec<String>,
  ///리다이렉트가 있으면 어디로 가야하는지 문자열이 저장됩니다. 리다이렉트랑 고정주석 두 개 다 있다면 리다이렉트를 우선적으로 처리하시는 것을 추천드립니다.
  pub redirect: Option<String>,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Objects {
  ///말그대로 문자열 하나입니다. 유니코드든 아스키든 하나가 char 하나로 취급되는것로 알고있습니다.
  Char(char),
  ///해더, 메크로, 삼단중괄호 문법 등등을 하나의 변종으로 묶었습니다. {{{}}}안에 있는 문자열은 여기에 들어갑니다. <code></code>안에 넣어야 하기 때문입니다.
  RenderObject(RenderObject),
}
#[derive(Debug, PartialEq, Clone)]
///파싱 과정중에 쓰이는 것으로 신경은 안쓰셔도 됩니다.
pub (crate) enum Expect {
  None,
  Link,
  Link2,
  SyntaxTriple,
  TripleWithNamuMark,
  TripleWithNamuMark2,
  TripleWithNamuMark3,
  JustTriple,
  NamuMacro(NamuMacroType),
  List(usize),
  Quote(usize),
  Heading(usize),
  Color,
  Plus,
  Minus,
}
#[derive(Debug, PartialEq, Clone)]
/// *, 1., I, 등등의 리스트의 타입을 나타내는 enum입니다. 한글은 원작에서 지원하는지 기억이 안나서 그냥 넣었습니다.
pub enum ListType {
  ///가. 나. 다. ...
  Hangul,
  ///a. b. c. ...
  AlphaSmall,
  ///A. B. C. ...
  AlphaBig,
  ///I. II. III. ...
  RomanBig,
  ///i. ii. iii. ...
  RomanSmall,
  ///번호가 없는 리스트 문법입니다
  List,
  ///1. 2. 3. ...
  Arabia,
}
#[derive(Debug, PartialEq, Clone)]
///메크로의 타잎을 정의해두었습니다. 아마 이름만 봐도 알만할것입니다.
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
  Ruby,
}
impl ToString for NamuMacroType {
  ///파싱 과정중에 쓰이는 것으로 신경은 안쓰셔도 됩니다.
  fn to_string(&self) -> String {
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
  ///파싱 객체를 반환합니다. From trait을 만족시키기 귀찮습니다. 문자열을 넣으면 됩니다.
  pub fn from(string: String) -> Compiler {
    let mut compiler = Compiler {
      index: 0,
      array: Vec::new(),
      expected: Vec::new(),
      lastrollbackindex: Vec::new(),
      fixed_comments: vec![String::new()],
      redirect: None,
    };
    for char in string.chars() {
      compiler.array.push(Objects::Char(char));
    }
    return compiler;
  }
  ///compiler::from("asf").parse() 이런식으로 호출해주시면 됩니다.
  pub fn parse(&mut self) {
    parse_first(self, Expect::None);
    self.fixed_comments.pop();
  }
  ///파싱 과정중에 쓰이는 것으로 신경은 안쓰셔도 됩니다.
  pub (crate) fn get(&mut self, idx: usize) -> Option<&Objects> {
    self.array.get(idx)
  }
  ///파싱 과정중에 쓰이는 것으로 신경은 안쓰셔도 됩니다.
  pub (crate) fn current(&self) -> Option<Objects> {
    self.array.get(self.index).cloned()
  }
  ///파싱 과정중에 쓰이는 것으로 신경은 안쓰셔도 됩니다.
  pub (crate) fn peak(&mut self, str: &str) -> bool {
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
  ///파싱 과정중에 쓰이는 것으로 신경은 안쓰셔도 됩니다.
  pub (crate) fn peak_line(&mut self, str: &str) -> bool {
    let mut idx = 0;
    if self.index == 0 || self.get(self.index - 1) == Some(&Objects::Char('\n')) {
      idx += 1;
    } else {
      return false;
    }
    for ch in str.chars() {
      if let Some(Objects::Char(cha)) = self.get(self.index + idx - 1) {
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
  ///파싱 과정중에 쓰이는 것으로 신경은 안쓰셔도 됩니다.
  pub (crate) fn peak_repeat_line(&mut self, ch: char, end: Option<&str>) -> (bool, usize) {
    if self.index == 0 || self.get(self.index - 1) == Some(&Objects::Char('\n')) {
      let mut idx = 0;
      loop {
        if self.get(self.index + idx) == Some(&Objects::Char(ch)) {
          idx += 1;
        } else if end == None && idx != 0 {
          return (true, idx);
        } else if end.is_some()
          && end.unwrap().len() + self.index + idx + 1 <= self.array.len()
          && 에휴_진짜_왜그럼(
            &self.array[self.index + idx..end.unwrap().len() + self.index + idx],
          ) == String::from(end.unwrap())
          && idx != 0
        {
          println!("a");
          return (true, idx);
        } else {
          return (false, 0);
        }
      }
    } else {
      (false, 0)
    }
  }
  pub (crate) fn is_color(&self) -> bool {
    let munjayeol = "{{{#";
    let colors: [&str; 148] = [
      "aliceblue",
      "antiquewhite",
      "aqua",
      "aquamarine",
      "azure",
      "beige",
      "bisque",
      "black",
      "blanchedalmond",
      "blue",
      "blueviolet",
      "brown",
      "burlywood",
      "cadetblue",
      "chartreuse",
      "chocolate",
      "coral",
      "cornflowerblue",
      "cornsilk",
      "crimson",
      "cyan",
      "darkblue",
      "darkcyan",
      "darkgoldenrod",
      "darkgray",
      "darkgrey",
      "darkgreen",
      "darkkhaki",
      "darkmagenta",
      "darkolivegreen",
      "darkorange",
      "darkorchid",
      "darkred",
      "darksalmon",
      "darkseagreen",
      "darkslateblue",
      "darkslategray",
      "darkslategrey",
      "darkturquoise",
      "darkviolet",
      "deeppink",
      "deepskyblue",
      "dimgray",
      "dimgrey",
      "dodgerblue",
      "firebrick",
      "floralwhite",
      "forestgreen",
      "fuchsia",
      "gainsboro",
      "ghostwhite",
      "gold",
      "goldenrod",
      "gray",
      "grey",
      "green",
      "greenyellow",
      "honeydew",
      "hotpink",
      "indianred",
      "indigo",
      "ivory",
      "khaki",
      "lavender",
      "lavenderblush",
      "lawngreen",
      "lemonchiffon",
      "lightblue",
      "lightcoral",
      "lightcyan",
      "lightgoldenrodyellow",
      "lightgray",
      "lightgrey",
      "lightgreen",
      "lightpink",
      "lightsalmon",
      "lightseagreen",
      "lightskyblue",
      "lightslategray",
      "lightslategrey",
      "lightsteelblue",
      "lightyellow",
      "lime",
      "limegreen",
      "linen",
      "magenta",
      "maroon",
      "mediumaquamarine",
      "mediumblue",
      "mediumorchid",
      "mediumpurple",
      "mediumseagreen",
      "mediumslateblue",
      "mediumspringgreen",
      "mediumturquoise",
      "mediumvioletred",
      "midnightblue",
      "mintcream",
      "mistyrose",
      "moccasin",
      "navajowhite",
      "navy",
      "oldlace",
      "olive",
      "olivedrab",
      "orange",
      "orangered",
      "orchid",
      "palegoldenrod",
      "palegreen",
      "paleturquoise",
      "palevioletred",
      "papayawhip",
      "peachpuff",
      "peru",
      "pink",
      "plum",
      "powderblue",
      "purple",
      "rebeccapurple",
      "red",
      "rosybrown",
      "royalblue",
      "saddlebrown",
      "salmon",
      "sandybrown",
      "seagreen",
      "seashell",
      "sienna",
      "silver",
      "skyblue",
      "slateblue",
      "slategray",
      "slategrey",
      "snow",
      "springgreen",
      "steelblue",
      "tan",
      "teal",
      "thistle",
      "tomato",
      "turquoise",
      "violet",
      "wheat",
      "white",
      "whitesmoke",
      "yellow",
      "yellowgreen",
    ];
    let mut index = 0;
    for ch in munjayeol.chars() {
      if index == 5 {
        break;
      }
      if self.array.get(self.index + index) != Some(&Objects::Char(ch)) {
        return false;
      }
      index += 1;
    }
    let mut is_color: bool = false;
    let mut indexes = Vec::new();
    for color in colors {
      let mut is_this_color = true;
      for ch in color.chars() {
        if self.array.get(self.index + index) != Some(&Objects::Char(ch)) {
          is_this_color = false;
        } else {
          index += 1;
        }
      }
      if is_this_color {
        indexes.push(index);
        is_color = true;
      }
      index = 4;
    }
    for i in indexes.clone() {
      if i > index {
        index = i;
      }
    }
    if !is_color {
      loop {
        if let Some(Objects::Char(ch)) = self.array.get(self.index + index)
          && ch.is_ascii_hexdigit()
        {
          index += 1;
        } else {
          is_color = index == 10 || index == 7;
          break;
        }
      }
    }
    if !is_color {
      return false;
    }
    if let Some(Objects::Char(' ')) = self.array.get(self.index + index) {
      return is_color;
      //3축약, 풀헥스 지원
    }
    if let Some(Objects::Char(',')) = self.array.get(self.index + index)
      && let Some(Objects::Char('#')) = self.array.get(self.index + index + 1)
    {
      index += 2;
    } else {
      false;
    }
    indexes.clear();
    is_color = false;
    let rollbackindex = index;
    for color in colors {
      let mut is_this_color = true;
      for ch in color.chars() {
        if self.array.get(self.index + index) != Some(&Objects::Char(ch)) {
          is_this_color = false;
        } else {
          index += 1;
        }
      }
      if is_this_color {
        is_color = true;
        indexes.push(index);
      }
      index = rollbackindex;
    }
    for i in indexes {
      if i > index {
        index = i;
      }
    }
    if !is_color {
      loop {
        if let Some(Objects::Char(ch)) = self.array.get(self.index + index)
          && ch.is_ascii_hexdigit()
        {
          index += 1;
        } else {
          if index == rollbackindex + 3 || index == rollbackindex + 6 {
            break;
          } else {
            return false;
          }
        }
      }
    }
    if let Some(Objects::Char(' ')) = self.array.get(self.index + index) {
      return index == rollbackindex + 3 || index == rollbackindex + 6;
      //3축약, 풀헥스 지원
    }
    false
  }
}
///파싱 과정중에 쓰이는 것으로 신경은 안쓰셔도 됩니다.
///이런거가 함수가 나눠진다는게 너무 한십해서 그랬습니다
fn 에휴_진짜_왜그럼(sliceee: &[Objects]) -> String {
  let mut result = String::new();
  for obj in sliceee {
    if let Objects::Char(ch) = obj {
      result.push(ch.clone());
    } else {
      panic!();
    }
  }
  result
}
