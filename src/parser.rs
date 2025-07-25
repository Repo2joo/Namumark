//자 이건 내 마지막 경고오.
//대략적인 파서의 알고리즘을 이해하고 오쇼.
//그리고 여려움이 있으면 연락하쇼
use core::panic;
use std::vec;

use crate::{
  renderobjs::{
    Languages, Link, LinkType, List, ListLine, NamuTriple, NamumarkMacro, RenderObject, Syntax,
  },
  structs::{Compiler, Expect, ListType, NamuMacroType, Objects},
};

pub fn parse_first(compiler: &mut Compiler, close: Expect) -> RenderObject {
  let mut namumarkresult: Vec<Objects> = Vec::new();
  let mut result: RenderObject = RenderObject::NopNopNop;
  let mut close = close;
  prepare_result(&close, &mut result);
  while namumarker(compiler, &mut close, &mut namumarkresult, &mut result) {
    if compiler.lastrollbackindex.len() == 61 {
      panic!("문법 깊이 제한에 도달했습니다.")
    }
  }
  result
}
fn prepare_result(close: &Expect, mut result: &mut RenderObject) {
  match close {
    Expect::None => *result = RenderObject::NopNopNop,
    Expect::Link => {
      *result = RenderObject::Link(Link {
        to: String::new(),
        show: Vec::new(),
        link_type: LinkType::Hyper,
      })
    }
    Expect::Link2 => panic!(),
    Expect::SyntaxTriple => {
      *result = RenderObject::Syntax(Syntax {
        language: Languages::NotSupported,
        content: String::new(),
      })
    }
    Expect::TripleWithNamuMark => {
      *result = RenderObject::NamuTriple(NamuTriple {
        triplename: String::new(),
        attr: Some(String::new()),
        content: Some(Vec::new()),
      })
    }
    Expect::TripleWithNamuMark2 => panic!(),
    Expect::JustTriple => *result = RenderObject::Literal(String::new()),
    Expect::TripleWithNamuMark3 => panic!(),
    Expect::NamuMacro(namu_macro_type) => {
      *result = RenderObject::NamumarkMacro(NamumarkMacro {
        macroname: namu_macro_type.to_string(),
        macroarg: Some(String::new()),
      });
    }
    Expect::List(lvl) => {
      *result = RenderObject::ListLine(ListLine {
        lvl: lvl.clone(),
        content: Vec::new(),
      })
    }
  }
}
fn namumarker(
  compiler: &mut Compiler,
  close: &mut Expect,
  namumarkresult: &mut Vec<Objects>,
  result: &mut RenderObject,
) -> bool {
  if let Some(Objects::Char(ch)) = compiler.current() {
    let ch = ch.to_owned();
    let whattodo = parsing_close(compiler, close, result, namumarkresult);
    if let Some(bool) = whattodo {
      return bool;
    }
    if matches!(close, Expect::Link) {
      if let RenderObject::Link(link) = result {
        if ch == '|' {
          *close = Expect::Link2;
          *compiler.expected.last_mut().unwrap() = Expect::Link2;
          compiler.index += 1;
        } else {
          link.to.push(ch);
          compiler.index += 1;
        }
      } else {
        panic!()
      }
    } else if matches!(close, Expect::TripleWithNamuMark) {
      if let RenderObject::NamuTriple(nt) = result {
        if ch == ' ' && nt.triplename.len() != 0 {
          *close = Expect::TripleWithNamuMark2;
        } else {
          nt.triplename.push(ch);
        }
        compiler.index += 1;
      }
    } else if matches!(close, Expect::TripleWithNamuMark2) {
      if let RenderObject::NamuTriple(nt) = result {
        if ch == '\n' {
          *close = Expect::TripleWithNamuMark3;
        } else if nt.attr == None {
          nt.attr = Some(String::from(ch))
        } else {
          nt.attr.as_mut().unwrap().push(ch);
        }
        compiler.index += 1;
      }
    } else if matches!(close, Expect::JustTriple) && !compiler.peak("{{{") {
      if let RenderObject::Literal(s) = result {
        s.push(ch);
        compiler.index += 1;
      }
    } else if matches!(close, Expect::NamuMacro(_)) {
      if let RenderObject::NamumarkMacro(namu_macro) = result {
        namu_macro.macroarg.as_mut().unwrap().push(ch);
        compiler.index += 1;
      } else {
        panic!()
      }
    } else {
      let mut thisparsing: Option<RenderObject> = None;
      if compiler.peak("[[") {
        compiler.index += 2;
        compiler.lastrollbackindex.push(compiler.index);
        compiler.expected.push(Expect::Link);
        thisparsing = Some(parse_first(compiler, Expect::Link));
      } else if compiler.peak("{{{#!syntax ") {
        compiler.index += 5;
        compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
        thisparsing = Some(parse_first(compiler, Expect::SyntaxTriple))
      } else if compiler.peak("{{{#!wiki ")
        || compiler.peak("{{{#!if ")
        || compiler.peak("{{{#!folding ")
      {
        compiler.index += 5;
        compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
        compiler.expected.push(Expect::TripleWithNamuMark);
        thisparsing = Some(parse_first(compiler, Expect::TripleWithNamuMark))
      } else if compiler.peak("{{{") {
        compiler.index += 3;
        compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
        compiler.expected.push(Expect::JustTriple);
        thisparsing = Some(parse_first(compiler, Expect::JustTriple));
      } else if compiler.peak("[date]") {
        compiler.index += 6;
        namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
          NamumarkMacro {
            macroname: String::from("date"),
            macroarg: None,
          },
        )));
        true;
      } else if compiler.peak("[datetime]") {
        compiler.index += 10;
        namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
          NamumarkMacro {
            macroname: String::from("date"),
            macroarg: None,
          },
        )));
        true;
      } else if compiler.peak("[목차]") {
        compiler.index += 4;
        namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
          NamumarkMacro {
            macroname: String::from("context"),
            macroarg: None,
          },
        )));
        true;
      } else if compiler.peak("[tableofcontents]") {
        compiler.index += 17;
        namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
          NamumarkMacro {
            macroname: String::from("context"),
            macroarg: None,
          },
        )));
        true;
      } else if compiler.peak("[각주]") {
        compiler.index += 4;
        namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
          NamumarkMacro {
            macroname: String::from("reference"),
            macroarg: None,
          },
        )));
        true;
      } else if compiler.peak("[footnote]") {
        compiler.index += 10;
        namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
          NamumarkMacro {
            macroname: String::from("reference"),
            macroarg: None,
          },
        )));
        true;
      } else if compiler.peak("[br]") {
        compiler.index += 4;
        namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
          NamumarkMacro {
            macroname: String::from("br"),
            macroarg: None,
          },
        )));
        true;
      } else if compiler.peak("[clearfix]") {
        compiler.index += 10;
        namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
          NamumarkMacro {
            macroname: String::from("clearfix"),
            macroarg: None,
          },
        )));
        true;
      } else if compiler.peak("[youtube(") {
        //TODO 이거메크로타잎 enmu 말고 String으로 저장하기
        compiler.index += 9;
        compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
        compiler
          .expected
          .push(Expect::NamuMacro(NamuMacroType::YouTube));
        thisparsing = Some(parse_first(
          compiler,
          Expect::NamuMacro(NamuMacroType::YouTube),
        ));
      } else if compiler.peak("[nicovideo(") {
        compiler.index += 11;
        compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
        compiler
          .expected
          .push(Expect::NamuMacro(NamuMacroType::NicoVideo));
        thisparsing = Some(parse_first(
          compiler,
          Expect::NamuMacro(NamuMacroType::NicoVideo),
        ));
      } else if compiler.peak("[vimeo(") {
        compiler.index += 7;
        compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
        compiler
          .expected
          .push(Expect::NamuMacro(NamuMacroType::Vimeo));
        thisparsing = Some(parse_first(
          compiler,
          Expect::NamuMacro(NamuMacroType::Vimeo),
        ));
      } else if compiler.peak("[navertv(") {
        compiler.index += 9;
        compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
        compiler
          .expected
          .push(Expect::NamuMacro(NamuMacroType::NaverTV));
        thisparsing = Some(parse_first(
          compiler,
          Expect::NamuMacro(NamuMacroType::NaverTV),
        ));
      } else if compiler.peak("[include(") {
        compiler.index += 9;
        compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
        compiler
          .expected
          .push(Expect::NamuMacro(NamuMacroType::Include));
        thisparsing = Some(parse_first(
          compiler,
          Expect::NamuMacro(NamuMacroType::Include),
        ));
      } else if compiler.peak("[age(") {
        compiler.index += 4;
        compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
        compiler
          .expected
          .push(Expect::NamuMacro(NamuMacroType::Age));
        thisparsing = Some(parse_first(compiler, Expect::NamuMacro(NamuMacroType::Age)));
      } else if compiler.peak("[dday(") {
        compiler.index += 6;
        compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
        compiler
          .expected
          .push(Expect::NamuMacro(NamuMacroType::DDay));
        thisparsing = Some(parse_first(
          compiler,
          Expect::NamuMacro(NamuMacroType::DDay),
        ));
      } else if compiler.peak("[pagecount(") {
        compiler.index += 11;
        compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
        compiler
          .expected
          .push(Expect::NamuMacro(NamuMacroType::PageCount));
        thisparsing = Some(parse_first(
          compiler,
          Expect::NamuMacro(NamuMacroType::PageCount),
        ));
      } else if compiler.peak("[ruby(") {
        compiler.index += 10;
        compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
        compiler
          .expected
          .push(Expect::NamuMacro(NamuMacroType::Ruby));
        thisparsing = Some(parse_first(
          compiler,
          Expect::NamuMacro(NamuMacroType::Ruby),
        ));
      } else if compiler.peak_line("#redirect ") {
        compiler.index += 10;
        compiler.redirect = Some(String::new());
        loop {
          if compiler.current() == Some(Objects::Char('\n')) || compiler.current() == None {
            break;
          } else {
            let current = compiler.current();
            if let Some(Objects::Char(ch)) = current {
              compiler.redirect.as_mut().unwrap().push(ch);
            } else {
              panic!()
            }
          }
          compiler.index += 1;
        }
        return false;
      } else if compiler.peak_line("##") {
        compiler.index += 2;
        let mut fix = false;
        if compiler.current() == Some(Objects::Char('@')) {
          fix = true;
        }
        loop {
          compiler.index += 1;
          if compiler.current() == Some(Objects::Char('\n')) || compiler.current() == None {
            if fix {
              compiler.fixed_comments.push("".to_string())
            }
            compiler.index += 1;
            break;
          }
          if fix {
            let current = compiler.current();
            if let Some(Objects::Char(ch)) = current {
              compiler.fixed_comments.last_mut().unwrap().push(ch);
            } else {
              panic!()
            }
          }
        }
        true;
      } else if let (true, how) = compiler.peak_repeat_line(' ', Some("1.")) {
        compiler.index += how + 2;
        thisparsing = Some(parse_first(compiler, Expect::List(how)));
        compiler.expected.push(Expect::List(0));
      } else if let (true, how) = compiler.peak_repeat_line(' ', Some("I.")) {
        compiler.index += how + 2;
        thisparsing = Some(parse_first(compiler, Expect::List(how)));
        compiler.expected.push(Expect::List(0));
      } else if let (true, how) = compiler.peak_repeat_line(' ', Some("i.")) {
        compiler.index += how + 2;
        thisparsing = Some(parse_first(compiler, Expect::List(how)));
        compiler.expected.push(Expect::List(0));
      } else if let (true, how) = compiler.peak_repeat_line(' ', Some("A.")) {
        compiler.index += how + 2;
        thisparsing = Some(parse_first(compiler, Expect::List(how)));
        compiler.expected.push(Expect::List(0));
      } else if let (true, how) = compiler.peak_repeat_line(' ', Some("a.")) {
        compiler.index += how + 2;
        thisparsing = Some(parse_first(compiler, Expect::List(how)));
        compiler.expected.push(Expect::List(0));
      } else if let (true, how) = compiler.peak_repeat_line(' ', Some("가.")) {
        compiler.index += how + 2;
        thisparsing = Some(parse_first(compiler, Expect::List(how)));
        compiler.expected.push(Expect::List(0));
      } else if let (true, how) = compiler.peak_repeat_line(' ', Some("*")) {
        compiler.index += how + 1;
        thisparsing = Some(parse_first(compiler, Expect::List(how)));
        compiler.expected.push(Expect::List(0));
      } else {
        namumarkresult.push(Objects::Char(ch));
        compiler.index += 1;
        true;
      }

      return if let Some(rendobj) = thisparsing {
        match rendobj {
          RenderObject::Nop(items) => {
            compiler.expected.pop();
            namumarkresult.extend(items);
            *result = RenderObject::Nop(a_whole_my_vec(result, namumarkresult, close));
            false
          }
          RenderObject::NopString(exp) => {
            compiler.expected.pop();
            if compiler.lastrollbackindex.len() == 1 {
              if exp == Expect::TripleWithNamuMark {
                namumarkresult.extend(slices("{{{#!".to_string()));
                compiler.index = *compiler.lastrollbackindex.last().unwrap();
              }
              if exp == Expect::Link {
                namumarkresult.extend(slices("[[".to_string()));
                compiler.index = *compiler.lastrollbackindex.last().unwrap();
              }
              if exp == Expect::JustTriple {
                namumarkresult.extend(slices("{{{".to_string()));
                compiler.index = *compiler.lastrollbackindex.last().unwrap();
              }
              compiler.lastrollbackindex.pop();
              return true;
            }
            compiler.lastrollbackindex.pop();
            *result = RenderObject::NopString(exp);
            return false;
          }
          RenderObject::EarlyParse(tuple) => {
            compiler.expected.pop();
            if tuple.0 == *close {
              match tuple.0 {
                Expect::None => {
                  panic!("아 그거 여기서 처리하는거 아닌데 ㅋㅋㄹㅃㅃㅃㅃ");
                }
                Expect::Link | Expect::Link2 => {
                  //생각해보니까 link는 earlyparse될 일이 없잖아
                  if let RenderObject::Link(link) = result {
                    link.show.extend(tuple.1.to_vec());
                  } else {
                    panic!()
                  }
                  return false;
                }
                Expect::TripleWithNamuMark3 => {
                  if let RenderObject::NamuTriple(nt) = result {
                    namumarkresult.extend(tuple.1);
                    nt.content.as_mut().unwrap().extend(namumarkresult.clone());
                    //namumarkresult는 빌려준건데 (더이상 쓸 필요 없긴 한데)
                  } else {
                    panic!()
                  }
                  return false;
                }
                Expect::TripleWithNamuMark2 => {
                  if let RenderObject::NamuTriple(nt) = result {
                    let mut rs = String::from(&nt.triplename);
                    rs.push_str(&nt.attr.as_mut().unwrap());
                    *result = RenderObject::Literal(rs);
                    return false;
                  } else {
                    panic!()
                  }
                }
                Expect::TripleWithNamuMark => {
                  if let RenderObject::NamuTriple(nt) = result {
                    let rs = String::from(&nt.triplename);
                    *result = RenderObject::Literal(rs);
                    return false;
                  } else {
                    panic!()
                  }
                }
                _ => panic!(), //여기서 처리하는 건 없음
              }
            } else {
              namumarkresult.extend(tuple.1);
              *result = RenderObject::EarlyParse((
                tuple.0,
                a_whole_my_vec(&result, namumarkresult, &close),
              ));
              return false;
            }
          } //[[ {{{#!wiki 안녕]] }}} 대충 이런거 처리용
          RenderObject::NopNopNop => panic!("이게 뭐value하는 베리언트였더라"),
          RenderObject::ListLine(ll) => {
            if let Some(Objects::RenderObject(RenderObject::List(lt))) = namumarkresult.last_mut() {
              lt.content.push(ll);
            } else {
              panic!();
            }
            return true;
          }
          obj => {
            if close == &Expect::JustTriple {
              if let RenderObject::Literal(lt) = result {
                if let RenderObject::Literal(lt2) = obj {
                  lt.push_str(&format!("{{{{{{{}}}}}}}", lt2)); //wow it sucks
                } else {
                  panic!()
                }
              } else {
                panic!()
              }
            } else {
              namumarkresult.push(Objects::RenderObject(obj));
            }
            true
          }
        }
      } else {
        true
      };
    }
  } else {
    if *close == Expect::None {
      compiler.array = namumarkresult.to_vec();
      *result = RenderObject::NopNopNop;
      return false;
    } else {
      let a = compiler.expected.clone();
      let find = a.iter().find(|ex| {
        ex == &&Expect::Link
          || ex == &&Expect::Link2
          || ex == &&Expect::TripleWithNamuMark
          || ex == &&Expect::TripleWithNamuMark2
          || ex == &&Expect::TripleWithNamuMark3
          || ex == &&Expect::JustTriple
      });
      if find == Some(&Expect::Link) || find == Some(&Expect::Link2) {
        *result = RenderObject::NopString(Expect::Link);
        return false;
      }
      if find == Some(&Expect::TripleWithNamuMark)
        || find == Some(&Expect::TripleWithNamuMark2)
        || find == Some(&Expect::TripleWithNamuMark3)
      {
        *result = RenderObject::NopString(Expect::TripleWithNamuMark);
        return false;
      }
      if find == Some(&Expect::JustTriple) {
        *result = RenderObject::NopString(Expect::JustTriple);
        return false;
      }

      *result = RenderObject::Nop(a_whole_my_vec(result, namumarkresult, close));
      return false;
    }
  }
  return true;
}
fn a_whole_my_vec(
  result: &RenderObject,
  namumarkresult: &mut Vec<Objects>,
  close: &Expect,
) -> Vec<Objects> {
  match close {
    Expect::Link => {
      let mut resultt = vec![Objects::Char('['), Objects::Char('[')];
      if let RenderObject::Link(link) = result {
        resultt.extend_from_slice(&slices(link.to.clone()));
      } else {
        panic!();
      };
      return resultt;
    }
    Expect::NamuMacro(nm) => {
      let mut resultt = vec![Objects::Char('[')];
      if let RenderObject::NamumarkMacro(macroname) = result {
        resultt.extend_from_slice(&slices(macroname.macroname.to_string()));
        resultt.push(Objects::Char('('));
        resultt.extend_from_slice(&slices(
          macroname.macroarg.clone().unwrap_or("".to_string()),
        ));
      } else {
        panic!();
      };
      return resultt;
    }
    Expect::Link2 => {
      let mut resultt = vec![Objects::Char('['), Objects::Char('[')];
      if let RenderObject::Link(link) = result {
        resultt.extend_from_slice(&slices(link.to.clone()));
        resultt.push(Objects::Char('|'));
        resultt.extend_from_slice(&namumarkresult);
      } else {
        panic!();
      };
      return resultt;
    }
    Expect::TripleWithNamuMark => {
      let mut resultt = slices("{{{#!".to_string());
      if let RenderObject::NamuTriple(nt) = result {
        resultt.extend_from_slice(&slices(nt.triplename.clone()));
      } else {
        panic!();
      };
      return resultt;
    }
    Expect::TripleWithNamuMark2 => {
      let mut resultt = slices("{{{#!".to_string());
      if let RenderObject::NamuTriple(nt) = result {
        resultt.extend_from_slice(&slices(nt.triplename.clone()));
        resultt.push(Objects::Char(' '));
        resultt.extend_from_slice(&slices(nt.attr.clone().unwrap()));
      } else {
        panic!();
      };
      return resultt;
    }
    Expect::TripleWithNamuMark3 => {
      let mut resultt = slices("{{{#!".to_string());
      if let RenderObject::NamuTriple(nt) = result {
        resultt.extend_from_slice(&slices(nt.triplename.clone()));
        resultt.push(Objects::Char(' '));
        resultt.extend_from_slice(&slices(nt.attr.clone().unwrap()));
        resultt.push(Objects::Char('\n'));
        resultt.extend_from_slice(&slices(nt.attr.clone().unwrap()));
      } else {
        panic!();
      };
      return resultt;
    }
    Expect::JustTriple => {
      let mut resultt = slices("{{{".to_string());
      if let RenderObject::Literal(lt) = result {
        resultt.extend_from_slice(&slices(lt.clone()));
        resultt.extend_from_slice(&namumarkresult);
      } else {
        panic!();
      };
      resultt
    }
    Expect::List(_) => {
      if let Some(Objects::RenderObject(RenderObject::List(lt))) = namumarkresult.last_mut()
        && let RenderObject::ListLine(ll) = result
      {
        return vec![Objects::RenderObject(RenderObject::ListLine(ListLine {
          lvl: ll.lvl,
          content: namumarkresult.to_vec(),
        }))];
      } else {
        panic!()
      }
    }
    Expect::None => {
      return namumarkresult.to_vec();
    }
    _ => {
      panic!("이거나 먹어라: {:?}", close);
    }
  }
}
pub fn slices(s: String) -> Vec<Objects> {
  let mut result: Vec<Objects> = Vec::new();
  for i in s.chars() {
    result.push(Objects::Char(i));
  }
  result
}
// 닫히는 구문 처리.
//예를들자면 ]]라던가 }}}라던가 )]라던가.....
//가독성을 위해 함수화를 함
//한 번만 호출되니까 컴파일 시간에 llvm에 의해서 삽입이 이뤄짐으로 어셈블리 상으로 call을 안할 것으로 예상
//-> 리턴 스텍을 설정하는 오버해드 걸리지 않음
//여러번 쓰이는 것을 함수화 하라고 하긴 하지만 이거는 함수화를 안하면 못읽어...
fn parsing_close(
  compiler: &mut Compiler,
  close: &Expect,
  result: &mut RenderObject,
  namumarkresult: &mut Vec<Objects>,
) -> Option<bool> {
  if compiler.peak("]]") {
    //그냥 메크로는 간단한 파싱문구라서 메게변수 없는 건 여기서 처리하지 않는 것이 맞을듯...
    if *close == Expect::Link2 || *close == Expect::Link {
      compiler.index += 2;
      compiler.lastrollbackindex.pop();
      compiler.expected.pop();
      if let RenderObject::Link(link) = result {
        link.show = namumarkresult.to_vec();
        if link.to.starts_with("파일:") {
          link.link_type = LinkType::File
        }
        if link.to.starts_with("분류:") {
          link.link_type = LinkType::Cat
        }
      } else {
        panic!("내 생각 안에서는 불가능한데");
      }
      return Some(false);
    } else if compiler.expected.get(0).unwrap() == &Expect::Link
      || compiler.expected.get(0).unwrap() == &Expect::Link2
    {
      *result = RenderObject::EarlyParse((
        compiler.expected.get(0).unwrap().clone(),
        a_whole_my_vec(result, namumarkresult, close),
      ));
      compiler.index += 2;
      return Some(false);
    } else {
      namumarkresult.push(Objects::Char(']'));
      namumarkresult.push(Objects::Char(']'));
      compiler.index += 2;
      return Some(true);
    }
  } else if compiler.peak(")]") {
    //그냥 메크로는 간단한 파싱문구라서 메게변수 없는 건 여기서 처리하지 않는 것이 맞을듯...
    if matches!(close, Expect::NamuMacro(_)) {
      compiler.index += 2;
      compiler.lastrollbackindex.pop();
      compiler.expected.pop();
      return Some(false);
    } else if let Some(Expect::NamuMacro(nt)) = compiler
      .expected
      .iter()
      .find(|x| matches!(x, Expect::NamuMacro(_)))
    {
      *result = RenderObject::EarlyParse((
        Expect::NamuMacro(nt.clone()),
        a_whole_my_vec(result, namumarkresult, close),
      ));
      compiler.index += 2;
      return Some(false);
    } else {
      namumarkresult.push(Objects::Char(')'));
      namumarkresult.push(Objects::Char(']'));
      compiler.index += 2;
      return Some(true);
    }
  } else if compiler.peak("}}}") {
    if *close == Expect::JustTriple
      || *close == Expect::SyntaxTriple
      || *close == Expect::TripleWithNamuMark3
    {
      compiler.index += 3;
      compiler.lastrollbackindex.pop();
      compiler.expected.pop();
      match result {
        RenderObject::Syntax(_) => {
          return Some(false); //이건 근데 ㄹㅇ 할깨 없음 신텍스는 문자열만 처리하는거라서
        }
        RenderObject::NamuTriple(namu_triple) => {
          //첫줄 리터럴, 두번째줄 나무마크인 것들
          namu_triple.content = Some(namumarkresult.to_vec());
          return Some(false);
        }
        RenderObject::Literal(_) => {
          return Some(false);
        }
        _ => {
          panic!()
        }
      }
    } else if *close == Expect::TripleWithNamuMark2 || *close == Expect::TripleWithNamuMark {
      let mut i = compiler.index;
      let a = loop {
        if compiler.get(i) == Some(&Objects::Char('\n')) {
          break false;
        } else if compiler.get(i) == None {
          break true;
        } else {
          i += 1;
        }
      };
      if a {
        if let RenderObject::NamuTriple(nt) = result.clone() {
          *result = RenderObject::Literal(String::from(format!(
            "#!{} {}",
            nt.triplename,
            nt.attr.unwrap_or_default()
          )));
          compiler.index += 3;
          compiler.expected.pop();
          compiler.lastrollbackindex.pop();
          return Some(false);
        } else {
          panic!()
        }
      } else {
        if let RenderObject::NamuTriple(nt) = result {
          nt.attr.as_mut().unwrap().push_str("}}}");
          compiler.index += 3;
          return Some(true);
        } else {
          panic!();
        }
      }
    } else if compiler.expected.contains(&Expect::JustTriple) {
      *result = RenderObject::EarlyParse((Expect::JustTriple, namumarkresult.to_vec()));
      compiler.index += 3;
      return Some(false);
    } else if compiler.expected.contains(&Expect::TripleWithNamuMark) {
      *result = RenderObject::EarlyParse((
        Expect::TripleWithNamuMark3,
        a_whole_my_vec(result, namumarkresult, close),
      ));
      compiler.index += 3;
      return Some(false);
    } else if compiler.expected.contains(&Expect::SyntaxTriple) {
      *result = RenderObject::EarlyParse((Expect::SyntaxTriple, namumarkresult.to_vec()));
      compiler.index += 3;
      return Some(false);
    } else if compiler.expected.contains(&Expect::TripleWithNamuMark)
      || compiler.expected.contains(&Expect::TripleWithNamuMark)
    {
      //리터럴 처리용
      *result = RenderObject::EarlyParse((Expect::TripleWithNamuMark, namumarkresult.to_vec()));
      compiler.index += 3;
      return Some(false);
    } else {
      namumarkresult.extend(slices("}}}".to_string()));
      compiler.index += 3;
      return Some(true);
    }
  }
  return None;
}
//todo 최적화.
//컴퓨터는 이런걸로 0.3초나 걸리지 않음
