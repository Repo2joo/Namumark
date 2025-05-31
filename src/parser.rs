use core::panic;

use crate::{renderobjs::{Languages, Link, LinkType, NamuTriple, RenderObject, Syntax}, structs::{Compiler, Expect, Objects}};

pub fn parse_first(compiler: &mut Compiler, close: Expect) -> RenderObject {
    let mut namumarkresult: Vec<Objects> = Vec::new();
    let mut result: RenderObject = RenderObject::NopNopNop;
    let mut close = close;
    parsing_listener(compiler, &close, &namumarkresult, &mut result);
    while namumarker(compiler, &mut close, &mut namumarkresult, &mut result) {if compiler.lastrollbackindex.len() == 61 {panic!("미완성인 리터럴을 이따구로 도베하는 것은 허용되지 않는답니다. 리터럴 처리를 꼭 해주세요")}};
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
        Expect::SyntaxTriple => {
                *result = RenderObject::Syntax(Syntax {
                    language:Languages::NotSupported,
                    content:String::new(),
                })
            },
        Expect::TripleWithNamuMark => {
            *result = RenderObject::NamuTriple(NamuTriple {
                    triplename:String::new(),
                    attr:String::new(),
                    content:Vec::new(),
                })
        },
        Expect::TripleWithNamuMark2 => panic!(),
        Expect::JustTriple => todo!(),
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
                compiler.expected.pop();
                if let RenderObject::Link(link) = result{
                    link.show = Some(namumarkresult.to_vec());
                } else {
                    panic!("내 생각 안에서는 불가능한데");
                }
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
        } else if ch == '}' &&  compiler.peak("}}}") {
            if *close == Expect::JustTriple || *close == Expect::TripleWithNamuMark || *close == Expect::TripleWithNamuMark || *close == Expect::SyntaxTriple {
                compiler.index += 3;
                compiler.lastrollbackindex.pop();
                compiler.expected.pop();
                if let RenderObject::Link(link) = result{
                    link.show = Some(namumarkresult.to_vec());
                } else {
                    panic!("내 생각 안에서는 불가능한데");
                }
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
                    compiler.index += 1;
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
                compiler.expected.push(Expect::Link);
                thisparsing = Some(parse_first(compiler, Expect::Link));
            } else if ch == '{' && compiler.peak("{{{#!syntax ") {
                compiler.index += 5;
                compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
                thisparsing = Some(parse_first(compiler, Expect::SyntaxTriple))
            } else if ch == '{' && (compiler.peak("{{{#!wiki ") || compiler.peak("{{{#!if ") || compiler.peak("{{{#!folding ")) {
                compiler.index += 5;
                compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
                thisparsing = Some(parse_first(compiler, Expect::TripleWithNamuMark))
            } else if ch == '{' && compiler.peak("{{{") {
                compiler.index += 5;
                compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
                thisparsing = Some(parse_first(compiler, Expect::JustTriple))
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
                        *result = RenderObject::Nop(namumarkresult.to_vec());
                        false
                    },
                    RenderObject::NopForLink => {
                        compiler.expected.pop();
                        if compiler.lastrollbackindex.len() == 1 {
                            compiler.index = *compiler.lastrollbackindex.last().unwrap();
                            compiler.lastrollbackindex.pop();
                            return true
                        }
                        compiler.lastrollbackindex.pop();
                        *result = RenderObject::NopForLink;
                        return false;
                    },
                    RenderObject::EarlyParse(tuple) => {
                        compiler.expected.pop();
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
                            *result = RenderObject::EarlyParse((tuple.0, a_whole_my_vec(&result, &namumarkresult, &close)));
                            return false;
                        }
                    } //[[ {{{#!wiki 안녕]] }}} 대충 이런거 처리용
                    RenderObject::NopNopNop => panic!("이게 뭐하는 베리언트였더라"),
                    obj => {namumarkresult.push(Objects::RenderObject(obj)); true}
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
            if (*close == Expect::Link2 || *close == Expect::Link) && compiler.lastrollbackindex.len() != 1 {
                compiler.lastrollbackindex.pop();
            }
            if compiler.expected.contains(&Expect::Link) || compiler.expected.contains(&Expect::Link2) {
                *result = RenderObject::NopForLink;
                return false;
            }
            print!("{:?}", compiler.expected);
            *result = RenderObject::Nop(a_whole_my_vec(result, namumarkresult, close));
            return false;
        }
    }
    return true;
}
fn a_whole_my_vec (result: &RenderObject, namumarkresult: &Vec<Objects>, close:&Expect) -> Vec<Objects> {
    match close {
        Expect::Link => {
            let mut resultt = vec![Objects::Char('['), Objects::Char('[')];
            if let RenderObject::Link(link) = result {
                resultt.extend_from_slice(&slices(link.to.clone()));
            } else {
                panic!();
            };
            return resultt;
        },
        Expect::Link2 => {
            let mut resultt = namumarkresult.to_vec();
            if let RenderObject::Link(link) = result {
                resultt.extend_from_slice(&slices(link.to.clone()));
            } else {
                panic!();
            };
            return resultt;
        },
        _ => {
            panic!("이거나 먹어라");
        }
    }
}
fn slices(s:String) -> Vec<Objects> {
    let mut result: Vec<Objects> = Vec::new();
    for i in s.chars() {
        result.push(Objects::Char(i));
    }
    result
}