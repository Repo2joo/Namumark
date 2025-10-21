use core::panic;
use std::{mem::discriminant, vec};

use crate::{
  renderobjs::{
    Color, Heading, Link, LinkType, List, ListLine, Minus, NamuTriple, NamumarkMacro, Plus, Quote,
    QuoteLine, RenderObject,
  },
  structs::{Compiler, Expect, ListType, NamuMacroType, Objects},
};

pub(crate) fn parse_first(compiler: &mut Compiler, close: Expect) -> RenderObject {
  let mut namumarkresult: Vec<Objects> = Vec::new();
  let mut result: RenderObject = RenderObject::NopNopNop;
  let mut close = close;
  if !prepare_result(&close, &mut result, compiler) {
    return result;
  }
  while namumarker(compiler, &mut close, &mut namumarkresult, &mut result) {}
  result
}
fn prepare_result(close: &Expect, result: &mut RenderObject, compiler: &mut Compiler) -> bool {
  match close {
    Expect::None => *result = RenderObject::NopNopNop,
    Expect::Link => {
      let index = compiler.index;
      let mut to = String::new();
      loop {
        if let Some(Objects::Char(ch)) = compiler.get(compiler.index) {
          let ch = ch.to_owned();
          if compiler.peak("]]") {
            compiler.index += 2;
            compiler.expected.pop();
            *result = RenderObject::Link(Link {
              to,
              show: Vec::new(),
              link_type: LinkType::Hyper,
            });
            let what: Vec<Objects> = Vec::new();
            last_dance(result, &what);
            return false;
          }
          if ch == '|' {
            compiler.index += 1;
            break;
          }
          to.push(ch);
          compiler.index += 1;
        } else {
          compiler.index = index;
          compiler.expected.pop();
          *result = compiler.get_before_earlyparse(slices("[[".to_string()));
          return false;
        }
      }
      *result = RenderObject::Link(Link {
        to,
        show: Vec::new(),
        link_type: LinkType::Hyper,
      })
    }
    Expect::TripleWithNamuMark => {
      let mut triplename = String::new();
      let mut do_attr = false;
      let mut attr = String::new();
      let index = compiler.index;
      loop {
        if let Some(Objects::Char(ch)) = compiler.get(compiler.index) {
          let ch = ch.to_owned();
          if ch == '\n' {
            compiler.index += 1;
            break;
          }
          if ch == ' ' {
            do_attr = true;
            compiler.index += 1;
            continue;
          }
          if !do_attr {
            triplename.push(ch);
          } else {
            attr.push(ch);
          }
          compiler.index += 1;
        } else {
          compiler.index = index;
          compiler.expected.pop();
          *result = compiler.get_before_earlyparse(slices("{{{#!".to_string()));
          return false;
        }
      }
      *result = RenderObject::NamuTriple(NamuTriple {
        triplename,
        attr: Some(attr),
        content: Some(Vec::new()),
      })
    }
    Expect::Reference => {
      let mut name = String::new();
      let index = compiler.index;
      loop {
        if compiler.peak(" ") {
          compiler.index += 1;
          if name.is_empty() {
            *result = RenderObject::Reference(crate::renderobjs::Reference {
              name: None,
              content: Some(Vec::new()),
            });
          } else {
            *result = RenderObject::Reference(crate::renderobjs::Reference {
              name: Some(name),
              content: Some(Vec::new()),
            });
          }
          return true;
        } else if compiler.peak("]") {
          if name.is_empty() {
            *result = RenderObject::Reference(crate::renderobjs::Reference {
              name: None,
              content: None,
            });
          } else {
            *result = RenderObject::Reference(crate::renderobjs::Reference {
              name: Some(name),
              content: None,
            });
          }
          return false;
        } else if compiler.current().is_none() {
          compiler.index = index;
          *result = RenderObject::AddBefore(slices("[*".to_string()));
          return false;
        } else {
          compiler.index += 1;
          if let Some(Objects::Char(ch)) = compiler.current() {
            name.push(ch);
          }
        }
      }
    }
    Expect::JustTriple => {
      let mut triplecount: usize = 1;
      let index = compiler.index;
      let mut string = String::new();
      loop {
        if compiler.current().is_none() {
          compiler.index = index;
          *result = compiler.get_before_earlyparse(slices("{{{".to_string()));
          compiler.expected.pop();
          return false;
        }
        if compiler.peak("{{{") {
          compiler.index += 3;
          string.push_str("{{{");
          triplecount += 1;
        } else if compiler.peak("}}}") {
          compiler.index += 3;
          triplecount -= 1;
          if triplecount == 0 {
            compiler.expected.pop();
            break;
          } else {
            string.push_str("}}}");
          }
        } else if let Some(Objects::Char(ch)) = compiler.current() {
          string.push(ch);
          compiler.index += 1;
        }
      }
      *result = RenderObject::Literal(string);
      return false;
    }
    Expect::NamuMacro => {
      let mut macroname = String::new();
      let mut do_macroarg = false;
      let mut macroarg = String::new();
      let index = compiler.index;
      loop {
        if let Some(Objects::Char(ch)) = compiler.get(compiler.index) {
          let ch = ch.to_owned();
          if compiler.peak(")]") {
            compiler.index += 2;
            break;
          }
          if ch == '(' {
            compiler.index += 1;
            do_macroarg = true;
            continue;
          }
          if !do_macroarg {
            macroname.push(ch);
          } else {
            macroarg.push(ch);
          }
          compiler.index += 1;
        } else {
          compiler.index = index;
          compiler.expected.pop();
          *result = compiler.get_before_earlyparse(slices("[".to_string()));
          return false;
        }
      }
      let macrotype = match macroname.to_lowercase().as_str() {
        "youtube" => NamuMacroType::YouTube,
        "kakaotv" => NamuMacroType::KakaoTV,
        "nicovideo" => NamuMacroType::NicoVideo,
        "vimeo" => NamuMacroType::Vimeo,
        "navertv" => NamuMacroType::NaverTV,
        "include" => NamuMacroType::Include,
        "age" => NamuMacroType::Age,
        "dday" => NamuMacroType::DDay,
        "pagecount" => NamuMacroType::PageCount,
        "ruby" => NamuMacroType::Ruby,
        _ => NamuMacroType::Custom,
      };
      *result = RenderObject::NamumarkMacro(NamumarkMacro {
        macroname,
        macroarg: Some(macroarg),
        macrotype,
      });
      return false;
    }
    Expect::List(lvl) => {
      *result = RenderObject::ListLine(ListLine {
        lvl: *lvl,
        content: Vec::new(),
      })
    }
    Expect::Quote(lvl) => {
      *result = RenderObject::QuoteLine(QuoteLine {
        lvl: *lvl,
        content: Vec::new(),
      })
    }
    Expect::Heading(lvl) => {
      *result = RenderObject::Heading(Heading {
        lvl: *lvl,
        folded: false,
        content: Vec::new(),
      })
    }
    Expect::Color => {
      let mut rst = RenderObject::Color(Color {
        first: String::new(),
        second: None,
        content: Vec::new(),
      });
      let mut resultt = String::from("#");
      loop {
        if let Some(&Objects::Char(ch)) = compiler.get(compiler.index) {
          if ch.eq(&' ') {
            if let RenderObject::Color(ref mut cl) = rst {
              if cl.first.is_empty() {
                cl.first = resultt.clone();
              } else {
                cl.second = Some(resultt.clone());
              }
            }
            break;
          } else if ch.eq(&',') {
            if let RenderObject::Color(ref mut cl) = rst {
              cl.first = resultt.clone();
              resultt.clear();
            }
          } else {
            resultt.push(ch);
          }
          compiler.index += 1;
        } else {
          panic!("issue https://github.com/repo2joo/namumark");
        }
      }
      *result = rst;
    }
    Expect::Plus => {
      *result = RenderObject::Plus(Plus {
        content: Vec::new(),
        how: 0,
      })
    }
    Expect::Minus => {
      *result = RenderObject::Minus(Minus {
        content: Vec::new(),
        how: 0,
      })
    }
    _ => {
      panic!("issue Repo2joo/Namumark!")
    }
  }
  true
}
fn namumarker(
  compiler: &mut Compiler,
  close: &mut Expect,
  namumarkresult: &mut Vec<Objects>,
  result: &mut RenderObject,
) -> bool {
  fn listeq(namumarkresultlast: Option<&Objects>, listtype: ListType) -> bool {
    if let Some(Objects::RenderObject(RenderObject::List(lt))) = namumarkresultlast
      && lt.listtype == listtype
    {
      return false;
    }
    true
  }
  if let Some(Objects::Char(ch)) = compiler.current() {
    if compiler.rollbacks.is_some()
      && let Some((_, how, _)) = compiler.expected.get(compiler.rollbacks.unwrap())
      && how == &compiler.index
    {
      compiler.index += 1;
      compiler.expected.pop();
      namumarkresult.push(Objects::Char(ch));
      return true;
    }
    let ch = ch.to_owned();
    let whattodo = parsing_close(compiler, close, result, namumarkresult);
    if let Some(bool) = whattodo {
      return bool;
    }
    let mut thisparsing: Option<RenderObject> = None;
    if compiler.peak("[[") {
      compiler.expected.push((Expect::Link, compiler.index, true));
      compiler.index += 2;
      thisparsing = Some(parse_first(compiler, Expect::Link));
    } else if compiler.peak("{{{#!wiki ")
      || compiler.peak("{{{#!if ")
      || compiler.peak("{{{#!folding ")
    {
      compiler
        .expected
        .push((Expect::TripleWithNamuMark, compiler.index, true));
      compiler.index += 5;
      thisparsing = Some(parse_first(compiler, Expect::TripleWithNamuMark))
    } else if compiler.is_color() {
      compiler.index += 4;
      compiler
        .expected
        .push((Expect::Color, compiler.index, false));
      thisparsing = Some(parse_first(compiler, Expect::Color));
    } else if compiler.peak("{{{+") && {
      if let Objects::Char(ch) = compiler.get(compiler.index + 4).unwrap() {
        ch.to_string().parse().is_ok_and(|num| matches!(num, 0..=5))
      } else {
        false
      }
    } {
      compiler.index += 4;
      compiler
        .expected
        .push((Expect::Plus, compiler.index, false));
      thisparsing = Some(parse_first(compiler, Expect::Plus));
    } else if compiler.peak("{{{-") && {
      if let Objects::Char(ch) = compiler.get(compiler.index + 4).unwrap() {
        ch.to_string().parse().is_ok_and(|num| matches!(num, 0..=5))
      } else {
        false
      }
    } {
      compiler.index += 4;
      compiler
        .expected
        .push((Expect::Minus, compiler.index, false));
      thisparsing = Some(parse_first(compiler, Expect::Minus));
    } else if compiler.peak("{{{") {
      compiler.index += 3;
      compiler
        .expected
        .push((Expect::JustTriple, compiler.index, false));
      thisparsing = Some(parse_first(compiler, Expect::JustTriple));
    } else if compiler.peak("[*") {
      compiler.index += 2;
      compiler
        .expected
        .push((Expect::Reference, compiler.index, true));
      thisparsing = Some(parse_first(compiler, Expect::Reference));
    } else if compiler.peak("[date]") {
      compiler.index += 6;
      namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
        NamumarkMacro {
          macroname: String::from("date"),
          macroarg: None,
          macrotype: NamuMacroType::Date,
        },
      )));
      return true;
    } else if compiler.peak("[datetime]") {
      compiler.index += 10;
      namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
        NamumarkMacro {
          macroname: String::from("datetime"),
          macroarg: None,
          macrotype: NamuMacroType::Date,
        },
      )));
      return true;
    } else if compiler.peak("[목차]") {
      compiler.index += 4;
      namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
        NamumarkMacro {
          macroname: String::from("목차"),
          macroarg: None,
          macrotype: NamuMacroType::TableOfContents,
        },
      )));
      return true;
    } else if compiler.peak("[tableofcontents]") {
      compiler.index += 17;
      namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
        NamumarkMacro {
          macroname: String::from("tableofcontents"),
          macroarg: None,
          macrotype: NamuMacroType::TableOfContents,
        },
      )));
      return true;
    } else if compiler.peak("[각주]") {
      compiler.index += 4;
      namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
        NamumarkMacro {
          macroname: String::from("각주"),
          macroarg: None,
          macrotype: NamuMacroType::Reference,
        },
      )));
      return true;
    } else if compiler.peak("[footnote]") {
      compiler.index += 10;
      namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
        NamumarkMacro {
          macroname: String::from("footnote"),
          macroarg: None,
          macrotype: NamuMacroType::Reference,
        },
      )));
      return true;
    } else if compiler.peak("[br]") {
      compiler.index += 4;
      namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
        NamumarkMacro {
          macroname: String::from("br"),
          macroarg: None,
          macrotype: NamuMacroType::Br,
        },
      )));
      return true;
    } else if compiler.peak("[clearfix]") {
      compiler.index += 10;
      namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
        NamumarkMacro {
          macroname: String::from("clearfix"),
          macroarg: None,
          macrotype: NamuMacroType::Clearfix,
        },
      )));
      return true;
    } else if let Some(s) = compiler.peak_macro() {
      namumarkresult.push(Objects::RenderObject(RenderObject::NamumarkMacro(
        NamumarkMacro {
          macroname: s,
          macroarg: None,
          macrotype: NamuMacroType::Custom,
        },
      )));
      return true;
    } else if compiler.peak("[youtube(")
      || compiler.peak("[nicovideo(")
      || compiler.peak("[vimeo(")
      || compiler.peak("[navertv(")
      || compiler.peak("[kakaotv(")
      || compiler.peak("[include(")
      || compiler.peak("[age(")
      || compiler.peak("[dday(")
      || compiler.peak("[pagecount(")
      || compiler.peak("[ruby(")
      || compiler.peak_macro_arg()
    {
      compiler.index += 1;
      compiler
        .expected
        .push((Expect::NamuMacro, compiler.index, false));
      thisparsing = Some(parse_first(compiler, Expect::NamuMacro));
    } else if compiler.peak_line("#redirect ") {
      compiler.index += 10;
      compiler.redirect = Some(String::new());
      loop {
        if compiler.current() == Some(Objects::Char('\n')) || compiler.current().is_none() {
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
        if compiler.current() == Some(Objects::Char('\n')) || compiler.current().is_none() {
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
      return true;
    } else if let (true, how) = compiler.peak_repeat_line(' ', Some("1.")) {
      compiler.index += how + 2;
      compiler
        .expected
        .push((Expect::List(0), compiler.index, false));
      thisparsing = Some(parse_first(compiler, Expect::List(how)));
      if listeq(namumarkresult.last(), ListType::Arabia) {
        namumarkresult.push(Objects::RenderObject(RenderObject::List(List {
          from: Some(0),
          listtype: ListType::Arabia,
          content: Vec::new(),
        })));
      }
    } else if let (true, how) = compiler.peak_repeat_line(' ', Some("I.")) {
      compiler.index += how + 2;
      compiler
        .expected
        .push((Expect::List(0), compiler.index, false));
      thisparsing = Some(parse_first(compiler, Expect::List(how)));
      if listeq(namumarkresult.last(), ListType::RomanBig) {
        namumarkresult.push(Objects::RenderObject(RenderObject::List(List {
          from: Some(0),
          listtype: ListType::RomanBig,
          content: Vec::new(),
        })));
      }
    } else if let (true, how) = compiler.peak_repeat_line(' ', Some("i.")) {
      compiler.index += how + 2;
      compiler
        .expected
        .push((Expect::List(0), compiler.index, false));
      thisparsing = Some(parse_first(compiler, Expect::List(how)));
      if listeq(namumarkresult.last(), ListType::RomanSmall) {
        namumarkresult.push(Objects::RenderObject(RenderObject::List(List {
          from: Some(0),
          listtype: ListType::RomanSmall,
          content: Vec::new(),
        })));
      }
    } else if let (true, how) = compiler.peak_repeat_line(' ', Some("A.")) {
      compiler.index += how + 2;
      compiler
        .expected
        .push((Expect::List(0), compiler.index, false));
      thisparsing = Some(parse_first(compiler, Expect::List(how)));
      if listeq(namumarkresult.last(), ListType::AlphaBig) {
        namumarkresult.push(Objects::RenderObject(RenderObject::List(List {
          from: Some(0),
          listtype: ListType::AlphaBig,
          content: Vec::new(),
        })));
      }
    } else if let (true, how) = compiler.peak_repeat_line(' ', Some("a.")) {
      compiler.index += how + 2;
      compiler
        .expected
        .push((Expect::List(0), compiler.index, false));
      thisparsing = Some(parse_first(compiler, Expect::List(how)));
      if listeq(namumarkresult.last(), ListType::AlphaSmall) {
        namumarkresult.push(Objects::RenderObject(RenderObject::List(List {
          from: Some(0),
          listtype: ListType::AlphaSmall,
          content: Vec::new(),
        })));
      }
    } else if let (true, how) = compiler.peak_repeat_line(' ', Some("가.")) {
      compiler.index += how + 2;
      compiler
        .expected
        .push((Expect::List(0), compiler.index, false));
      thisparsing = Some(parse_first(compiler, Expect::List(how)));
      if listeq(namumarkresult.last(), ListType::Hangul) {
        namumarkresult.push(Objects::RenderObject(RenderObject::List(List {
          from: Some(0),
          listtype: ListType::Hangul,
          content: Vec::new(),
        })));
      }
    } else if let (true, how) = compiler.peak_repeat_line(' ', Some("*")) {
      compiler.index += how + 1;
      compiler
        .expected
        .push((Expect::List(0), compiler.index, false));
      thisparsing = Some(parse_first(compiler, Expect::List(how)));
      if listeq(namumarkresult.last(), ListType::List) {
        namumarkresult.push(Objects::RenderObject(RenderObject::List(List {
          from: Some(0),
          listtype: ListType::List,
          content: Vec::new(),
        })));
      }
    } else if let (true, how) = compiler.peak_repeat_line('>', None) {
      if how <= 8 {
        compiler.index += how;
        thisparsing = Some(parse_first(compiler, Expect::Quote(how)));
        compiler
          .expected
          .push((Expect::List(0), compiler.index, false));
      } else {
        compiler.index += 1;
        namumarkresult.push(Objects::Char('>'));
      }
      if !matches!(
        namumarkresult.last(),
        Some(Objects::RenderObject(RenderObject::Quote(_)))
      ) {
        namumarkresult.push(Objects::RenderObject(RenderObject::Quote(Quote {
          content: Vec::new(),
        })));
      }
    } else if let (true, how) = compiler.peak_repeat_line('=', None) {
      if how <= 6 {
        compiler.index += how;
        thisparsing = Some(parse_first(compiler, Expect::Heading(how)));
        compiler
          .expected
          .push((Expect::Heading(0), compiler.index, false));
      } else {
        compiler.index += 1;
        namumarkresult.push(Objects::Char('='));
      }
    } else {
      namumarkresult.push(Objects::Char(ch));
      compiler.index += 1;
      return true;
    }

    if let Some(rendobj) = thisparsing {
      match rendobj {
        RenderObject::LastRollBack => {
          if Expect::None == *close {
            compiler.index = compiler.expected.first().unwrap().1;
            return true;
          }
          *result = RenderObject::LastRollBack;
          return false;
        }
        RenderObject::Nop(items) => {
          compiler.expected.pop();
          namumarkresult.extend(items);
          *result = RenderObject::Nop(a_whole_my_vec(result, namumarkresult, close));
          false
        }
        RenderObject::EarlyParseRollBack(exp) => {
          if exp == *close {
            compiler.index = compiler
              .expected
              .get(compiler.rollbacks.unwrap())
              .unwrap()
              .1;
            return true;
          }
          *result = RenderObject::EarlyParseRollBack(exp);
          return false;
        }
        RenderObject::AddBefore(vec) => {
          compiler.expected.pop();
          namumarkresult.extend(vec);
          return true;
        }
        RenderObject::EarlyParse(tuple) => {
          compiler.expected.pop();
          if discriminant(close) == discriminant(&tuple.0) {
            match tuple.0 {
              Expect::Link => {
                if let RenderObject::Link(link) = result {
                  link.show.extend(tuple.1.to_vec());
                } else {
                  panic!()
                }
                return false;
              }
              Expect::Color => {
                if let RenderObject::Color(cl) = result {
                  cl.content.extend(tuple.1.to_vec());
                  return false;
                } else {
                  panic!()
                }
              }
              Expect::Plus => {
                if let RenderObject::Plus(pl) = result {
                  pl.content.extend(tuple.1.to_vec());
                  return false;
                } else {
                  panic!()
                }
              }
              Expect::Minus => {
                if let RenderObject::Minus(mx) = result {
                  mx.content.extend(tuple.1.to_vec());
                  return false;
                } else {
                  panic!()
                }
              }
              Expect::TripleWithNamuMark => {
                if let RenderObject::NamuTriple(nt) = result {
                  namumarkresult.extend(tuple.1);
                  nt.content.as_mut().unwrap().extend(namumarkresult.clone());
                } else {
                  panic!()
                }
                return false;
              }
              Expect::List(_) => {
                if let RenderObject::ListLine(ll) = result {
                  ll.content.extend(tuple.1);
                }
                return false;
              }
              Expect::Quote(_) => {
                if let RenderObject::QuoteLine(ll) = result {
                  ll.content.extend(tuple.1);
                }
                return false;
              }
              Expect::Heading(_) => {
                if let RenderObject::Heading(hd) = result {
                  hd.content.extend(namumarkresult.to_vec());
                  return false;
                } else {
                  panic!()
                }
              }
              _ => panic!(),
            }
          } else {
            namumarkresult.extend(tuple.1);
            *result =
              RenderObject::EarlyParse((tuple.0, a_whole_my_vec(result, namumarkresult, close)));
            return false;
          }
        }
        RenderObject::ListLine(ll) => {
          if let Some(Objects::RenderObject(RenderObject::List(lt))) = namumarkresult.last_mut() {
            lt.content.push(ll);
          } else {
            panic!();
          }
          return true;
        }
        RenderObject::QuoteLine(ql) => {
          if let Some(Objects::RenderObject(RenderObject::Quote(qt))) = namumarkresult.last_mut() {
            qt.content.push(ql);
          } else {
            panic!();
          }
          return true;
        }
        obj => {
          namumarkresult.push(Objects::RenderObject(obj));
          true
        }
      }
    } else {
      true
    };
  } else if *close == Expect::None {
    compiler.array = namumarkresult.to_vec();
    *result = RenderObject::NopNopNop;
    return false;
  } else {
    if let Expect::List(how) = close {
      *result = RenderObject::ListLine(ListLine {
        lvl: *how,
        content: namumarkresult.to_vec(),
      });
      return false;
    }
    let listfind = compiler.contains_for_parsing_more(|x| matches!(x, &Expect::List(_)));

    if let (true, what, how, Expect::List(listhow)) = listfind {
      if what {
        compiler.rollbacks = Some(how);
        *result = RenderObject::EarlyParseRollBack(Expect::List(listhow));
        return false;
      } else {
        *result = RenderObject::EarlyParse((
          Expect::List(listhow),
          a_whole_my_vec(result, namumarkresult, close),
        ));
        return false;
      }
    }
    if let Expect::Quote(how) = close {
      *result = RenderObject::QuoteLine(QuoteLine {
        lvl: *how,
        content: namumarkresult.to_vec(),
      });
      return false;
    }
    if let (true, what, how, Expect::Quote(quotehow)) =
      compiler.contains_for_parsing_more(|x| matches!(x, &Expect::Quote(_)))
    {
      if what {
        compiler.rollbacks = Some(how);
        *result = RenderObject::EarlyParseRollBack(Expect::Quote(quotehow));
        return false;
      } else {
        *result = RenderObject::EarlyParse((
          Expect::Quote(quotehow),
          a_whole_my_vec(result, namumarkresult, close),
        ));
        return false;
      }
    }
    if let (_, true, us) = compiler.contains_for_parsing(|x| x == &Expect::None) {
      compiler.rollbacks = Some(us);
      *result = RenderObject::LastRollBack;
      return false;
    }
    *result = RenderObject::Nop(a_whole_my_vec(result, namumarkresult, close));
    return false;
  }
  true
}
fn a_whole_my_vec(
  result: &RenderObject,
  namumarkresult: &mut Vec<Objects>,
  close: &Expect,
) -> Vec<Objects> {
  match close {
    Expect::NamuMacro => {
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
      resultt
    }
    Expect::Link => {
      let mut resultt = vec![Objects::Char('['), Objects::Char('[')];
      if let RenderObject::Link(link) = result {
        resultt.extend_from_slice(&slices(link.to.clone()));
        resultt.push(Objects::Char('|'));
        resultt.extend_from_slice(namumarkresult);
      } else {
        panic!();
      };
      resultt
    }
    Expect::Color => {
      let mut resultt = slices("{{{#".to_string());
      if let RenderObject::Color(cl) = result {
        resultt.extend_from_slice(&slices(cl.first.clone()));
        resultt.push(Objects::Char(','));
        resultt.extend_from_slice(&slices(cl.second.clone().unwrap()));
        resultt.push(Objects::Char(' '));
      } else {
        panic!();
      };
      resultt
    }
    Expect::Plus => {
      let mut resultt = slices("{{{+".to_string());
      if let RenderObject::Plus(pl) = result {
        resultt.extend_from_slice(&slices(pl.how.to_string()));
        resultt.push(Objects::Char(' '));
        resultt.extend_from_slice(&pl.content);
      } else {
        panic!();
      };
      resultt
    }
    Expect::Minus => {
      let mut resultt = slices("{{{-".to_string());
      if let RenderObject::Minus(mx) = result {
        resultt.extend_from_slice(&slices(mx.how.to_string()));
        resultt.push(Objects::Char(' '));
        resultt.extend_from_slice(&mx.content);
      } else {
        panic!();
      };
      resultt
    }
    Expect::TripleWithNamuMark => {
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
      resultt
    }
    Expect::JustTriple => {
      let mut resultt = slices("{{{".to_string());
      if let RenderObject::Literal(lt) = result {
        resultt.extend_from_slice(&slices(lt.clone()));
        resultt.extend_from_slice(namumarkresult);
      } else {
        panic!();
      };
      resultt
    }
    Expect::List(_) => {
      if let RenderObject::ListLine(ll) = result {
        vec![Objects::RenderObject(RenderObject::ListLine(ListLine {
          lvl: ll.lvl,
          content: namumarkresult.to_vec(),
        }))]
      } else {
        panic!()
      }
    }
    Expect::Quote(_) => {
      if let RenderObject::QuoteLine(ql) = result {
        vec![Objects::RenderObject(RenderObject::QuoteLine(QuoteLine {
          lvl: ql.lvl,
          content: namumarkresult.to_vec(),
        }))]
      } else {
        panic!()
      }
    }
    Expect::Heading(how) => {
      let mut rt = slices("=".repeat(*how));
      rt.extend(namumarkresult.to_vec());
      rt
    }
    Expect::None => namumarkresult.to_vec(),
    _ => {
      panic!("이거나 먹어라: {:?}", close);
    }
  }
}
pub(crate) fn slices(s: String) -> Vec<Objects> {
  let mut result: Vec<Objects> = Vec::new();
  for i in s.chars() {
    result.push(Objects::Char(i));
  }
  result
}
fn parsing_close(
  compiler: &mut Compiler,
  close: &Expect,
  result: &mut RenderObject,
  namumarkresult: &mut Vec<Objects>,
) -> Option<bool> {
  if compiler.peak("]") {
    if *close == Expect::Reference {
      compiler.index += 1;
      compiler.expected.pop();
      return Some(false);
    } else if let (true, what, how) = compiler.contains_for_parsing(|x| x == &Expect::Reference) {
      compiler.index += 1;
      if what {
        *result = RenderObject::EarlyParseRollBack(Expect::Reference);
        compiler.rollbacks = Some(how);
        return Some(false);
      } else {
        compiler.expected.pop();
        *result = RenderObject::EarlyParse((
          Expect::Reference,
          a_whole_my_vec(result, namumarkresult, close),
        ));
        return Some(false);
      }
    } else if compiler.peak("]]") {
      compiler.index += 2;
      if *close == Expect::Link {
        compiler.expected.pop();
        last_dance(result, namumarkresult);
        return Some(false);
      } else if let (true, what, how) = compiler.contains_for_parsing(|x| x == &Expect::Link) {
        if what {
          *result = RenderObject::EarlyParseRollBack(Expect::Link);
          compiler.rollbacks = Some(how);
          return Some(false);
        } else {
          compiler.expected.pop();
          *result =
            RenderObject::EarlyParse((Expect::Link, a_whole_my_vec(result, namumarkresult, close)));
          return Some(false);
        }
      } else {
        namumarkresult.push(Objects::Char(']'));
        namumarkresult.push(Objects::Char(']'));
        return Some(true);
      }
    }
  } else if compiler.peak("\n") {
    if matches!(close, Expect::List(_)) {
      compiler.index += 1;
      compiler.expected.pop();
      if let RenderObject::ListLine(ll) = result {
        ll.content = namumarkresult.to_vec();
      }
      return Some(false);
    }
    if let Some((Expect::List(lt), _, _)) = compiler
      .expected
      .clone()
      .iter()
      .find(|x| matches!(x.0, Expect::List(_)))
      && let (true, what, how) = compiler.contains_for_parsing(|x| x == &Expect::List(*lt))
    {
      compiler.index += 1;
      if what {
        *result = RenderObject::EarlyParseRollBack(Expect::List(*lt));
        compiler.rollbacks = Some(how);
        return Some(false);
      } else {
        compiler.expected.pop();
        *result = RenderObject::EarlyParse((
          Expect::List(*lt),
          a_whole_my_vec(result, namumarkresult, close),
        ));
        return Some(false);
      }
    }
    if matches!(close, Expect::Quote(_)) {
      compiler.index += 1;
      compiler.expected.pop();
      if let RenderObject::QuoteLine(ql) = result {
        ql.content = namumarkresult.to_vec();
      }
      return Some(false);
    } else if let Some((Expect::Quote(qt), _, _)) = compiler
      .expected
      .clone()
      .iter()
      .find(|x| matches!(x.0, Expect::Quote(_)))
      && let (true, what, how) = compiler.contains_for_parsing(|x| x == &Expect::Quote(*qt))
    {
      compiler.index += 1;
      if what {
        *result = RenderObject::EarlyParseRollBack(Expect::Quote(*qt));
        compiler.rollbacks = Some(how);
        return Some(false);
      } else {
        compiler.expected.pop();
        *result = RenderObject::EarlyParse((
          Expect::Quote(*qt),
          a_whole_my_vec(result, namumarkresult, close),
        ));
        return Some(false);
      }
    }
    if let Some(idx) = compiler
      .expected
      .iter()
      .position(|x| matches!(x.0, Expect::Heading(_)))
      && let Some((_, b, c)) = compiler
        .expected
        .clone()
        .iter()
        .find(|x| matches!(x.0, Expect::Heading(_)))
    {
      compiler.expected.remove(idx);
      compiler.expected.insert(idx, (Expect::None, *b, *c));
      return Some(true);
    }
    return None;
  } else if compiler.peak("=\n")
    || (compiler.peak("=") && compiler.index + 1 == compiler.array.len())
  {
    compiler.index += 2;
    if matches!(close, Expect::Heading(_)) {
      last_dance(result, namumarkresult);
      compiler.expected.pop();
      return Some(false);
    } else if let Some((Expect::Heading(hd), _, _)) = compiler
      .expected
      .iter()
      .find(|x| matches!(x.0, Expect::Heading(_)))
      && let (true, what, how) = compiler.contains_for_parsing(|x| x == &Expect::Heading(*hd))
    {
      if what {
        *result = RenderObject::EarlyParseRollBack(Expect::Heading(*hd));
        compiler.rollbacks = Some(how);
        return Some(false);
      } else {
        *result = RenderObject::EarlyParse((
          Expect::Heading(*hd),
          a_whole_my_vec(result, namumarkresult, close),
        ));
        return Some(false);
      }
    } else {
      namumarkresult.push(Objects::Char('='));
      namumarkresult.push(Objects::Char('\n'));
      return Some(true); 

    }
  } else if compiler.peak("}}}") {
    compiler.index += 3;
    if *close == Expect::TripleWithNamuMark {
      compiler.expected.pop();
      if let RenderObject::NamuTriple(nt) = result {
        nt.content = Some(namumarkresult.to_vec());
        return Some(false);
      }
    }
    if *close == Expect::Plus
      && let RenderObject::Plus(pl) = result
    {
      if let Some(Objects::Char(ch)) = namumarkresult.first() {
        pl.how = ch.to_string().parse().unwrap();
        namumarkresult.remove(0);
        namumarkresult.remove(0);
      } else {
        panic!();
      }
      pl.content = namumarkresult.to_vec();
      compiler.expected.pop();
      return Some(false);
    }
    if *close == Expect::Minus
      && let RenderObject::Minus(pl) = result
    {
      if let Some(Objects::Char(ch)) = namumarkresult.first() {
        pl.how = ch.to_string().parse().unwrap();
        namumarkresult.remove(0);
        namumarkresult.remove(0);
      } else {
        panic!();
      }
      pl.content = namumarkresult.to_vec();
      compiler.expected.pop();
      return Some(false);
    }
    let find = compiler.contains_for_parsing_more(|exp| {
      exp == &Expect::Color
        || exp == &Expect::TripleWithNamuMark
        || exp == &Expect::Plus
        || exp == &Expect::Minus
    });
    if let (true, what, how, Expect::Color) = find {
      if what {
        *result = RenderObject::EarlyParseRollBack(Expect::Color);
        compiler.rollbacks = Some(how);
        return Some(false);
      } else {
        *result =
          RenderObject::EarlyParse((Expect::Color, a_whole_my_vec(result, namumarkresult, close)));
        return Some(false);
      }
    } else if let (true, what, how, Expect::TripleWithNamuMark) = find {
      if what {
        *result = RenderObject::EarlyParseRollBack(Expect::TripleWithNamuMark);
        compiler.rollbacks = Some(how);
        return Some(false);
      } else {
        *result = RenderObject::EarlyParse((
          Expect::TripleWithNamuMark,
          a_whole_my_vec(result, namumarkresult, close),
        ));
        return Some(false);
      }
    } else if let (true, what, how, Expect::Plus) = find {
      if what {
        *result = RenderObject::EarlyParseRollBack(Expect::Plus);
        compiler.rollbacks = Some(how);
        return Some(false);
      } else {
        *result =
          RenderObject::EarlyParse((Expect::Plus, a_whole_my_vec(result, namumarkresult, close)));
        return Some(false);
      }
    } else if let (true, what, how, Expect::Minus) = find {
      if what {
        *result = RenderObject::EarlyParseRollBack(Expect::Minus);
        compiler.rollbacks = Some(how);
        return Some(false);
      } else {
        *result =
          RenderObject::EarlyParse((Expect::Minus, a_whole_my_vec(result, namumarkresult, close)));
        return Some(false);
      }
    }
    namumarkresult.extend(slices("}}}".to_string()));
    return Some(true);
  }
  None
}
fn last_dance(result: &mut RenderObject, namumarkresult: &Vec<Objects>) {
  match result {
    RenderObject::Link(link) => {
      link.show = namumarkresult.to_vec();
      if link.to.starts_with("파일:") {
        link.link_type = LinkType::File
      }
      if link.to.starts_with("분류:") {
        link.link_type = LinkType::Cat
      }
    }
    RenderObject::Heading(hd) => {
      hd.content = namumarkresult.to_vec();
      let mut index = 1;
      let mut reversed = namumarkresult.to_owned();
      reversed.reverse();
      for item in reversed {
        if let Objects::Char('=') = item
          && index < 6
        {
          index += 1;
        } else {
          let bigger = std::cmp::max(hd.lvl, index);
          if hd.lvl == index {
            if namumarkresult.first() == Some(&Objects::Char('#'))
              && namumarkresult.get(namumarkresult.len() - index) == Some(&Objects::Char('#'))
            {
              hd.folded = true;
            }
            for _ in 1..index {
              hd.content.pop();
            }
          } else if bigger == hd.lvl {
            if namumarkresult.first() == Some(&Objects::Char('#'))
              && namumarkresult.get(namumarkresult.len() - index) == Some(&Objects::Char('#'))
            {
              hd.folded = true;
            }
            for _ in 1..index {
              hd.content.insert(0, Objects::Char('='));
              hd.content.pop();
            }
            hd.lvl = index;
          } else if bigger == index {
            if namumarkresult.first() == Some(&Objects::Char('#'))
              && namumarkresult.get(namumarkresult.len() - index) == Some(&Objects::Char('#'))
            {
              hd.folded = true;
            }
            for _ in 1..hd.lvl {
              hd.content.pop();
            }
          }
          break;
        }
      }
    }
    _ => {
      panic!("issue gh")
    }
  }
}
