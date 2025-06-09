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
