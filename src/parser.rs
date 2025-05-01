use crate::{main, toskens::Tokens, Compiler};
impl Compiler {
    pub fn parse(&mut self) {
        self.token_to_objects();
        for _ in 0..3 {
            let parsed:Vec<Objects> = Vec::new();
            let mut temp:Vec<Vec<Objects>> = Vec::new();
            temp.push(parsed);
            loop {
                let mut expected_token:Vec<Tokens> = vec![Tokens::Nop];
                self.idx = 0;
                match self.get_current() {
                    Objects::RenderObject(render_object) => {
                        temp.last_mut().unwrap().push(Objects::RenderObject(render_object));
                    },
                    Objects::Tokens(tokens) => {
                        match tokens {
                            Tokens::Header(level) => {
                                let expect = expected_token.iter().enumerate().rfind(|&(ref _i, &ref x)| *x == Tokens::Header(level)); //fuck
                                match expect {
                                    Some((i, _value)) => {
                                        if self.get_next() == Objects::Tokens(Tokens::NewLine) {
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
                                                temp.last_mut().unwrap().push(Objects::RenderObject(RenderObject::Heading(Heading {
                                                    folded:match tempvec.first().unwrap() {
                                                        Objects::RenderObject(_render_object) => false,
                                                        Objects::Tokens(tokens) => {
                                                            match tokens.to_owned() {
                                                                Tokens::Literal(string) => { //이게 String은 포인터다보니까 이런짓을 하는것. 왜냐면 eq로 비교를 하면 활당되는 메모리 주소가 다르니까 무조건 false가 뜸 러스트의 안좋은 점중 하나.
                                                                    match string.as_str() {
                                                                        "#" => true,
                                                                        _ => false
                                                                    }
                                                                },
                                                                _ => false
                                                            }
                                                        },
                                                    } || match tempvec.last().unwrap() {
                                                        Objects::RenderObject(_render_object) => false,
                                                        Objects::Tokens(tokens) => {
                                                            match tokens.to_owned() {
                                                                Tokens::Literal(string) => { //이게 String은 포인터다보니까 이런짓을 하는것. 왜냐면 eq로 비교를 하면 활당되는 메모리 주소가 다르니까 무조건 false가 뜸 러스트의 안좋은 점중 하나.
                                                                    match string.as_str() {
                                                                        "#" => true,
                                                                        _ => false
                                                                    }
                                                                },
                                                                _ => false
                                                            }
                                                        },
                                                    },
                                                    render_objects:tempvec,
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
                                todo!()
                            }, //match macroOpen
                            Tokens::MacroClose => {
                                todo!()
                            }, //match macroClose
                            Tokens::Happy => {
                                todo!()
                            }, //match happycat
                            Tokens::Sad => {
                                todo!()
                            }, //match bananacat
                            _ => {}
                        }
                    }
                }
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
#[derive(Debug,PartialEq,Clone)]
pub struct Literal {
    pub literal:String
} 
#[derive(Debug,PartialEq,Clone)]
pub struct Main {
    render_objects:Vec<RenderObject>,
}