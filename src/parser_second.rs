use crate::{renderobjs::{Reference, RenderObject}, structs::{Compiler, Expect}};

pub fn parse_second (compiler: &mut Compiler, expect:Expect) -> RenderObject {
    let mut namumarkresult: Vec<Objects> = Vec::new();
    let mut result: RenderObject = RenderObject::NopNopNop;
    if matches!(RenderObject::Reference(_), expect);
    let mut close = close;

    while namumarker(compiler, &mut close, &mut namumarkresult, &mut result) {
        if compiler.lastrollbackindex.len() == 61 {
            panic!("문법 깊이 제한에 도달했습니다.")
        }
    }
    result
}
}
