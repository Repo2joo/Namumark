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
                                let expect = expected_token.iter().enumerate().rfind(|&(_i, &x)| x == Tokens::Header(level));
                                match expect {
                                    Some((i, value)) => {
                                        temp.get(i).unwrap()
                                    },
                                    None => {
                                        temp.push(Vec::new());
                                        expected_token.push(Tokens::Header(level));
                                    },
                                }
                            },
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
            Some(token) => return token.clone(),
            None => return Objects::Tokens(Tokens::Nop),
        }
    }
    fn get_before(&self, how_much:usize) -> Tokens {
        if self.idx < how_much {
            return  Tokens::Nop.clone();
        }
        return self.tokens.get(self.idx-how_much).unwrap_or(&Tokens::Nop).clone();
    }
}
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum RenderObject {
    Heading(Heading)
}
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Objects {
    RenderObject(RenderObject),
    Tokens(Tokens)
}
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Heading {
    folded:bool,
    render_objects:Vec<RenderObject>,
    level:u8
}
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Main {
    render_objects:Vec<RenderObject>,
}