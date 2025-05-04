use crate::{toskens::Tokens, Compiler};
impl Compiler {
    pub fn parse(&mut self) {
        self.token_to_objects();
        for _ in 0..3 {
            self.idx = 0;
            let parsed:Vec<Objects> = Vec::new();
            let mut temp:Vec<Vec<Objects>> = Vec::new();
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
                                            match tempvec.first() {
                                                Some(objects) => {
                                                    match objects {
                                                        Objects::RenderObject(render_object) => {
                                                            match render_object {
                                                                RenderObject::Literal(literal) => {
                                                                    match literal.literal.as_str() { //메치문에서 or 어케씀???
                                                                        "각주" | "footnote" | "ref" => {
                                                                            
                                                                        },
                                                                        "목차" | "toc" | "tableofcontents" => {

                                                                        },
                                                                        "개행" | "br" => {
                                                                            
                                                                        }
                                                                        "삽입" | "include" => {

                                                                        },
                                                                        "age" | "나이" => {

                                                                        },
                                                                        "date" | "datetime" | "시간" => {

                                                                        },
                                                                        "dday" | "디데이" => {

                                                                        },
                                                                        "clearfix" | "플로우 속성 초기화" | "클픽" => {

                                                                        },
                                                                        "yt" | "유튶" | "유튜브" | "youtube" => {

                                                                        },
                                                                        "카카오티비" | "kakaotv" | "카카오tv" => {

                                                                        },
                                                                        "nicovideo" => {}, //이뭔씹
                                                                        "vimeo" => {

                                                                        },
                                                                        "navertv" => {

                                                                        }
                                                                        "펼접" => {

                                                                        }
                                                                        _ => todo!() //음
                                                                    }
                                                                },
                                                                _ => {}
                                                            }
                                                        },
                                                        Objects::Tokens(tokens) => {},
                                                    }
                                                },
                                                None => todo!(),
                                            }
                                        }
                                    }
                                    None => {
                                        temp.last_mut().unwrap().push(Objects::RenderObject(RenderObject::Literal(Literal {literal:String::from("]")})));
                                    },
                                }
                            }, //match macroClose
                            Tokens::Nop => {
                                break;
                            }
                            tok => {
                                temp.last_mut().unwrap().push(tok.to_literal());
                            }
                        }
                    }
                }
                self.idx += 1;
            }
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
    Literal(Literal)
}
#[derive(Debug,PartialEq,Clone)]
pub enum Objects {
    RenderObject(RenderObject),
    Tokens(Tokens)
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