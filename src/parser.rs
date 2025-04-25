use crate::{main, toskens::Tokens, Compiler};
impl Compiler {
    pub fn parse(&mut self) {
        let main_vec:Vec<RenderObject> = Vec::new();
        let here_vec:Vec<RenderObject> = main_vec; //와 이건 벡터가 포인터라서 좋은 점임
        let mut expect_token:Vec<Tokens> = Vec::new();
        self.idx = 0;
        let curr_token = match self.get_current() {
            Some(token) => {
                token
            },
            None => Tokens::Nop,
        };
        match curr_token {
            Tokens::Header(level) => {
                let currentidx = self.idx.clone();
                while true {
                    match self.get_current() {
                        Some(token) => {
                            match token {
                                Tokens::Header(lvl) => {
                                    if lvl == level {
                                        
                                    }
                                },
                                _ => {}
                            }
                        },
                        None => {

                        }
                    }
                }
            },
            _ => {

            }
        }
    }
    fn get_current(&self) -> Option<Tokens> {
        return self.tokens.get(self.idx).cloned()
    }
    fn get_before(&self, how_much:usize) -> Tokens {
        if self.idx < how_much {
            return  Tokens::Nop.clone();
        }
        return self.tokens.get(self.idx-how_much).unwrap_or(&Tokens::Nop).clone();
    }
}
#[derive(Debug)]
pub enum RenderObject {
    Heading(Heading)
}
#[derive(Debug)]
pub struct Heading {
    folded:bool,
    render_objects:Vec<RenderObject>,
    level:u8
}