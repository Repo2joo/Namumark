use std::vec;

use crate::{parse_third::parse_third, parser_first::parse_first, renderobjs::RenderObject};
#[derive(Debug)]
///Compiler struct. Can be obtained using [Compiler::from]
pub struct Compiler {
  pub(crate) index: usize,
  ///The array that result goes
  pub array: Vec<Objects>,
  pub(crate) expected: Vec<(Expect, usize, bool)>,
  ///Fixed comments. Doesn't parsed
  pub fixed_comments: Vec<String>,
  ///Redirect
  pub redirect: Option<String>,
  pub(crate) rollbacks: Option<usize>,
  pub(crate) custom_macro: Vec<CustomMacro>,
}
#[derive(Debug, Clone)]
pub struct CustomMacro {
  ///the name of macro
  name: String,
  ///weather the macro have argument
  ///# example
  ///\[각주\] has no argument.<br />
  ///\[include(argument)\] has argument
  arg: bool,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Objects {
  ///Char
  Char(char),
  ///see [RenderObject]
  RenderObject(RenderObject),
}
#[derive(Debug, PartialEq, Clone)]
///No need to see.
pub enum Expect {
  None,
  Link,
  SyntaxTriple,
  TripleWithNamuMark,
  JustTriple,
  NamuMacro,
  List(usize),
  Quote(usize),
  Heading(usize),
  Color,
  Plus,
  Minus,
  Reference,
  Bold,
  Itelic,
  DelTidal,
  DelBar,
  UnderLine,
  Upper,
  Lower,
  Table,
}
#[derive(Debug, PartialEq, Clone)]
/// The type variant of [crate::renderobjs::List]
pub enum ListType {
  Hangul,
  AlphaSmall,
  AlphaBig,
  RomanBig,
  RomanSmall,
  ///the star(*) list
  List,
  Arabia,
}
#[derive(Debug, PartialEq, Clone)]
///the type variant of [crate::renderobjs::NamumarkMacro]
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
  Date,
  TableOfContents,
  Reference,
  Br,
  Clearfix,
  ///the custom macro
  Custom,
}
impl Compiler {
  pub(crate) fn get_before_earlyparse(&self, vec: Vec<Objects>) -> RenderObject {
    RenderObject::AddBefore(vec)
  }
  pub(crate) fn peak_macro(&mut self) -> Option<String> {
    for i in self.custom_macro.to_vec() {
      if i.arg {
        continue;
      };
      if self.peak(format!("[{}]", i.name).as_str()) {
        self.index += i.name.len() + 2;
        return Some(i.name);
      }
    }
    None
  }
  pub(crate) fn peak_macro_arg(&self) -> bool {
    for i in self.custom_macro.clone() {
      if !i.arg {
        continue;
      };
      if self.peak(format!("[{}(", i.name).as_str()) {
        return true;
      }
    }
    false
  }
  ///return Compiler from string
  pub fn from(string: String) -> Compiler {
    let mut compiler = Compiler {
      index: 0,
      array: Vec::new(),
      expected: Vec::new(),
      fixed_comments: vec![String::new()],
      redirect: None,
      rollbacks: None,
      custom_macro: Vec::new(),
    };
    for char in string.chars() {
      compiler.array.push(Objects::Char(char));
    }
    compiler
  }
  ///parse the string
  pub fn parse(&mut self) {
    parse_first(self, Expect::None);
    self.index = 0;
    self.expected.clear();
    parse_third(self, Expect::None);
    self.fixed_comments.pop();
  }
  pub fn add_custom_macros(&mut self, macros: Vec<CustomMacro>) {
    self.custom_macro.extend(macros);
  }
  pub(crate) fn get(&self, idx: usize) -> Option<&Objects> {
    self.array.get(idx)
  }
  pub(crate) fn current(&self) -> Option<Objects> {
    self.array.get(self.index).cloned()
  }
  pub(crate) fn peak(&self, str: &str) -> bool {
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
    true
  }
  pub(crate) fn peak_line(&mut self, str: &str) -> bool {
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
    true
  }
  pub(crate) fn contains_for_parsing(
    &self,
    closure: impl Fn(&Expect) -> bool,
  ) -> (bool, bool, usize) {
    let a = self.contains_for_parsing_more(closure);
    (a.0, a.1, a.2)
  }
  pub(crate) fn contains_for_parsing_more(
    &self,
    closure: impl Fn(&Expect) -> bool,
  ) -> (bool, bool, usize, Expect) {
    let mut has_to_rollback = false;
    let index = 0..self.expected.len();
    for i in index {
      let item = self.expected.get(i).unwrap();
      if closure(&item.0) {
        return (true, has_to_rollback, i + 1, item.0.clone());
      } else if item.2 {
        has_to_rollback = true;
      }
    }
    (false, has_to_rollback, 0, Expect::None)
  }
  pub(crate) fn peak_repeat_line(&mut self, ch: char, end: Option<&str>) -> (bool, usize) {
    if self.index == 0 || self.get(self.index - 1) == Some(&Objects::Char('\n')) {
      let mut idx = 0;
      loop {
        if self.get(self.index + idx) == Some(&Objects::Char(ch)) {
          idx += 1;
        } else if end.is_none() && idx != 0 {
          return (true, idx);
        } else if end.is_some()
          && end.unwrap().len() + self.index + idx < self.array.len()
          && 에휴_진짜_왜그럼(
            &self.array[self.index + idx..end.unwrap().len() + self.index + idx],
          ) == end.unwrap()
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
  pub(crate) fn is_color(&self) -> bool {
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
        } else if index == rollbackindex + 3 || index == rollbackindex + 6 {
          break;
        } else {
          return false;
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
fn 에휴_진짜_왜그럼(sliceee: &[Objects]) -> String {
  let mut result = String::new();
  for obj in sliceee {
    if let Objects::Char(ch) = obj {
      result.push(*ch);
    } else {
      panic!();
    }
  }
  result
}
