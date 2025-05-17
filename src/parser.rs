use crate::{Compiler, render::render_raw, toskens::Tokens};
impl Compiler {
    pub fn parse(&mut self) {
        self.token_to_objects();
        for _ in 0..3 {
            self.idx = 0;
            let parsed: Vec<Objects> = Vec::new();
            let mut temp: Vec<Vec<Objects>> = Vec::new();
            temp.push(parsed);
            let mut pipelinecount = 0;
            let mut expected_token: Vec<Tokens> = vec![Tokens::Nop];
            loop {
                match self.get_current() {
                    Objects::RenderObject(render_object) => {
                        temp.last_mut()
                            .unwrap()
                            .push(Objects::RenderObject(render_object));
                    }
                    Objects::Tokens(tokens) => {
                        match tokens {
                            Tokens::Header(level) => {
                                let expect = expected_token
                                    .iter()
                                    .enumerate()
                                    .rfind(|&(ref _i, &ref x)| x == &Tokens::Header(level)); //fuck
                                match expect {
                                    Some((i, _value)) => {
                                        if self.get_next() == Objects::Tokens(Tokens::NewLine) {
                                            let mut tempvec = temp.get(i).unwrap().to_owned();
                                            if i != expected_token.len() - 1 {
                                                for i in i + 1..expected_token.len() {
                                                    tempvec.push(
                                                        expected_token
                                                            .get(i)
                                                            .unwrap()
                                                            .to_owned()
                                                            .to_literal(),
                                                    );
                                                    tempvec.extend(temp.get(i).unwrap().to_owned());
                                                }
                                                for _ in i..expected_token.len() {
                                                    temp.pop();
                                                    expected_token.pop();
                                                }
                                                temp.last_mut().unwrap().push(
                                                    Objects::RenderObject(RenderObject::Heading(
                                                        Heading {
                                                            folded: match tempvec.first().unwrap() {
                                                                Objects::Tokens(tokens) => {
                                                                    match tokens {
                                                                        Tokens::Sharp => true,
                                                                        _ => false,
                                                                    }
                                                                }
                                                                _ => false,
                                                            } || match tempvec
                                                                .last()
                                                                .unwrap()
                                                            {
                                                                Objects::Tokens(tokens) => {
                                                                    match tokens {
                                                                        Tokens::Sharp => true,
                                                                        _ => false,
                                                                    }
                                                                }
                                                                _ => false,
                                                            },
                                                            render_objects: tempvec.clone(),
                                                            level,
                                                        },
                                                    )),
                                                );
                                            } else {
                                                for _ in i..expected_token.len() {
                                                    temp.pop();
                                                    expected_token.pop();
                                                }
                                                let folded = match tempvec.first().unwrap() {
                                                    Objects::Tokens(tokens) => match tokens {
                                                        Tokens::Sharp => true,
                                                        _ => false,
                                                    },
                                                    _ => false,
                                                } || match tempvec.last().unwrap() {
                                                    Objects::Tokens(tokens) => match tokens {
                                                        Tokens::Sharp => true,
                                                        _ => false,
                                                    },
                                                    _ => false,
                                                };
                                                if folded {
                                                    tempvec.remove(0);
                                                    tempvec.remove(tempvec.len() - 1);
                                                }
                                                temp.last_mut().unwrap().push(
                                                    Objects::RenderObject(RenderObject::Heading(
                                                        Heading {
                                                            folded,
                                                            render_objects: tempvec.clone(),
                                                            level,
                                                        },
                                                    )),
                                                );
                                            }
                                        } else {
                                            temp.last_mut().unwrap().push(Objects::RenderObject(
                                                RenderObject::Literal(Literal {
                                                    literal: "=".repeat(level.into()).to_string(),
                                                }),
                                            ));
                                        }
                                    }
                                    None => {
                                        if self.get_before(1) == Objects::Tokens(Tokens::NewLine) {
                                            temp.push(Vec::new());
                                            expected_token.push(Tokens::Header(level));
                                        }
                                    }
                                }
                            } //match header
                            Tokens::MacroOpen => {
                                temp.push(Vec::new());
                                expected_token.push(Tokens::MacroOpen);
                            } //match macroOpen
                            Tokens::MacroClose => {
                                let expect = expected_token
                                    .iter()
                                    .enumerate()
                                    .find(|&(ref _i, &ref x)| *x == Tokens::MacroOpen); //fuck
                                match expect {
                                    Some((i, _value)) => {
                                        let mut tempvec = temp.get(i).unwrap().to_owned();
                                        if i != expected_token.len() - 1 {
                                            for i in i + 1..expected_token.len() {
                                                tempvec.push(
                                                    expected_token
                                                        .get(i)
                                                        .unwrap()
                                                        .to_owned()
                                                        .to_literal(),
                                                );
                                                tempvec.extend(temp.get(i).unwrap().to_owned());
                                            }
                                        }
                                        for _ in i..expected_token.len() {
                                            temp.pop();
                                            expected_token.pop();
                                        }
                                        if let Some(Objects::RenderObject(RenderObject::Literal(
                                            literal,
                                        ))) = tempvec.first()
                                        {
                                            match literal.literal.to_lowercase().as_str() { //메치문에서 or 어케씀???
                                                    "각주" | "footnote" | "ref" => {dont_have_argument(tempvec, &mut temp, MacroType::Footnote)},
                                                    "목차" | "toc" | "topic" | "tableofcontents" => {dont_have_argument(tempvec, &mut temp, MacroType::Topic)},
                                                    "개행" | "br" => {dont_have_argument(tempvec, &mut temp, MacroType::BreakLine)},
                                                    "삽입" | "include" => {have_argument(tempvec, &mut temp, MacroType::Include)},
                                                    "age" | "나이" => {have_argument(tempvec, &mut temp, MacroType::Age)},
                                                    "date" | "datetime" | "시간" => {dont_have_argument(tempvec, &mut temp, MacroType::Date)},
                                                    "dday" | "디데이" => {have_argument(tempvec, &mut temp, MacroType::Dday)},
                                                    "clearfix" | "플로우 속성 초기화" | "클픽" | "클리어픽스" => {dont_have_argument(tempvec, &mut temp, MacroType::ClearFix)},
                                                    "yt" | "유튶" | "유튜브" | "youtube" => {have_argument(tempvec, &mut temp, MacroType::Youtube)},
                                                    "카카오티비" | "kakaotv" | "카카오tv" => {have_argument(tempvec, &mut temp, MacroType::KakaoTV)},
                                                    "nicovideo" | "니코니코" /*| "니코카도 아보카도"*/ => {have_argument(tempvec, &mut temp, MacroType::NicoNicoTV)}, //이뭔씹.
                                                    "vimeo" | "비메오" => {have_argument(tempvec, &mut temp, MacroType::Vimeo)},
                                                    "navertv" | "네이버티비" => {have_argument(tempvec, &mut temp, MacroType::NaverTV)},
                                                    "anchor" | "링크북마크" | "엥커" => {have_argument(tempvec, &mut temp, MacroType::Anchor)}
                                                    "펼접" => {dont_have_argument(tempvec, &mut temp, MacroType::펼접)}, //나중에 투표 추가할 예정
                                                    _ => {unexpected_macro(&mut temp, tempvec)}
                                                }
                                        } else {
                                            unexpected_macro(&mut temp, tempvec)
                                        }
                                    }
                                    None => {
                                        temp.last_mut().unwrap().push(Objects::RenderObject(
                                            RenderObject::Literal(Literal {
                                                literal: String::from("]"),
                                            }),
                                        ));
                                    }
                                }
                                fn unexpected_macro(
                                    temp: &mut Vec<Vec<Objects>>,
                                    tempvec: Vec<Objects>,
                                ) {
                                    temp.last_mut().unwrap().push(Objects::RenderObject(
                                        RenderObject::Literal(Literal {
                                            literal: String::from("["),
                                        }),
                                    ));
                                    temp.last_mut().unwrap().extend(tempvec);
                                    temp.last_mut().unwrap().push(Objects::RenderObject(
                                        RenderObject::Literal(Literal {
                                            literal: String::from("]"),
                                        }),
                                    ));
                                }
                                fn have_argument(
                                    tempvec: Vec<Objects>,
                                    temp: &mut Vec<Vec<Objects>>,
                                    macrotype: MacroType,
                                ) {
                                    match (tempvec.get(1), tempvec.last().unwrap()) {
                                        (Some(object), lastobject) => {
                                            if object.to_owned() == Objects::Tokens(Tokens::Sad)
                                                && lastobject.to_owned()
                                                    == Objects::Tokens(Tokens::Happy)
                                            {
                                                temp.last_mut().unwrap().push(
                                                    Objects::RenderObject(RenderObject::Macro(
                                                        Macro {
                                                            typeofmacro: macrotype,
                                                            argument: Some(render_raw(
                                                                &(&tempvec[2..=tempvec.len() - 2])
                                                                    .to_vec(),
                                                            )),
                                                        },
                                                    )),
                                                );
                                            } else {
                                                unexpected_macro(temp, tempvec)
                                            }
                                        }
                                        (None, _) => unexpected_macro(temp, tempvec),
                                    }
                                }
                                fn dont_have_argument(
                                    tempvec: Vec<Objects>,
                                    temp: &mut Vec<Vec<Objects>>,
                                    macrotype: MacroType,
                                ) {
                                    match tempvec.get(1) {
                                        Some(_) => unexpected_macro(temp, tempvec),
                                        None => temp.last_mut().unwrap().push(
                                            Objects::RenderObject(RenderObject::Macro(Macro {
                                                argument: None,
                                                typeofmacro: macrotype,
                                            })),
                                        ),
                                    }
                                }
                            } //match macroClose
                            Tokens::LinkClose => {
                                let mut expect: Option<(usize, &Tokens)> = None; //fuck
                                let mut last_found_index: Option<usize> = None;
                                let mut index: usize = 0;
                                for token in &expected_token {
                                    if token == &Tokens::LinkOpen(false) {
                                        expect = Some((index.into(), &Tokens::LinkOpen(false)));
                                        break;
                                    } else if token == &Tokens::LinkOpen(true) {
                                        last_found_index = Some(index);
                                    }
                                    index += 1;
                                }
                                if expect == None && last_found_index != None {
                                    expect = Some((
                                        last_found_index.unwrap().into(),
                                        expected_token.get(last_found_index.unwrap()).unwrap(),
                                    ))
                                }
                                match expect {
                                    Some((i, _value)) => {
                                        let mut tempvec = temp.get(i).unwrap().to_owned();
                                        if i != expected_token.len() - 1 {
                                            for i in i..expected_token.len() {
                                                tempvec.push(
                                                    expected_token
                                                        .get(i)
                                                        .unwrap()
                                                        .to_owned()
                                                        .to_literal(),
                                                );
                                                tempvec.extend(temp.get(i).unwrap().to_owned());
                                            }
                                        }
                                        for _ in i..expected_token.len() {
                                            temp.pop();
                                            expected_token.pop();
                                        }
                                        println!("{:?}", tempvec);
                                        match tempvec.iter().enumerate().rfind(|(_index, x)| {
                                            x.to_owned() == &Objects::Tokens(Tokens::PipeLine)
                                        }) {
                                            Some((index, _token)) => {
                                                let (a, b) = tempvec.split_at(index);
                                                temp.last_mut().unwrap().push(
                                                    Objects::RenderObject(RenderObject::Link(
                                                        Link {
                                                            to: render_raw(&a.to_vec()),
                                                            typeoflink: get_link_type(&tempvec),
                                                            view: Some(b[1..].to_vec()),
                                                        },
                                                    )),
                                                );
                                            }
                                            None => {
                                                temp.last_mut().unwrap().push(
                                                    Objects::RenderObject(RenderObject::Link(
                                                        Link {
                                                            to: render_raw(&tempvec),
                                                            typeoflink: get_link_type(&tempvec),
                                                            view: None,
                                                        },
                                                    )),
                                                );
                                            }
                                        }
                                        fn get_link_type(tempvec: &Vec<Objects>) -> LinkType {
                                            if let Some(Objects::RenderObject(
                                                RenderObject::Literal(literal),
                                            )) = tempvec.first()
                                            {
                                                if literal
                                                    .literal
                                                    .to_lowercase()
                                                    .starts_with("file")
                                                    || literal
                                                        .literal
                                                        .to_lowercase()
                                                        .starts_with("파일")
                                                {
                                                    return LinkType::Picture;
                                                } else if literal
                                                    .literal
                                                    .to_lowercase()
                                                    .starts_with("category")
                                                    || literal
                                                        .literal
                                                        .to_lowercase()
                                                        .starts_with("분류")
                                                {
                                                    return LinkType::Cat;
                                                }
                                                return LinkType::Hyper;
                                            } else {
                                                return LinkType::Hyper;
                                            }
                                        }
                                    }
                                    None => {
                                        temp.last_mut().unwrap().push(Objects::RenderObject(
                                            RenderObject::Literal(Literal {
                                                literal: String::from("]]"),
                                            }),
                                        ));
                                    }
                                }
                            } //match LinkClose. 지금 이거에 약간 심각한 문제가 있는데 그거 해결하려면 파서 개발을 접고 싶어짐으로 딴거 할꺼임 라고 적는 동안 OS 텀이 돌았어. OS배울꺼임
                            Tokens::LinkOpen(_) => {
                                temp.push(Vec::new());
                                expected_token.push(Tokens::LinkOpen(false));
                            } //match macroClose
                            Tokens::Nop => {
                                let clone = temp.clone();
                                for i in 1..expected_token.len() {
                                    temp.first_mut().unwrap().push(
                                        expected_token.get(i).unwrap().to_owned().to_literal(),
                                    );
                                    temp.first_mut()
                                        .unwrap()
                                        .extend(clone.get(i).unwrap().to_owned()); //소유권은 참 이상해
                                }
                                for _ in 1..expected_token.len() {
                                    temp.pop();
                                    expected_token.pop();
                                }
                                break;
                            }
                            //몇몇 리터럴이 되면 안되는 토큰들 ~~생각해보니까 다 리터럴이 되면 안되긴 해~~ 그럼 왜 나머지 케이스로 안하냐고? 혹시 모르잖아~
                            Tokens::PipeLine => {
                                let expect = expected_token
                                    .iter()
                                    .enumerate()
                                    .rfind(|&(ref _i, &ref x)| *x == Tokens::LinkOpen(false)); //fuck
                                match expect {
                                    Some((i, _token)) => {
                                        pipelinecount += 1;
                                        temp.last_mut()
                                            .unwrap()
                                            .push(Objects::Tokens(Tokens::PipeLine));
                                        expected_token[i] = Tokens::LinkOpen(true);
                                    }
                                    None => {
                                        temp.last_mut().unwrap().push(Tokens::PipeLine.to_literal())
                                    }
                                }
                            },
                            Tokens::ShBoom => {
                                temp.push(Vec::new());
                                expected_token.push(Tokens::ShBoom);
                            },
                            Tokens::TripleClose => {
                                let expect = expected_token
                                    .iter()
                                    .enumerate()
                                    .find(|&(ref _i, &ref x)| *x == Tokens::MacroOpen); //fuck
                                match expect {
                                    Some((i, _value)) => {
                                        let mut tempvec = temp.get(i).unwrap().to_owned();
                                        if i != expected_token.len() - 1 {
                                            for i in i + 1..expected_token.len() {
                                                tempvec.push(
                                                    expected_token
                                                        .get(i)
                                                        .unwrap()
                                                        .to_owned()
                                                        .to_literal(),
                                                );
                                                tempvec.extend(temp.get(i).unwrap().to_owned());
                                            }
                                        }
                                        for _ in i..expected_token.len() {
                                            temp.pop();
                                            expected_token.pop();
                                        }
                                        if let Some(Objects::RenderObject(RenderObject::Literal(Literal { literal }))) = tempvec.first() {
                                            match literal.to_lowercase().as_str() {
                                                "wiki" | "folding" | "syntax" /*| "oiiaiioiiiaii" | "synTeX"*/ | "if" => { //여기서 if문은 따로 파서를 만들어서 (씨발) 돌아야 하지 않을까
                                                    temp.last_mut().unwrap().push(Objects::RenderObject(RenderObject::ShBoom(LifeCouldBeADream {objects:tempvec[2..].to_vec(), name:literal.to_owned(), typeof_life_could_be_adream: return_typeof_life_could_be_adream(literal.to_owned()) })));
                                                },
                                                _ => {
                                                    temp.last_mut().unwrap().push(Objects::RenderObject(RenderObject::NoWiki(NoWiki { objects: tempvec })));
                                                }
                                            }
                                            fn return_typeof_life_could_be_adream(name:String) -> TypeOfLifeCouldBeADream {
                                                match name.to_lowercase().as_str() {
                                                    "wiki" => return TypeOfLifeCouldBeADream::Wiki,
                                                    "folding" => return TypeOfLifeCouldBeADream::Folding,
                                                    "syntax" => return TypeOfLifeCouldBeADream::SynTeX, //sus
                                                    "if" => return TypeOfLifeCouldBeADream::If,
                                                    _ => {panic!("Parser Paniced because return type doesn't exist. but how??? If you see this message NOT IN THE SOURCE CODE, connect hazer24879@gmail.com
파서가 예상치 못한 오류를 일으켰습니다. hazer24879@gmail.com에 연락해주세요. 근데 진짜 예상치 못한 이유이긴 해
                                                    ")}
                                                }
                                            }
                                        } else {
                                            temp.last_mut().unwrap().push(Objects::RenderObject(
                                                RenderObject::Literal(Literal {
                                                    literal: String::from("}}}"),
                                                }),
                                            )); 
                                        }
                                        
                                    }
                                    None => {
                                        temp.last_mut().unwrap().push(Objects::RenderObject(
                                            RenderObject::Literal(Literal {
                                                literal: String::from("}}}"),
                                            }),
                                        ));
                                    }
                                }
                            }
                            Tokens::Sharp => temp
                                .last_mut()
                                .unwrap()
                                .push(Objects::Tokens(Tokens::Sharp)),
                            Tokens::Sad => {
                                temp.last_mut().unwrap().push(Objects::Tokens(Tokens::Sad))
                            }
                            Tokens::Happy => temp
                                .last_mut()
                                .unwrap()
                                .push(Objects::Tokens(Tokens::Happy)),
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
        match self.parsetemp.get(self.idx + 1) {
            Some(token) => return token.to_owned(),
            None => return Objects::Tokens(Tokens::Nop),
        }
    }
    fn get_before(&self, how_much: usize) -> Objects {
        if self.idx < how_much {
            return Objects::Tokens(Tokens::Nop.clone());
        }
        return self
            .parsetemp
            .get(self.idx - how_much)
            .unwrap_or(&Objects::Tokens(Tokens::Nop.clone()))
            .clone();
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum RenderObject {
    Heading(Heading),
    Literal(Literal),
    Macro(Macro),
    Link(Link),
    NoWiki(NoWiki),
    ShBoom(LifeCouldBeADream)
}
#[derive(Debug, PartialEq, Clone)]
pub enum Objects {
    RenderObject(RenderObject),
    Tokens(Tokens),
}
#[derive(Debug, PartialEq, Clone)]
pub struct Macro {
    argument: Option<String>,
    typeofmacro: MacroType,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Link {
    to: String,
    typeoflink: LinkType,
    view: Option<Vec<Objects>>,
}
#[derive(Debug, PartialEq, Clone)]
pub struct NoWiki {
    objects:Vec<Objects>
}
#[derive(Debug, PartialEq, Clone)]
pub struct LifeCouldBeADream {
    objects:Vec<Objects>,
    name:String,
    typeof_life_could_be_adream:TypeOfLifeCouldBeADream
}
impl NoWiki {
    pub fn get_objects(&self) -> Vec<Objects> {
        return self.objects.clone()
    }
}
impl Macro {
    pub fn getarg(&self) -> Option<String> {
        return self.argument.clone();
    }
    pub fn gettype(&self) -> MacroType {
        return self.typeofmacro.clone();
    }
}
#[derive(Debug, PartialEq, Clone)]
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
#[derive(Debug, PartialEq, Clone)]
pub enum TypeOfLifeCouldBeADream { //fuck~
    SynTeX, //sus
    If,
    Wiki,
    Folding
}
#[derive(Debug, PartialEq, Clone)]
pub enum LinkType {
    Hyper,
    Picture,
    Cat,                  //meow~
    IsThereSomeThingElse, //:D
}
#[derive(Debug, PartialEq, Clone)]
pub struct Heading {
    folded: bool,
    render_objects: Vec<Objects>,
    level: u8,
}
impl Heading {
    pub fn get_lvl(&self) -> u8 {
        return self.level;
    }
    pub fn get_render_objects(&self) -> Vec<Objects> {
        return self.render_objects.clone();
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Literal {
    pub literal: String,
}
#[derive(Debug, PartialEq, Clone)]
pub struct Main {
    render_objects: Vec<RenderObject>,
}
