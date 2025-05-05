use crate::{render::render_raw, toskens::Tokens, Compiler};
impl Compiler {
    pub fn parse(&mut self) {
        self.token_to_objects();
        for _ in 0..3 {
            self.idx = 0;
            let parsed:Vec<Objects> = Vec::new();
            let mut temp: Vec<Vec<Objects>> = Vec::new();
            temp.push(parsed);
            let mut expected_token:Vec<Tokens> = vec![Tokens::Nop];
            loop {
                match self.get_current() {
                    Objects::RenderObject(render_object) => {
                        temp.last_mut().unwrap().push(Objects::RenderObject(render_object));
                    },
                    Objects::Tokens(tokens) => {
                        match tokens {
                            Tokens::Header(level) => {
                                let expect = expected_token.iter().enumerate().rfind(|&(ref _i, &ref x)| x == &Tokens::Header(level)); //fuck
                                match expect {
                                    Some((i, _value)) => {
                                        if self.get_next() == Objects::Tokens(Tokens::NewLine) {
                                            let mut tempvec = temp.get(i).unwrap().to_owned();
                                            if i != expected_token.len()-1 {
                                                for i in i+1..expected_token.len() {
                                                    tempvec.push(expected_token.get(i).unwrap().to_owned().to_literal());
                                                    tempvec.extend(temp.get(i).unwrap().to_owned());
                                                }
                                                for _ in i..expected_token.len() {
                                                    temp.pop();
                                                    expected_token.pop();
                                                }
                                                temp.last_mut().unwrap().push(Objects::RenderObject(RenderObject::Heading(Heading {
                                                    folded:match tempvec.first().unwrap() {
                                                        Objects::Tokens(tokens) => {
                                                            match tokens  {
                                                                Tokens::Sharp => true,
                                                                _ => false
                                                            }
                                                        },
                                                        _ => false
                                                    } || match tempvec.last().unwrap() {
                                                        Objects::Tokens(tokens) => {
                                                            match tokens  {
                                                                Tokens::Sharp => true,
                                                                _ => false
                                                            }
                                                        },
                                                        _ => false
                                                    },
                                                    render_objects:tempvec.clone(),
                                                    level:level,
                                                })));
                                            } else {
                                                for _ in i..expected_token.len() {
                                                    temp.pop();
                                                    expected_token.pop();
                                                }
                                                let folded = match tempvec.first().unwrap() {
                                                    Objects::Tokens(tokens) => {
                                                        match tokens  {
                                                            Tokens::Sharp => true,
                                                            _ => false
                                                        }
                                                    },
                                                    _ => false
                                                } || match tempvec.last().unwrap() {
                                                    Objects::Tokens(tokens) => {
                                                        match tokens  {
                                                            Tokens::Sharp => true,
                                                            _ => false
                                                        }
                                                    },
                                                    _ => false
                                                };
                                                if folded {
                                                    tempvec.remove(0);
                                                    tempvec.remove(tempvec.len()-1);
                                                }
                                                temp.last_mut().unwrap().push(Objects::RenderObject(RenderObject::Heading(Heading {
                                                    folded:folded,
                                                    render_objects:tempvec.clone(),
                                                    level:level,
                                                })));
                                            }
                                        } else {
                                            temp.last_mut().unwrap().push(Objects::RenderObject(RenderObject::Literal(Literal { literal: "=".repeat(level.into()).to_string() })));
                                        }
                                    },
                                    None => {
                                        if self.get_before(1) == Objects::Tokens(Tokens::NewLine) {
                                            temp.push(Vec::new());
                                            expected_token.push(Tokens::Header(level));
                                        }
                                    },
                                }
                            }, //match header
                            Tokens::MacroOpen => {
                                temp.push(Vec::new());
                                expected_token.push(Tokens::MacroClose);
                            }, //match macroOpen
                            Tokens::MacroClose => {
                                let expect = expected_token.iter().enumerate().rfind(|&(ref _i, &ref x)| *x == Tokens::MacroClose); //fuck
                                match expect {
                                    Some((i, _value)) => {
                                        let mut tempvec = temp.get(i).unwrap().to_owned();
                                        if i != expected_token.len() {
                                            for i in i+1..expected_token.len() {
                                                tempvec.push(expected_token.get(i).unwrap().to_owned().to_literal());
                                                tempvec.extend(temp.get(i).unwrap().to_owned());
                                            }
                                            for _ in i..expected_token.len() {
                                                temp.pop();
                                                expected_token.pop();
                                            }
                                            if let Some(Objects::RenderObject(RenderObject::Literal(literal)) = tempvec.first() {
                                                match literal.literal.to_lowercase().as_str() { //메치문에서 or 어케씀???
                                                    "각주" | "footnote" | "ref" => {dont_have_argument(tempvec, &mut temp, MacroType::Footnote)},
                                                    "목차" | "toc" | "topic" | "tableofcontents" => {dont_have_argument(tempvec, &mut temp, MacroType::Topic)},
                                                    "개행" | "br" => {dont_have_argument(tempvec, &mut temp, MacroType::BreakLine)},
                                                    "삽입" | "include" => {have_argument(tempvec, &mut temp, MacroType::Include)},
                                                    "age" | "나이" => {have_argument(tempvec, &mut temp, MacroType::Include)},
                                                    "date" | "datetime" | "시간" => {dont_have_argument(tempvec, &mut temp, MacroType::Date)},
                                                    "dday" | "디데이" => {have_argument(tempvec, &mut temp, MacroType::Dday)},
                                                    "clearfix" | "플로우 속성 초기화" | "클픽" | "클리어픽스" => {dont_have_argument(tempvec, &mut temp, MacroType::ClearFix)},
                                                    "yt" | "유튶" | "유튜브" | "youtube" => {have_argument(tempvec, &mut temp, MacroType::Youtube)},
                                                    "카카오티비" | "kakaotv" | "카카오tv" => {have_argument(tempvec, &mut temp, MacroType::KakaoTV)},
                                                    "nicovideo" | "니코니코" /*| "니코카도 아보카도"*/ => {have_argument(tempvec, &mut temp, MacroType::NicoNicoTV)}, //이뭔씹.
                                                    "vimeo" | "비메오" => {have_argument(tempvec, &mut temp, MacroType::Vimeo)},
                                                    "navertv" | "네이버티비" => {have_argument(tempvec, &mut temp, MacroType::NaverTV)},
                                                    "anchor" | "링크북마크" | "엥커" => {have_argument(tempvec, &mut temp, MacroType::Anchor)}
                                                    "펼접" => {dont_have_argument(tempvec, &mut temp, MacroType::펼접)},
                                                    _ => {unexpected_macro(&mut temp, tempvec)}
                                                }
                                            } else {unexpected_macro(&mut temp, tempvec)}
                                        }
                                    }
                                    None => {
                                        temp.last_mut().unwrap().push(Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("]")})));
                                    },
                                }
                                fn unexpected_macro (temp:&mut Vec<Vec<Objects>>, tempvec:Vec<Objects>) {
                                    temp.last_mut().unwrap().push(Objects::RenderObject(RenderObject::Literal(Literal {literal : String::from("[")})));
                                    temp.last_mut().unwrap().extend(tempvec);
                                    temp.last_mut().unwrap().push(Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("]")})));
                                }
                                fn have_argument (tempvec:Vec<Objects>, temp:&mut Vec<Vec<Objects>>, macrotype:MacroType) {
                                    match (tempvec.get(1), tempvec.last().unwrap()) {
                                        (Some(object), lastobject) => {
                                            if object.to_owned() == Objects::Tokens(Tokens::Sad) && lastobject.to_owned() == Objects::Tokens(Tokens::Happy) {
                                                temp.last_mut().unwrap().push(Objects::RenderObject(RenderObject::Macro(Macro {typeofmacro: macrotype, argument:Some(render_raw((&tempvec[2..=tempvec.len()-2]).to_vec())) })));   
                                            } else {unexpected_macro(temp, tempvec)}
                                        },
                                        (None, _) => {unexpected_macro(temp, tempvec)}
                                    }
                                }
                                fn dont_have_argument (tempvec:Vec<Objects>, temp:&mut Vec<Vec<Objects>>, macrotype:MacroType) {
                                    match tempvec.get(1) {
                                        Some(_) => {unexpected_macro(temp, tempvec)},
                                        None => {temp.last_mut().unwrap().push(Objects::RenderObject(RenderObject::Macro(Macro { argument: None, typeofmacro: macrotype })))}
                                    }
                                }
                            }, //match macroClose
                            Tokens::LinkClose => {
                                let expect = expected_token.iter().enumerate().rfind(|&(ref _i, &ref x)| *x == Tokens::LinkClose); //fuck
                                match expect {
                                    Some((i, _value)) => {
                                        let mut tempvec = temp.get(i).unwrap().to_owned();
                                        if i != expected_token.len() {
                                            for i in i+1..expected_token.len() {
                                                tempvec.push(expected_token.get(i).unwrap().to_owned().to_literal());
                                                tempvec.extend(temp.get(i).unwrap().to_owned());
                                            }
                                            for _ in i..expected_token.len() {
                                                temp.pop();
                                                expected_token.pop();
                                            }
                                            let  = tempvec.split_once(|object| object == &Objects::Tokens(Tokens::PipeLine)); //슬라이스를 스플릿 하는건 불안정한 기능임. 나중에 삭제되면 크레이트라도 써야지...
                                            //밥먹으로ㅓ 떠남
                                        }
                                    }
                                    None => {
                                        temp.last_mut().unwrap().push(Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("]]")})));
                                    },
                                }
                            }, //match macroClose
                            Tokens::LinkOpen => {
                                temp.push(Vec::new());
                                expected_token.push(Tokens::LinkOpen);
                            }, //match macroClose
                            Tokens::Nop => {
                                break;
                            },
                            //몇몇 리터럴이 되면 안되는 토큰들 ~~생각해보니까 다 리터럴이 되면 안되긴 해~~ 그럼 왜 나머지 케이스로 안하냐고? 혹시 모르잖아~
                            Tokens::PipeLine => {temp.last_mut().unwrap().push(Objects::Tokens(Tokens::PipeLine))},
                            Tokens::Sharp => {temp.last_mut().unwrap().push(Objects::Tokens(Tokens::Sharp))},
                            Tokens::Sad => {temp.last_mut().unwrap().push(Objects::Tokens(Tokens::Sad))},
                            Tokens::Happy => {temp.last_mut().unwrap().push(Objects::Tokens(Tokens::Happy))},
                            tok => {
                                temp.last_mut().unwrap().push(tok.to_literal());
                            }
                        }
                    }
                }
                self.idx += 1;
            }
            self.parsetemp = temp.first().unwrap().to_owned();
        }
    }
    fn token_to_objects(&mut self) {
        for token in self.tokens.clone() {
            self.parsetemp.push(Objects::Tokens(token));
        }
    }
    fn get_current(&self) -> Objects {
        match self.parsetemp.get(self.idx) {
            Some(token) => return token.to_owned(),
            None => return Objects::Tokens(Tokens::Nop),
        }
    }
    fn get_next(&self) -> Objects {
        match self.parsetemp.get(self.idx+1) {
            Some(token) => return token.to_owned(),
            None => return Objects::Tokens(Tokens::Nop),
        }
    }
    fn get_before(&self, how_much:usize) -> Objects {
        if self.idx < how_much {
            return Objects::Tokens(Tokens::Nop.clone());
        }
        return self.parsetemp.get(self.idx-how_much).unwrap_or(&Objects::Tokens(Tokens::Nop.clone())).clone();
    }
}
#[derive(Debug,PartialEq,Clone)]
pub enum RenderObject {
    Heading(Heading),
    Literal(Literal),
    Macro(Macro)
}
#[derive(Debug,PartialEq,Clone)]
pub enum Objects {
    RenderObject(RenderObject),
    Tokens(Tokens)
}
#[derive(Debug,PartialEq,Clone)]
pub struct Macro {
    argument:Option<String>,
    typeofmacro:MacroType,
}
impl Macro {
    pub fn getarg(&self) -> Option<String> {
        return self.argument.clone()
    }
    pub fn gettype(&self) -> MacroType {
        return self.typeofmacro.clone()
    }
}
#[derive(Debug,PartialEq,Clone)]
pub enum MacroType {
    Anchor,
    Footnote,
    Topic,
    BreakLine,
    Include,
    Age,
    Date,
    Dday,
    ClearFix,
    Youtube,
    KakaoTV,
    NicoNicoTV, //i'm always two televisions away... to televisions away
    Vimeo,
    NaverTV,
    펼접,
}
#[derive(Debug,PartialEq,Clone)]
pub struct Heading {
    folded:bool,
    render_objects:Vec<Objects>,
    level:u8
}
impl Heading {
    pub fn get_lvl(&self) -> u8 {
        return self.level;
    }
    pub fn get_render_objects(&self) -> Vec<Objects> {
        return self.render_objects.clone();
    }
}
#[derive(Debug,PartialEq,Clone)]
pub struct Literal {
    pub literal:String
} 
#[derive(Debug,PartialEq,Clone)]
pub struct Main {
    render_objects:Vec<RenderObject>,
}
