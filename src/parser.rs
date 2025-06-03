use core::panic;

use crate::{
    renderobjs::{Languages, Link, LinkType, NamuTriple, RenderObject, Syntax},
    structs::{Compiler, Expect, Objects},
};

pub fn parse_first(compiler: &mut Compiler, close: Expect) -> RenderObject {
    let mut namumarkresult: Vec<Objects> = Vec::new();
    let mut result: RenderObject = RenderObject::NopNopNop;
    let mut close = close;
    parsing_listener(compiler, &close, &namumarkresult, &mut result);
    while namumarker(compiler, &mut close, &mut namumarkresult, &mut result) {
        if compiler.lastrollbackindex.len() == 61 {
            panic!(
                "미완성인 리터럴을 이따구로 도베하는 것은 허용되지 않는답니다. 리터럴 처리를 꼭 해주세요"
            )
        }
    }
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
    }
}
fn namumarker(
    compiler: &mut Compiler,
    mut close: &mut Expect,
    namumarkresult: &mut Vec<Objects>,
    mut result: &mut RenderObject,
) -> bool {
    if let Some(Objects::Char(ch)) = compiler.current() {
        let ch = ch.to_owned();
        if ch == ']' && compiler.peak("]]") {
            //그냥 메크로는 간단한 파싱문구라서 메게변수 없는 건 여기서 처리하지 않는 것이 맞을듯...
            if *close == Expect::Link2 || *close == Expect::Link {
                compiler.index += 2;
                compiler.lastrollbackindex.pop();
                compiler.expected.pop();
                if let RenderObject::Link(link) = result {
                    link.show = Some(namumarkresult.to_vec());
                } else {
                    panic!("내 생각 안에서는 불가능한데");
                }
                return false;
            } else if compiler.expected.contains(&Expect::Link)
                || compiler.expected.contains(&Expect::Link2)
            {
                *result = RenderObject::EarlyParse((
                    Expect::Link,
                    a_whole_my_vec(result, namumarkresult, &Expect::Link),
                ));
                compiler.index += 2;
                return false;
            } else {
                compiler.index += 2;
                namumarkresult.push(Objects::Char(']'));
                namumarkresult.push(Objects::Char(']'));
            }
            compiler.index += 2;
        } else if ch == '}' && compiler.peak("}}}") {
            if *close == Expect::JustTriple
                || *close == Expect::TripleWithNamuMark2
                || *close == Expect::TripleWithNamuMark
                || *close == Expect::SyntaxTriple
                || *close == Expect::TripleWithNamuMark3
            {
                compiler.index += 3;
                compiler.lastrollbackindex.pop();
                compiler.expected.pop();
                match result {
                    RenderObject::Syntax(_) => {
                        return false; //이건 근데 ㄹㅇ 할깨 없음 신텍스는 문자열만 처리하는거라서
                    }
                    RenderObject::NamuTriple(namu_triple) => {
                        //첫줄 리터럴, 두번째줄 나무마크인 것들
                        namu_triple.content = Some(namumarkresult.to_vec());
                        return false;
                    },
                    RenderObject::Literal(_) => {
                        return false;
                    },
                    _ => {
                        panic!()
                    }
                }
            } else if compiler.expected.contains(&Expect::JustTriple) {
                *result = RenderObject::EarlyParse((Expect::JustTriple, namumarkresult.to_vec()));
                compiler.index += 3;
                return false;
            } else if compiler.expected.contains(&Expect::TripleWithNamuMark) {
                *result = RenderObject::EarlyParse((
                    Expect::TripleWithNamuMark3,
                    a_whole_my_vec(result, namumarkresult, close),
                ));
                compiler.index += 3;
                return false;
            } else if compiler.expected.contains(&Expect::SyntaxTriple) {
                //이 contains구문 너무 비효울적임. find로 잘 ㅎ래서 함수화 하셈 TODO 
                *result = RenderObject::EarlyParse((Expect::SyntaxTriple, namumarkresult.to_vec()));
                compiler.index += 3;
                return false;
            } else if compiler.expected.contains(&Expect::TripleWithNamuMark)
                || compiler.expected.contains(&Expect::TripleWithNamuMark)
            {
                //리터럴 처리용
                *result =
                    RenderObject::EarlyParse((Expect::TripleWithNamuMark, namumarkresult.to_vec()));
                compiler.index += 3;
                return false;
            } else {
                namumarkresult.push(Objects::Char('}'));
                namumarkresult.push(Objects::Char('}'));
                namumarkresult.push(Objects::Char('}'));
            }
            compiler.index += 3;
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
        } else if matches!(close, Expect::JustTriple) {
            if let RenderObject::Literal(s) = result {
                s.push(ch);
                compiler.index += 1;
            }
        } else {
            let mut thisparsing: Option<RenderObject> = None;
            if ch == '[' && compiler.peak("[[") {
                compiler.index += 2;
                compiler.lastrollbackindex.push(compiler.index);
                compiler.expected.push(Expect::Link);
                thisparsing = Some(parse_first(compiler, Expect::Link));
            } else if ch == '{' && compiler.peak("{{{#!syntax ") {
                compiler.index += 5;
                compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
                thisparsing = Some(parse_first(compiler, Expect::SyntaxTriple))
            } else if ch == '{'
                && (compiler.peak("{{{#!wiki ")
                    || compiler.peak("{{{#!if ")
                    || compiler.peak("{{{#!folding "))
            {
                compiler.index += 5;
                compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
                compiler.expected.push(Expect::TripleWithNamuMark);
                thisparsing = Some(parse_first(compiler, Expect::TripleWithNamuMark))
            } else if ch == '{' && compiler.peak("{{{") {
                compiler.index += 5;
                compiler.lastrollbackindex.push(compiler.index); //트리플 문들은 첫출은 다 리터럴이던데
                thisparsing = Some(parse_first(compiler, Expect::JustTriple))
            } else {
                namumarkresult.push(Objects::Char(ch));
                println!("{:?}", namumarkresult);
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
                            compiler.index = *compiler.lastrollbackindex.last().unwrap();
                            if exp == Expect::Link {
                                namumarkresult.push(Objects::Char('['));
                                namumarkresult.push(Objects::Char('[')); //아니 이게 인정하긴 싫은데 작동은 해 진짜 전형적인. 그니까 분명 예외가 있을 것 같은데 예외가 없는 미친 캐이스. 그니까 earlyparse 단계에서는 a_whole_my_vec이 자동으로 처리해주다 보니까 예외를 생각하기가 쉽지가 않음
                            //이런거 특징: 나중에 겁나 큰 예외 생겨서 갈아엎어야함
                            //오늘의 결론:주석화 잘하자 코드가 1000줄이 될 위기가 보이니 함수 분리 + 깃헙에 질문창을 열거나 해야겠음ㅇㅅㅇ;;;
                            } else if exp == Expect::TripleWithNamuMark {
                                namumarkresult.push(Objects::Char('{'));
                                namumarkresult.push(Objects::Char('{'));
                                namumarkresult.push(Objects::Char('{'));
                                namumarkresult.push(Objects::Char('#'));
                                namumarkresult.push(Objects::Char('!')); //아니 이게 인정하긴 싫은데 작동은 해 진짜 전형적인. 그니까 분명 예외가 있을 것 같은데 예외가 없는 미친 캐이스. 그니까 earlyparse 단계에서는 a_whole_my_vec이 자동으로 처리해주다 보니까 예외를 생각하기가 쉽지가 않음
                                //이런거 특징: 나중에 겁나 큰 예외 생겨서 갈아엎어야함
                                //오늘의 결론:주석화 잘하자 코드가 1000줄이 될 위기가 보이니 함수 분리 + 깃헙에 질문창을 열거나 해야겠음ㅇㅅㅇ;;;
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
                                Expect::Link2 => {
                                    //생각해보니까 link는 earlyparse될 일이 없잖아
                                    if let RenderObject::Link(link) = result {
                                        link.show.as_mut().unwrap().extend(tuple.1.to_vec());
                                    } else {
                                        panic!()
                                    }
                                    return false;
                                }
                                Expect::TripleWithNamuMark3 => {
                                    if let RenderObject::NamuTriple(nt) = result {
                                        namumarkresult.extend(tuple.1);
                                        nt.content.as_mut().unwrap().extend(namumarkresult.clone());
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
                                a_whole_my_vec(&result, &namumarkresult, &close),
                            ));
                            return false;
                        }
                    } //[[ {{{#!wiki 안녕]] }}} 대충 이런거 처리용
                    RenderObject::NopNopNop => panic!("이게 뭐하는 베리언트였더라"),
                    obj => {
                        namumarkresult.push(Objects::RenderObject(obj));
                        true
                    }
                }
            } else {
                true
            }
        }
    } else {
        if *close == Expect::None {
            compiler.array = namumarkresult.to_vec();
            *result = RenderObject::NopNopNop;
            return false;
        } else {
            let find = compiler.expected.iter().find(|ex| {
                ex == &&Expect::Link
                    || ex == &&Expect::Link2
                    || ex == &&Expect::TripleWithNamuMark
                    || ex == &&Expect::TripleWithNamuMark2
                    || ex == &&Expect::TripleWithNamuMark3
            });
            if (*close == Expect::Link2 || *close == Expect::Link)
                && compiler.lastrollbackindex.len() != 1
            {
                compiler.lastrollbackindex.pop();
            }
            if find == Some(&Expect::Link) || find == Some(&Expect::Link2) {
                *result = RenderObject::NopString(Expect::Link);
                return false;
            }
            if (*close == Expect::TripleWithNamuMark
                || *close == Expect::TripleWithNamuMark2
                || *close == Expect::TripleWithNamuMark3)
                && compiler.lastrollbackindex.len() != 1
            {
                compiler.lastrollbackindex.pop();
            }
            if find == Some(&Expect::TripleWithNamuMark) {
                *result = RenderObject::NopString(Expect::TripleWithNamuMark);
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
    namumarkresult: &Vec<Objects>,
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
            let mut resultt = vec![
                Objects::Char('{'),
                Objects::Char('{'),
                Objects::Char('{'),
                Objects::Char('#'),
                Objects::Char('!'),
            ];
            if let RenderObject::NamuTriple(nt) = result {
                resultt.extend_from_slice(&slices(nt.triplename.clone()));
            } else {
                panic!();
            };
            return resultt;
        }
        Expect::TripleWithNamuMark2 => {
            let mut resultt = vec![
                Objects::Char('{'),
                Objects::Char('{'),
                Objects::Char('{'),
                Objects::Char('#'),
                Objects::Char('!'),
            ];
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
            let mut resultt = vec![
                Objects::Char('{'),
                Objects::Char('{'),
                Objects::Char('{'),
                Objects::Char('#'),
                Objects::Char('!'),
            ];
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
        }Expect::None => {
            return namumarkresult.to_vec();
        }
        _ => {
            panic!("이거나 먹어라: {:?}", close);
        }
    }
}
fn slices(s: String) -> Vec<Objects> {
    let mut result: Vec<Objects> = Vec::new();
    for i in s.chars() {
        result.push(Objects::Char(i));
    }
    result
}
