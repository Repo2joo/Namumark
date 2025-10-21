use std::mem::discriminant;

use regex::Regex;

//이게 파서냐
use crate::{
  parser_first::slices,
  renderobjs::{
    Altitude, Bold, CellAttribute, DelBar, DelTidal, Direction, Itelic, Lower, RenderObject, Table,
    TableCell, TableRow, UnderLine, Upper,
  },
  structs::{Compiler, Expect, Objects},
};

pub fn parse_third(compiler: &mut Compiler, close: Expect) -> RenderObject {
  let mut result = RenderObject::NopNopNop;
  prepare_result(&mut result, &close);
  let mut namumarkresult: Vec<Objects> = Vec::new();
  while namumarker(compiler, &close, &mut result, &mut namumarkresult) {}
  result
}
fn prepare_result(result: &mut RenderObject, close: &Expect) {
  match close {
    Expect::None => *result = RenderObject::NopNopNop,
    Expect::Bold => {
      *result = RenderObject::Bold(Bold {
        content: Vec::new(),
      })
    }
    Expect::Itelic => {
      *result = RenderObject::Itelic(Itelic {
        content: Vec::new(),
      })
    }
    Expect::DelTidal => {
      *result = RenderObject::DelTidal(DelTidal {
        content: Vec::new(),
      })
    }
    Expect::DelBar => {
      *result = RenderObject::DelBar(DelBar {
        content: Vec::new(),
      })
    }
    Expect::UnderLine => {
      *result = RenderObject::UnderLine(UnderLine {
        content: Vec::new(),
      })
    }
    Expect::Upper => {
      *result = RenderObject::Upper(Upper {
        content: Vec::new(),
      })
    }
    Expect::Lower => {
      *result = RenderObject::Lower(Lower {
        content: Vec::new(),
      })
    }
    Expect::Table => *result = RenderObject::TableRow(Vec::new()),
    _ => {
      panic!()
    }
  }
}
fn namumarker(
  compiler: &mut Compiler,
  close: &Expect,
  result: &mut RenderObject,
  namumarkresult: &mut Vec<Objects>,
) -> bool {
  let mut thisparsing: Option<RenderObject> = None;
  match compiler.current() {
    Some(Objects::Char(ch)) => {
      if !parsing_close(compiler, close, result, namumarkresult) {
        return false;
      }
      if compiler.peak("'''") {
        compiler.index += 3;
        compiler
          .expected
          .push((Expect::Bold, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::Bold));
      } else if compiler.peak("''") {
        compiler.index += 2;
        compiler
          .expected
          .push((Expect::Itelic, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::Itelic));
      } else if compiler.peak("~~") {
        compiler.index += 2;
        compiler
          .expected
          .push((Expect::DelTidal, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::DelTidal));
      } else if compiler.peak("--") {
        compiler.index += 2;
        compiler
          .expected
          .push((Expect::DelBar, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::DelBar));
      } else if compiler.peak("__") {
        compiler.index += 2;
        compiler
          .expected
          .push((Expect::UnderLine, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::UnderLine));
      } else if compiler.peak("^^") {
        compiler.index += 2;
        compiler
          .expected
          .push((Expect::Upper, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::Upper));
      } else if compiler.peak(",,") {
        compiler.index += 2;
        compiler
          .expected
          .push((Expect::Lower, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::Lower));
      } else if compiler.peak_line("||") && close != &Expect::Table {
        compiler.index += 2;
        compiler
          .expected
          .push((Expect::Table, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::Table));
        if let Some(RenderObject::TableRow(ref tr)) = thisparsing {
          if let Some(Objects::RenderObject(RenderObject::Table(_))) = namumarkresult.last_mut() {
          } else {
            namumarkresult.push(Objects::RenderObject(RenderObject::Table(Table {
              table_row: Vec::new(),
            })));
          }
          if let Some(Objects::RenderObject(RenderObject::Table(tb))) = namumarkresult.last_mut() {
            let table_cells = get_table_cells(tr);
            tb.table_row.push(table_cells);
          } else {
            panic!("??????????");
          }
        }
      } else {
        compiler.index += 1;
        namumarkresult.push(Objects::Char(ch));
      }
      if let Some(rdobj) = thisparsing {
        match rdobj {
          RenderObject::Nop(items) => {
            compiler.expected.pop();
            namumarkresult.extend(items);
            *result = RenderObject::Nop(a_whole_my_vec(close, namumarkresult));
            return false;
          }
          RenderObject::EarlyParse(tuple) => {
            compiler.expected.pop();
            if discriminant(close) == discriminant(&tuple.0) {
              match tuple.0 {
                Expect::Bold => {
                  if let RenderObject::Bold(bd) = result {
                    bd.content.extend(tuple.1);
                    return false;
                  }
                }
                Expect::Itelic => {
                  if let RenderObject::Itelic(it) = result {
                    it.content.extend(tuple.1);
                    return false;
                  }
                }
                Expect::DelTidal => {
                  if let RenderObject::DelTidal(dt) = result {
                    dt.content.extend(tuple.1);
                    return false;
                  }
                }
                Expect::DelBar => {
                  if let RenderObject::DelBar(db) = result {
                    db.content.extend(tuple.1);
                    return false;
                  }
                }
                Expect::UnderLine => {
                  if let RenderObject::UnderLine(ul) = result {
                    ul.content.extend(tuple.1);
                    return false;
                  }
                }
                Expect::Upper => {
                  if let RenderObject::Upper(up) = result {
                    up.content.extend(tuple.1);
                    return false;
                  }
                }
                Expect::Lower => {
                  if let RenderObject::Lower(lw) = result {
                    lw.content.extend(tuple.1);
                    return false;
                  }
                }
                _ => panic!(), //여기서 처리하는 건 없음
              }
            } else {
              namumarkresult.extend(tuple.1);
              *result = RenderObject::EarlyParse((tuple.0, a_whole_my_vec(close, namumarkresult)));
              return false;
            }
          }
          obj => match obj {
            RenderObject::TableRow(_) => {
              return true;
            }
            _ => {
              namumarkresult.push(Objects::RenderObject(obj));
              return true;
            }
          },
        }
        false
      } else {
        true
      }
    }
    Some(Objects::RenderObject(rdobj)) => {
      compiler.index += 1;
      namumarkresult.push(Objects::RenderObject(rdobj));
      true
    }
    None => {
      if compiler.expected.is_empty() {
        compiler.array = namumarkresult.to_vec();
        false
      } else {
        *result = RenderObject::Nop(namumarkresult.to_vec());
        false
      }
    }
  }
}
fn parsing_close(
  compiler: &mut Compiler,
  close: &Expect,
  result: &mut RenderObject,
  namumarkresult: &mut Vec<Objects>,
) -> bool {
  if compiler.peak("||")
    && (compiler.index + 2 == compiler.array.len()
      || (compiler
        .get(compiler.index + 2)
        .is_some_and(|x| x.eq(&Objects::Char('\n')))))
  {
    compiler.index += 2;
    if compiler
      .get(compiler.index)
      .is_some_and(|x| x.eq(&Objects::Char('\n')))
    {
      compiler.index += 1;
    }
    if *close == Expect::Table {
      compiler.expected.pop();
      if let RenderObject::TableRow(tr) = result {
        *tr = namumarkresult.to_vec();
      } else {
        panic!("지름신불타네")
      }
      false
    } else if let (true, _, _) = compiler.contains_for_parsing(|x| x == &Expect::Table) {
      compiler.expected.pop();
      *result = RenderObject::EarlyParse((
        Expect::Table,
        a_whole_my_vec(&Expect::Table, namumarkresult),
      ));
      false
    } else {
      compiler.array.push(Objects::Char('|'));
      compiler.array.push(Objects::Char('|'));
      if compiler
        .get(compiler.index)
        .is_some_and(|x| x.eq(&Objects::Char('\n')))
      {
        compiler.array.push(Objects::Char('\n'));
      }
      true
    }
  } else if compiler.peak("'''") {
    compiler.index += 3;
    if *close == Expect::Bold {
      compiler.expected.pop();
      if let RenderObject::Bold(bd) = result {
        bd.content = namumarkresult.to_vec();
      } else {
        panic!("지름신불타네")
      }
      false
    } else if *close == Expect::Itelic {
      compiler.expected.pop();
      namumarkresult.push(Objects::Char('\''));
      *result = RenderObject::Itelic(Itelic {
        content: namumarkresult.to_vec(),
      });
      false
    } else if let (true, _, _, exp) =
      compiler.contains_for_parsing_more(|x| x == &Expect::Bold || x == &Expect::Itelic)
    {
      if exp == Expect::Bold {
        compiler.expected.pop();
        *result =
          RenderObject::EarlyParse((Expect::Bold, a_whole_my_vec(&Expect::Bold, namumarkresult)));
        false
      } else {
        compiler.expected.pop();
        namumarkresult.push(Objects::Char('\''));
        *result = RenderObject::EarlyParse((
          Expect::Itelic,
          a_whole_my_vec(&Expect::Itelic, namumarkresult),
        ));
        false
      }
    } else {
      compiler.index -= 3;
      true
    }
  } else if compiler.peak("''") {
    compiler.index += 2;
    if *close == Expect::Itelic {
      compiler.expected.pop();
      if let RenderObject::Itelic(it) = result {
        it.content = namumarkresult.to_vec();
      } else {
        panic!("지름신불타네")
      }
      false
    } else if *close == Expect::Bold {
      compiler.expected.pop();
      namumarkresult.insert(0, Objects::Char('\''));
      *result = RenderObject::Itelic(Itelic {
        content: namumarkresult.to_vec(),
      });
      false
    } else if let (true, _, _, exp) =
      compiler.contains_for_parsing_more(|x| x == &Expect::Itelic || x == &Expect::Bold)
    {
      if exp == Expect::Bold {
        compiler.expected.pop();
        namumarkresult.insert(0, Objects::Char('\''));
        *result = RenderObject::EarlyParse((
          Expect::Itelic,
          a_whole_my_vec(&Expect::Itelic, namumarkresult),
        ));
      } else {
        compiler.expected.pop();
        *result = RenderObject::EarlyParse((
          Expect::Itelic,
          a_whole_my_vec(&Expect::Itelic, namumarkresult),
        ));
      }
      false
    } else {
      compiler.index -= 2;
      true
    }
  } else if compiler.peak("~~") {
    compiler.index += 2;
    if *close == Expect::DelTidal {
      compiler.expected.pop();
      if let RenderObject::DelTidal(dt /*어... 어..?*/) = result {
        dt.content = namumarkresult.to_vec();
      } else {
        panic!("지름신불타네")
      }
      false
    } else if let (true, _, _) = compiler.contains_for_parsing(|x| x == &Expect::DelTidal) {
      compiler.expected.pop();
      *result = RenderObject::EarlyParse((
        Expect::DelTidal,
        a_whole_my_vec(&Expect::DelTidal, namumarkresult),
      ));
      false
    } else {
      compiler.index -= 2;
      true
    }
  } else if compiler.peak("--") {
    compiler.index += 2;
    if *close == Expect::DelBar {
      compiler.expected.pop();
      if let RenderObject::DelBar(db) = result {
        db.content = namumarkresult.to_vec();
      } else {
        panic!("지름신불타네")
      }
      false
    } else if let (true, _, _) = compiler.contains_for_parsing(|x| x == &Expect::DelBar) {
      compiler.expected.pop();
      *result = RenderObject::EarlyParse((
        Expect::DelBar,
        a_whole_my_vec(&Expect::DelBar, namumarkresult),
      ));
      false
    } else {
      compiler.index -= 2;
      true
    }
  } else if compiler.peak("__") {
    compiler.index += 2;
    if *close == Expect::UnderLine {
      compiler.expected.pop();
      if let RenderObject::UnderLine(ul) = result {
        ul.content = namumarkresult.to_vec();
      } else {
        panic!("지름신불타네")
      }
      false
    } else if let (true, _, _) = compiler.contains_for_parsing(|x| x == &Expect::UnderLine) {
      compiler.expected.pop();
      *result = RenderObject::EarlyParse((
        Expect::UnderLine,
        a_whole_my_vec(&Expect::UnderLine, namumarkresult),
      ));
      false
    } else {
      compiler.index -= 2;
      true
    }
  } else if compiler.peak(",,") {
    compiler.index += 2;
    if *close == Expect::Lower {
      compiler.expected.pop();
      if let RenderObject::Lower(lw) = result {
        lw.content = namumarkresult.to_vec();
      } else {
        panic!("지름신불타네")
      }
      false
    } else if let (true, _, _) = compiler.contains_for_parsing(|x| x == &Expect::Lower) {
      compiler.expected.pop();
      *result = RenderObject::EarlyParse((
        Expect::Lower,
        a_whole_my_vec(&Expect::Lower, namumarkresult),
      ));
      false
    } else {
      compiler.index -= 2;
      true
    }
  } else if compiler.peak("^^") {
    compiler.index += 3;
    if *close == Expect::Upper {
      compiler.expected.pop();
      if let RenderObject::Upper(up) = result {
        up.content = namumarkresult.to_vec();
      } else {
        panic!("지름신불타네")
      }
      false
    } else if let (true, _, _) = compiler.contains_for_parsing(|x| x == &Expect::Upper) {
      compiler.expected.pop();
      *result = RenderObject::EarlyParse((
        Expect::Upper,
        a_whole_my_vec(&Expect::Upper, namumarkresult),
      ));
      false
    } else {
      compiler.index -= 2;
      true
    }
  } else {
    true
  }
}
fn a_whole_my_vec(close: &Expect, namumarkresult: &mut Vec<Objects>) -> Vec<Objects> {
  match close {
    Expect::Bold => {
      let mut rst = slices("'''".to_string());
      rst.extend_from_slice(namumarkresult);
      rst
    }
    Expect::Itelic => {
      let mut rst = slices("''".to_string());
      rst.extend_from_slice(namumarkresult);
      rst
    }
    Expect::DelTidal => {
      let mut rst = slices("~~".to_string());
      rst.extend_from_slice(namumarkresult);
      rst
    }
    Expect::DelBar => {
      let mut rst = slices("--".to_string());
      rst.extend_from_slice(namumarkresult);
      rst
    }
    Expect::UnderLine => {
      let mut rst = slices("__".to_string());
      rst.extend_from_slice(namumarkresult);
      rst
    }
    Expect::Upper => {
      let mut rst = slices("^^".to_string());
      rst.extend_from_slice(namumarkresult);
      rst
    }
    Expect::Lower => {
      let mut rst = slices(",,".to_string());
      rst.extend_from_slice(namumarkresult);
      rst
    }
    Expect::Table => {
      let mut rst = slices("\n||".to_string());
      rst.extend_from_slice(namumarkresult);
      rst
    }
    _ => {
      panic!("issue github: {:?}", close)
    }
  }
}
//마지막 할거:
fn get_table_cells(row: &Vec<Objects>) -> TableRow {
  let mut table_cell: Vec<TableCell> = Vec::new();
  let mut i: usize = 0;
  let mut temp: Vec<Objects> = Vec::new();
  loop {
    if i == row.len() {
      table_cell.push(TableCell {
        attribute: CellAttribute::default(),
        content: temp.to_vec(),
        allign: None,
        height_align: None,
      });
      break;
    }
    if row.get(i) == Some(&Objects::Char('|')) && row.get(i + 1) == Some(&Objects::Char('|')) {
      i += 1;
      table_cell.push(TableCell {
        attribute: CellAttribute::default(),
        content: temp.to_vec(),
        allign: None,
        height_align: None,
      });
      temp.clear();
    } else {
      temp.push(row.get(i).unwrap().clone());
    }
    i += 1;
  }
  fn peak(array: &Vec<Objects>, str: &str, index: usize) -> bool {
    for (idx, ch) in str.chars().enumerate() {
      if let Some(Objects::Char(cha)) = array.get(idx + index) {
        if ch.to_lowercase().to_string() != *cha.to_lowercase().to_string() {
          return false;
        }
      } else {
        return false;
      }
    }
    true
  }
  fn detect_number(array: &Vec<Objects>, index: usize) -> Option<String> {
    let mut the_number = String::new();
    let mut idx = 0;
    loop {
      if let Some(Objects::Char(cha)) = array.get(idx + index) {
        if cha.is_digit(10) {
          the_number.push(*cha);
        } else if peak(array, "px>", index + idx) {
          the_number.push_str("px");
          return Some(the_number);
        } else if peak(array, "%>", index + idx) {
          the_number.push_str("%");
          return Some(the_number);
        } else if peak(array, ">", index + idx) {
          return Some(the_number);
        } else {
          println!("{}", cha);
          return None;
        }
      } else {
        return None;
      }
      idx += 1;
    }
  }
  fn detact_color(array: &Vec<Objects>, index: usize) -> Option<(Option<String>, Option<String>)> {
    let mut string = String::new();
    let mut i = 0;
    loop {
      if let Some(Objects::Char(ch)) = array.get(i + index) {
        if ch == &'>' {
          break;
        } else {
          string.push(*ch);
        }
      } else {
        return None;
      }
      i += 1;
    }
    //바이브 코딩은 신이야
    let color_pattern = r"#(?:[0-9a-f]{3}|[0-9a-f]{6})|aliceblue|antiquewhite|aqua|aquamarine|azure|beige|bisque|black|blanchedalmond|blue|blueviolet|brown|burlywood|cadetblue|chartreuse|chocolate|coral|cornflowerblue|cornsilk|crimson|cyan|darkblue|darkcyan|darkgoldenrod|darkgray|darkgrey|darkgreen|darkkhaki|darkmagenta|darkolivegreen|darkorange|darkorchid|darkred|darksalmon|darkseagreen|darkslateblue|darkslategray|darkslategrey|darkturquoise|darkviolet|deeppink|deepskyblue|dimgray|dimgrey|dodgerblue|firebrick|floralwhite|forestgreen|fuchsia|gainsboro|ghostwhite|gold|goldenrod|gray|grey|green|greenyellow|honeydew|hotpink|indianred|indigo|ivory|khaki|lavender|lavenderblush|lawngreen|lemonchiffon|lightblue|lightcoral|lightcyan|lightgoldenrodyellow|lightgray|lightgrey|lightgreen|lightpink|lightsalmon|lightseagreen|lightskyblue|lightslategray|lightslategrey|lightsteelblue|lightyellow|lime|limegreen|linen|magenta|maroon|mediumaquamarine|mediumblue|mediumorchid|mediumpurple|mediumseagreen|mediumslateblue|mediumspringgreen|mediumturquoise|mediumvioletred|midnightblue|mintcream|mistyrose|moccasin|navajowhite|navy|oldlace|olive|olivedrab|orange|orangered|orchid|palegoldenrod|palegreen|paleturquoise|palevioletred|papayawhip|peachpuff|peru|pink|plum|powderblue|purple|rebeccapurple|red|rosybrown|royalblue|saddlebrown|salmon|sandybrown|seagreen|seashell|sienna|silver|skyblue|slateblue|slategray|slategrey|snow|springgreen|steelblue|tan|teal|thistle|tomato|turquoise|violet|wheat|white|whitesmoke|yellow|yellowgreen";

    let full_pattern = format!(r"(?i)^({cp})(?:,({cp}))?$", cp = color_pattern);

    let re = Regex::new(&full_pattern).unwrap();

    if let Some(captures) = re.captures(&string) {
      let color1 = captures
        .get(1)
        .map(|m| Some(m.as_str().to_string()))
        .unwrap_or(None);
      let color2 = captures
        .get(2)
        .map(|m| Some(m.as_str().to_string()))
        .unwrap_or(None);
      Some((color1, color2))
    } else {
      None
    }
  }
  for i in &mut table_cell {
    let mut idx = 0;
    let content = &mut i.content;
    let mut rollback = 0;
    if content.first() == Some(&Objects::Char('<')) {
      loop {
        if peak(&content, "<:>", idx) {
          idx += 3;
          rollback += 3;
          i.allign = Some(Direction::Center)
        } else if peak(&content, "<(>", idx) {
          idx += 3;
          rollback += 3;
          i.allign = Some(Direction::Left)
        } else if peak(&content, "<)>", idx) {
          idx += 3;
          rollback += 3;
          i.allign = Some(Direction::Right)
        } else if peak(&content, "<nopad>", idx) {
          idx += 7;
          rollback += 7;
          i.attribute.nopad = true;
        } else if peak(&content, "<keepall>", idx) {
          idx += 9;
          rollback += 9;
          i.attribute.keepall = true;
        } else if peak(&content, "<colkeepall>", idx) {
          idx += 12;
          rollback += 12;
          i.attribute.colkeepall = true;
        } else if peak(&content, "<rowkeepall>", idx) {
          idx += 12;
          rollback += 12;
          i.attribute.rowkeepall = true;
        } else if peak(&content, "<-", idx) {
          idx += 2;
          if let Some(string) = detect_number(content, idx) {
            idx += string.len();
            rollback += 3 + string.len();
            i.attribute.rowspan = Some(string);
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        } else if peak(&content, "<v|", idx) {
          idx += 3;
          if let Some(string) = detect_number(content, idx) {
            idx += string.len();
            rollback += 4 + string.len();
            i.attribute.rowspan = Some(string);
            i.height_align = Some(Altitude::Low);
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        } else if peak(&content, "<|", idx) {
          idx += 3;
          if let Some(string) = detect_number(content, idx) {
            idx += string.len();
            rollback += 4 + string.len();
            i.attribute.rowspan = Some(string);
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        } else if peak(&content, "<^|", idx) {
          idx += 3;
          if let Some(string) = detect_number(content, idx) {
            idx += string.len();
            rollback += 4 + string.len();
            i.attribute.rowspan = Some(string);
            i.height_align = Some(Altitude::Low)
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        } else if peak(&content, "<width=", idx) {
          idx += 7;
          if let Some(string) = detect_number(content, idx) {
            idx += string.len();
            rollback += 8 + string.len();
            i.attribute.width = Some(string);
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        } else if peak(&content, "<height=", idx) {
          idx += 8;
          if let Some(string) = detect_number(content, idx) {
            idx += string.len();
            rollback += 9 + string.len();
            i.attribute.height = Some(string);
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        } else if peak(&content, "<bgcolor=", idx) {
          idx += 9;
          if let Some((Some(string), sec_color)) = detact_color(content, idx) {
            idx += string.len();
            if sec_color.is_some() {
              let sec_color = sec_color.clone().unwrap();
              idx += sec_color.len();
              idx += 1;
              rollback += sec_color.len();
              rollback += 1
            }
            rollback += 10 + string.len();
            i.attribute.bgcolor = Some((Some(string), sec_color));
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        }else if peak(&content, "<tablebordercolor=", idx) {
          idx += 18;
          if let Some((Some(string), sec_color)) = detact_color(content, idx) {
            idx += string.len();
            if sec_color.is_some() {
              let sec_color = sec_color.clone().unwrap();
              idx += sec_color.len();
              idx += 1;
              rollback += sec_color.len();
              rollback += 1
            }
            rollback += 19 + string.len();
            i.attribute.table_bordercolor = Some((Some(string), sec_color));
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        }else if peak(&content, "<table bordercolor=", idx) {
          idx += 19;
          if let Some((Some(string), sec_color)) = detact_color(content, idx) {
            idx += string.len();
            if sec_color.is_some() {
              let sec_color = sec_color.clone().unwrap();
              idx += sec_color.len();
              idx += 1;
              rollback += sec_color.len();
              rollback += 1
            }
            rollback += 20 + string.len();
            i.attribute.bordercolor = Some((Some(string), sec_color));
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        }else if peak(&content, "<color=", idx) {
          idx += 7;
          if let Some((Some(string), sec_color)) = detact_color(content, idx) {
            idx += string.len();
            if sec_color.is_some() {
              let sec_color = sec_color.clone().unwrap();
              idx += sec_color.len();
              idx += 1;
              rollback += sec_color.len();
              rollback += 1
            }
            rollback += 8 + string.len();
            i.attribute.color = Some((Some(string), sec_color));
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        }else if peak(&content, "<tablecolor=", idx) {
          idx += 12;
          if let Some((Some(string), sec_color)) = detact_color(content, idx) {
            idx += string.len();
            if sec_color.is_some() {
              let sec_color = sec_color.clone().unwrap();
              idx += sec_color.len();
              idx += 1;
              rollback += sec_color.len();
              rollback += 1
            }
            rollback += 13 + string.len();
            i.attribute.table_color = Some((Some(string), sec_color));
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        }else if peak(&content, "<table color=", idx) {
          idx += 13;
          if let Some((Some(string), sec_color)) = detact_color(content, idx) {
            idx += string.len();
            if sec_color.is_some() {
              let sec_color = sec_color.clone().unwrap();
              idx += sec_color.len();
              idx += 1;
              rollback += sec_color.len();
              rollback += 1
            }
            rollback += 14 + string.len();
            i.attribute.table_color = Some((Some(string), sec_color));
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        }else if peak(&content, "<colbgcolor=", idx) {
          idx += 12;
          if let Some((Some(string), sec_color)) = detact_color(content, idx) {
            idx += string.len();
            if sec_color.is_some() {
              let sec_color = sec_color.clone().unwrap();
              idx += sec_color.len();
              idx += 1;
              rollback += sec_color.len();
              rollback += 1
            }
            rollback += 13 + string.len();
            i.attribute.col_bgcolor = Some((Some(string), sec_color));
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        }else if peak(&content, "<rowbgcolor=", idx) {
          idx += 12;
          if let Some((Some(string), sec_color)) = detact_color(content, idx) {
            idx += string.len();
            if sec_color.is_some() {
              let sec_color = sec_color.clone().unwrap();
              idx += sec_color.len();
              idx += 1;
              rollback += sec_color.len();
              rollback += 1
            }
            rollback += 13 + string.len();
            i.attribute.row_bgcolor = Some((Some(string), sec_color));
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        }else if peak(&content, "<colcolor=", idx) {
          idx += 10;
          if let Some((Some(string), sec_color)) = detact_color(content, idx) {
            idx += string.len();
            if sec_color.is_some() {
              let sec_color = sec_color.clone().unwrap();
              idx += sec_color.len();
              idx += 1;
              rollback += sec_color.len();
              rollback += 1
            }
            rollback += 11 + string.len();
            i.attribute.col_color = Some((Some(string), sec_color));
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        }else if peak(&content, "<rowcolor=", idx) {
          idx +=10;
          if let Some((Some(string), sec_color)) = detact_color(content, idx) {
            idx += string.len();
            if sec_color.is_some() {
              let sec_color = sec_color.clone().unwrap();
              idx += sec_color.len();
              idx += 1;
              rollback += sec_color.len();
              rollback += 1
            }
            rollback += 11 + string.len();
            i.attribute.row_color = Some((Some(string), sec_color));
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        }else if peak(&content, "<tablebgcolor=", idx) {
          idx += 14;
          if let Some((Some(string), sec_color)) = detact_color(content, idx) {
            idx += string.len();
            if sec_color.is_some() {
              let sec_color = sec_color.clone().unwrap();
              idx += sec_color.len();
              idx += 1;
              rollback += sec_color.len();
              rollback += 1
            }
            rollback += 15 + string.len();
            i.attribute.table_bgcolor = Some((Some(string), sec_color));
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        }else if peak(&content, "<table bgcolor=", idx) {
          idx += 15;
          if let Some((Some(string), sec_color)) = detact_color(content, idx) {
            idx += string.len();
            if sec_color.is_some() {
              let sec_color = sec_color.clone().unwrap();
              idx += sec_color.len();
              idx += 1;
              rollback += sec_color.len();
              rollback += 1
            }
            rollback += 16 + string.len();
            i.attribute.table_bgcolor = Some((Some(string), sec_color));
          } else {
            *content = content[rollback..content.len()].to_vec();
          }
        } else {
          *content = content[rollback..content.len()].to_vec();
          break;
        }
      }
    }
  }
  TableRow {
    table_cell: table_cell,
  }
}
