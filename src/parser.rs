use core::panic;

use crate::structs::{Compiler, Expect, Link, LinkType, Objects, RenderObject};

pub fn parse_first(compiler: &mut Compiler, close: Expect) -> RenderObject {
    let mut namumarkresult: Vec<Objects> = Vec::new();
    let mut result: RenderObject = RenderObject::NopNopNop;
    let mut close = close;
    parsing_listener(compiler, &close, &namumarkresult, &mut result);
    while namumarker(compiler, &mut close, &mut namumarkresult, &mut result) {};
    result
}
fn parsing_listener(
    compiler: &mut Compiler,
    close: &Expect,
    namumarkresult: &Vec<Objects>,
    result: &mut RenderObject,
) {
    match close {
        Expect::None => *result = RenderObject::NopNopNop,
        Expect::Link => {
            *result = RenderObject::Link(Link {
                to: String::new(),
                show: Some(Vec::new()),
                link_type: LinkType::Hyper,
            })
        }
        Expect::Link2 => panic!(),
    }
}
fn namumarker(
    compiler: &mut Compiler,
    close:&mut Expect,
    namumarkresult: &mut Vec<Objects>,
    mut result: &mut RenderObject,
) -> bool {
    if let Some(Objects::Char(ch)) = compiler.current() {
        let ch = ch.to_owned();
        if ch == ']' && compiler.peak("]]") { //그냥 메크로는 간단한 파싱문구라서 메게변수 없는 건 여기서 처리하지 않는 것이 맞을듯...
                if *close == Expect::Link2 || *close == Expect::Link {
                    compiler.index += 2;
                    compiler.lastrollbackindex.pop();
                    return false;
                } else if compiler.expected.contains(&Expect::Link) || compiler.expected.contains(&Expect::Link2) {
                    *result = RenderObject::EarlyParse((Expect::Link, namumarkresult.to_vec()));
                    compiler.index += 2;
                    return false;
                } else {
                    namumarkresult.push(Objects::Char(']'));
                    namumarkresult.push(Objects::Char(']'));
                }
                compiler.index += 2;
            } else if matches!(close, Expect::Link) {
            if let RenderObject::Link(link) = result {
                if ch == '|' {
                    *close = Expect::Link2;
                    compiler.index += 1;
                } else {
                    link.to.push(ch);
                }
                // println!("{}", ch); 이럼 왜 소유권 안넘어감??? A:Copy trait을 만족시켜서
            } else {
                panic!()
            }
        } else {
            let mut thisparsing:Option<RenderObject> = None;
            if ch == '[' && compiler.peak("[[") {
                compiler.index += 2;
                compiler.lastrollbackindex.push(compiler.index);
                thisparsing = Some(parse_first(compiler, Expect::Link));
            } else {
                namumarkresult.push(Objects::Char(ch));
            }

            return if let Some(rendobj) = thisparsing {
                match rendobj {
                    RenderObject::Nop(items) => {
                        namumarkresult.extend(items);
                        *result = RenderObject::Nop(namumarkresult.to_vec());
                        false
                    },
                    RenderObject::NopForLink => {
                        if *close == Expect::Link2 && compiler.lastrollbackindex.len() == 1 {
                            compiler.index = *compiler.lastrollbackindex.last().unwrap();
                            return true
                        }
                        *result = RenderObject::NopForLink;
                        return false;
                    },
                    RenderObject::EarlyParse(tuple) => {
                        if tuple.0 == *close {
                            match tuple.0 {
                                Expect::None => {
                                  panic!("아 그거 여기서 처리하는거 아닌데 ㅋㅋㄹㅃㅃㅃㅃ");  
                                },
                                Expect::Link2 => { //생각해보니까 link는 earlyparse될 일이 없잖아
                                    if let RenderObject::Link(link) = result {
                                        link.show.as_mut().unwrap().extend(tuple.1.to_vec());
                                    } else {panic!()}
                                    return false;
                                },
                                _ => panic!(), //여기서 처리하는 건 없음
                            }
                        } else {
                            namumarkresult.extend(tuple.1);
                            *result = RenderObject::EarlyParse((tuple.0, namumarkresult.to_vec()));
                            return false;
                        }
                    } //[[ {{{#!wiki 안녕]] }}} 대충 이런거 처리용
                    RenderObject::NopNopNop => panic!("이게 뭐하는 베리언트였더라"),
                    obj => {namumarkresult.push(Objects::RenderObject(obj)); true}
                }
            } else {
                false
            }
        }
    }
    return true;
}
