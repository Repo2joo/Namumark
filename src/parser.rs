use crate::structs::{Compiler, Expect, Link, LinkType, Objects, RenderObject};

pub fn parse_first(compiler: &mut Compiler, close: Expect) -> RenderObject {
    let mut namumarkresult:Vec<Objects>;
    let mut result:RenderObject;
    RenderObject::NopForLink
}
fn parsing_listener(compiler: &mut Compiler, close: Expect, namumarkresult: Vec<Objects>, result:&mut RenderObject) {
    match close {
        Expect::None => {
            *result = RenderObject::NopNopNop;
            *compiler.array = namumarkresult},
        Expect::Link => *result = RenderObject::Link(Link {
            to: String::new(),
            show: Some(Vec::new()),
            link_type: LinkType::Hyper,
        }),
        Expect::Link2 => panic!(),
    }
}