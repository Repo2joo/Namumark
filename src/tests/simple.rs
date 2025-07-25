//test 하는 이유
//복잡한 파서 구조상 로직이 시도때도 없이 바뀌는데 그걸 하나하나 테스트 하기는 어렵기 때문.
//테스트에서 오차가 일어나면 바로 바꾸기 위해서
use crate::{
    parser::slices,
    renderobjs::{Link, LinkType, NamuTriple, RenderObject},
    structs::{Compiler, Objects},
};
#[test]
fn 링크() {
    let mut compiler = Compiler::from(String::from("[[테스트케이스]]"));
    compiler.parse();
    assert_eq!(
        compiler.array,
        vec![Objects::RenderObject(RenderObject::Link(Link {
            to: String::from("테스트케이스"),
            show: Vec::new(),
            link_type: LinkType::Hyper
        }))]
    )
}
#[test]
fn 링크_안닫힘() {
    let mut compiler = Compiler::from(String::from("[[테스트케이스"));
    compiler.parse();
    assert_eq!(
        compiler.array,
        vec![
            Objects::Char('['),
            Objects::Char('['),
            Objects::Char('테'),
            Objects::Char('스'),
            Objects::Char('트'),
            Objects::Char('케'),
            Objects::Char('이'),
            Objects::Char('스')
        ]
    )
}
#[test]
fn 표시링크() {
    let mut compiler = Compiler::from(String::from("[[테스트케이스|스이케트스테]]"));
    compiler.parse();
    assert_eq!(
        compiler.array,
        vec![Objects::RenderObject(RenderObject::Link(Link {
            to: String::from("테스트케이스"),
            show: vec![
                Objects::Char('스'),
                Objects::Char('이'),
                Objects::Char('케'),
                Objects::Char('트'),
                Objects::Char('스'),
                Objects::Char('테')
            ],
            link_type: LinkType::Hyper
        }))]
    )
}
#[test]
fn 표시링크_안닫힘() {
    let mut compiler = Compiler::from(String::from("[[테스트케이스|뀨"));
    compiler.parse();
    assert_eq!(
        compiler.array,
        vec![
            Objects::Char('['),
            Objects::Char('['),
            Objects::Char('테'),
            Objects::Char('스'),
            Objects::Char('트'),
            Objects::Char('케'),
            Objects::Char('이'),
            Objects::Char('스'),
            Objects::Char('|'),
            Objects::Char('뀨')
        ]
    )
}
#[test]
fn 트리플위키() {
    let mut compiler = Compiler::from(String::from(
        "{{{#!WiKi attribute
content}}}",
    ));
    compiler.parse();
    assert_eq!(
        compiler.array,
        vec![Objects::RenderObject(RenderObject::NamuTriple(
            NamuTriple {
                triplename: "WiKi".to_string(),
                attr: Some("attribute".to_string()),
                content: Some(vec![
                    Objects::Char('c'),
                    Objects::Char('o'),
                    Objects::Char('n'),
                    Objects::Char('t'),
                    Objects::Char('e'),
                    Objects::Char('n'),
                    Objects::Char('t'),
                ])
            }
        ))]
    )
}
#[test]
fn 트리플위키_안닫힘() {
    let mut compiler = Compiler::from(String::from(
        "{{{#!WiKi attribute
content",
    ));
    compiler.parse();
    assert_eq!(
        compiler.array,
        slices("{{{#!WiKi attribute\ncontent".to_string())
    )
}
#[test]
fn 트리플위키_속성에_삼중괄있음() {
    //이게 놀랍게도 원작고증이라는...(씨ㅂ)
    let mut compiler = Compiler::from(String::from(
        "{{{#!WiKi attribute}}}
content}}}",
    ));
    compiler.parse();
    assert_eq!(
        compiler.array,
        vec![Objects::RenderObject(RenderObject::NamuTriple(
            NamuTriple {
                attr: Some(String::from("attribute}}}")),
                content: Some(slices(String::from("content"))),
                triplename: String::from("WiKi")
            }
        ))]
    )
}
#[test]
fn 트리플위키_속성에_삼중괄있음2() {
    //이게 놀랍게도 원작고증이라는...(씨ㅂ)
    let mut compiler = Compiler::from(String::from(
        "{{{#!WiKi attribute{{{}}}
content}}}",
    ));
    compiler.parse();
    assert_eq!(
        compiler.array,
        vec![Objects::RenderObject(RenderObject::NamuTriple(
            NamuTriple {
                attr: Some(String::from("attribute{{{}}}")),
                content: Some(slices(String::from("content"))),
                triplename: String::from("WiKi")
            }
        ))]
    )
}
#[test]
fn 트리플위키_속성에_삼중괄있음_미완성() {
    //이게 놀랍게도 원작고증이라는...(씨ㅂ)
    let mut compiler = Compiler::from(String::from(
        "{{{#!WiKi attribute{{{}}}
content",
    ));
    compiler.parse();
    let mut vect = slices("{{{#!WiKi attribute".to_owned());
    vect.push(Objects::RenderObject(RenderObject::Literal(String::new())));
    vect.extend(slices("\ncontent".to_owned()));
    assert_eq!(compiler.array, vect)
}
#[test]
fn 파싱_우선순위() {
    //이게 놀랍게도 원작고증이라는...(씨ㅂ)
    let mut compiler = Compiler::from(String::from(
        "{{{#!wiki attr
content[[link|here}}}]]",
    ));
    compiler.parse();
    let mut vect = vec![Objects::RenderObject(RenderObject::NamuTriple(
        NamuTriple {
            attr: Some(String::from("attr")),
            content: Some(slices("content[[link|here".to_string())),
            triplename: String::from("wiki"),
        },
    ))];
    vect.extend_from_slice(&slices("]]".to_string()));
    assert_eq!(compiler.array, vect)
}
#[test]
fn 트리플_미완성_개행없이() {
    //이게 놀랍게도 원작고증이라는...(씨ㅂ)
    let mut compiler = Compiler::from(String::from("{{{#!wiki attr}}}"));
    compiler.parse();
    let vect = vec![Objects::RenderObject(RenderObject::Literal(String::from(
        "#!wiki attr",
    )))];
    assert_eq!(compiler.array, vect)
}
#[test]
fn 트리플_미완성_개행있이() {
    //이게 놀랍게도 원작고증이라는...(씨ㅂ)
    let mut compiler = Compiler::from(String::from(
        "{{{#!wiki attr}}}
",
    ));
    compiler.parse();
    let vect = slices("{{{#!wiki attr}}}\n".to_owned());
    assert_eq!(compiler.array, vect)
}
