use std::mem::discriminant;

//이게 파서냐
use crate::{parser_first::slices, renderobjs::{Bold, DelBar, DelTidal, Itelic, Lower, RenderObject, UnderLine, Upper}, structs::{Compiler, Expect, Objects}};

pub fn parse_third(compiler: &mut Compiler, close: Expect) -> RenderObject {
  let mut result = RenderObject::NopNopNop;
  prepare_result(&mut result, &close);
  let mut namumarkresult: Vec<Objects> = Vec::new();
  while namumarker(compiler, &close, &mut result, &mut namumarkresult) {}
  return result;
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
      *result = RenderObject::Itelic( Itelic {
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
      *result = RenderObject::Upper(Upper{
        content: Vec::new(),
      })
    }
    Expect::Lower => {
      *result = RenderObject::Lower( Lower{
        content: Vec::new(),
      })
    },
    _ => {
      panic!()
    }
  }
}
fn namumarker(compiler: &mut Compiler, close: &Expect, result: &mut RenderObject, namumarkresult:&mut Vec<Objects>) -> bool {
  let mut thisparsing:Option<RenderObject> = None;
  match compiler.current() {
    Some(Objects::Char(ch)) => {
      if !parsing_close(compiler, close, result, namumarkresult) {
        return false;
      }
      if compiler.peak("'''") {
        compiler.index += 3;
        compiler.expected.push((Expect::Bold, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::Bold));
      }else if compiler.peak("''") {
        compiler.index += 2;
        compiler.expected.push((Expect::Itelic, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::Itelic));
      }else if compiler.peak("~~") {
        compiler.index += 2;
        compiler.expected.push((Expect::DelTidal, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::DelTidal));
      }else if compiler.peak("--") {
        compiler.index += 2;
        compiler.expected.push((Expect::DelBar, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::DelBar));
      }else if compiler.peak("__") {
        compiler.index += 2;
        compiler.expected.push((Expect::UnderLine, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::UnderLine));
      }else if compiler.peak("^^") {
        compiler.index += 2;
        compiler.expected.push((Expect::Upper, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::Upper));
      }else if compiler.peak(",,") {
        compiler.index += 2;
        compiler.expected.push((Expect::Lower, compiler.index, false));
        thisparsing = Some(parse_third(compiler, Expect::Lower));
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
          },
          RenderObject::EarlyParse(tuple) => {
            compiler.expected.pop();
            if discriminant(close) == discriminant(&tuple.0) {
              match tuple.0 {
                Expect::Bold => {
                  if let RenderObject::Bold(bd) = result {
                    bd.content.extend(tuple.1);
                    return false;
                  }
                },
                Expect::Itelic => {
                  if let RenderObject::Itelic(it) = result {
                    it.content.extend(tuple.1);
                    return false;
                  }
                },
                Expect::DelTidal => {
                  if let RenderObject::DelTidal(dt) = result {
                    dt.content.extend(tuple.1);
                    return false;
                  }
                },
                Expect::DelBar => {
                  if let RenderObject::DelBar(db) = result {
                    db.content.extend(tuple.1);
                    return false;
                  }
                },
                Expect::UnderLine => {
                  if let RenderObject::UnderLine(ul) = result {
                    ul.content.extend(tuple.1);
                    return false;
                  }
                },
                Expect::Upper => {
                  if let RenderObject::Upper(up) = result {
                    up.content.extend(tuple.1);
                    return false;
                  }
                },
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
              *result =
              RenderObject::EarlyParse((tuple.0, a_whole_my_vec(close, namumarkresult)));
              return false;
            }
          },
          obj => {
            namumarkresult.push(Objects::RenderObject(obj));
            return true;
          }
        }
        return false;
      } else {
        return true;
      }
    },
    Some(Objects::RenderObject(rdobj)) => {
      compiler.index += 1;
      namumarkresult.push(Objects::RenderObject(rdobj));
      return true;
    }
    None => {
      if compiler.expected.is_empty() {
        compiler.array = namumarkresult.to_vec();
        return false;
      } else {
        *result = RenderObject::Nop(namumarkresult.to_vec());
        return false;
      }
    },
  }
}
fn parsing_close(compiler: &mut Compiler, close: &Expect, result:&mut RenderObject, namumarkresult:&mut Vec<Objects>) -> bool {
  if compiler.peak("'''") {
    compiler.index += 3;
    if *close == Expect::Bold {
      compiler.expected.pop();
      if let RenderObject::Bold(bd) = result {
        bd.content = namumarkresult.to_vec();
      } else {
        panic!("지름신불타네")
      }
      return false;
    } else if *close == Expect::Itelic {
      compiler.expected.pop();
      namumarkresult.push(Objects::Char('\''));
      *result = RenderObject::Itelic(Itelic { content: namumarkresult.to_vec() });
      return false;
    }else if let (true, _, _, exp) = compiler.contains_for_parsing_more(|x| x == &Expect::Bold || x == &Expect::Itelic) {
      if exp == Expect::Bold {
        compiler.expected.pop();
        *result = RenderObject::EarlyParse((
          Expect::Bold,
          a_whole_my_vec(&Expect::Bold, namumarkresult),
        ));
        return false;
      } else {
        compiler.expected.pop();
        namumarkresult.push(Objects::Char('\''));
        *result = RenderObject::EarlyParse((
          Expect::Itelic,
          a_whole_my_vec(&Expect::Itelic, namumarkresult),
        ));
        return false;
      }
    } else {
      compiler.index -= 3;
      return true;
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
      return false;
    } else if *close == Expect::Bold {
      compiler.expected.pop();
      namumarkresult.insert(0, Objects::Char('\''));
      *result = RenderObject::Itelic(Itelic {
        content:namumarkresult.to_vec()});
      return false;
    }else if let (true, _, _, exp) = compiler.contains_for_parsing_more(|x| x == &Expect::Itelic || x == &Expect::Bold) {
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
      return false;
    } else {
      compiler.index -= 2;
      return true;
    }
  }else if compiler.peak("~~") {
    compiler.index += 2;
    if *close == Expect::DelTidal {
      compiler.expected.pop();
      if let RenderObject::DelTidal(dt /*어... 어..?*/) = result {
        dt.content = namumarkresult.to_vec();
      } else {
        panic!("지름신불타네")
      }
      return false;
    }else if let (true, _, _) = compiler.contains_for_parsing(|x| x == &Expect::DelTidal) {
      compiler.expected.pop();
      *result = RenderObject::EarlyParse((
        Expect::DelTidal,
        a_whole_my_vec(&Expect::Bold, namumarkresult),
      ));
      return false;
    } else {
      compiler.index -= 2;
      return true;
    }
  }else if compiler.peak("--") {
    compiler.index += 2;
    if *close == Expect::DelBar {
      compiler.expected.pop();
      if let RenderObject::DelBar(db) = result {
        db.content = namumarkresult.to_vec();
      } else {
        panic!("지름신불타네")
      }
      return false;
    }else if let (true, _, _) = compiler.contains_for_parsing(|x| x == &Expect::DelBar) {
      compiler.expected.pop();
      *result = RenderObject::EarlyParse((
        Expect::DelBar,
        a_whole_my_vec(&Expect::DelBar, namumarkresult),
      ));
      return false;
    } else {
      compiler.index -= 2;
      return true;
    }
  }else if compiler.peak("__") {
    compiler.index += 2;
    if *close == Expect::UnderLine {
      compiler.expected.pop();
      if let RenderObject::UnderLine(ul) = result {
        ul.content = namumarkresult.to_vec();
      } else {
        panic!("지름신불타네")
      }
      return false;
    }else if let (true, _, _) = compiler.contains_for_parsing(|x| x == &Expect::UnderLine) {
      compiler.expected.pop();
      *result = RenderObject::EarlyParse((
        Expect::UnderLine,
        a_whole_my_vec(&Expect::UnderLine, namumarkresult),
      ));
      return false;
    } else {
      compiler.index -= 2;
      return true;
    }
  }else if compiler.peak(",,") {
    compiler.index += 2;
    if *close == Expect::Lower {
      compiler.expected.pop();
      if let RenderObject::Lower(lw) = result {
        lw.content = namumarkresult.to_vec();
      } else {
        panic!("지름신불타네")
      }
      return false;
    }else if let (true, _, _) = compiler.contains_for_parsing(|x| x == &Expect::Lower) {
      compiler.expected.pop();
      *result = RenderObject::EarlyParse((
        Expect::Lower,
        a_whole_my_vec(&Expect::Lower, namumarkresult),
      ));
      return false;
    } else {
      compiler.index -= 2;
      return true;
    }
  }else if compiler.peak("^^") {
    compiler.index += 3;
    if *close == Expect::Upper {
      compiler.expected.pop();
      if let RenderObject::Upper(up) = result {
        up.content = namumarkresult.to_vec();
      } else {
        panic!("지름신불타네")
      }
      return false;
    }else if let (true, _, _) = compiler.contains_for_parsing(|x| x == &Expect::Upper) {
      compiler.expected.pop();
      *result = RenderObject::EarlyParse((
        Expect::Upper,
        a_whole_my_vec(&Expect::Upper, namumarkresult),
      ));
      return false;
    } else {
      compiler.index -= 2;
      return true;
    }
  } else {
    return true;
  }
}
fn a_whole_my_vec(close:&Expect, namumarkresult:&mut Vec<Objects>) -> Vec<Objects> {
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
    _ => {
      panic!("issue github: {:?}", close)
    }
  }
}
