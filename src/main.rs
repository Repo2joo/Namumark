//!간단한 사용법을 알려드리겠습니다
//![`structs::Compiler`]의 from 메소드로 컴파일러 객채를 만드세요.
//!<code>Compiler::from("파싱할 문자열")</code>
//!
//!parse()메소드로 파싱하세요.
//!
//!<code>Compiler::from("파싱할 문자열").parse()</code>
//!
//!컴파일러 객채의 array 값이 파싱된 값입니다.
//!
//!array에 있는 값들을 이해하고 싶으시다면 [`renderobjs`] 참조바랍니다
//!
//!러스트 처음이신분은 문서 볼 때 impl 어쩌고가 많은데 Trait Implementations, Auto Trait Implementations, Blanket Implementations는 무시하세요. 저는 이거 만들면서 트레잇 구현한적 없습니다.(derive 트레잇은 메크로로 구현하긴 했지만 별 쓸모 없으니까) 그냥 Implmentations는 참고 하여도 좋습니다.
//!
//!러스트 잘하시는 분 있으면 크레이트에서만 쓰는건 가릴 수 있나 싶은데.
//!아 그리고 테스트케이스 작성은 언제나 환영입니다.
use std::time::Instant;

use structs::Compiler;

pub mod renderobjs;
pub mod structs;
#[cfg(test)]
mod tests;

mod parser_first; //아 이거 복잡하다
mod parser_second;

//이렇게 문서화 하면 나폴리탄 괴담같음...

//처음이신분은
//Trait Implementations, Auto Trait Implementations, Blanket Implementations는 무시하세요.
//그 impl들은 이 크레이트에서 직접 구현된 적이 없습니다.
//그냥 Implmentations는 참고 하여도 좋습니다. 모든 일반 impl들은 믿을 수 있는 사람(?)이 구현한것입니다.

fn main() {
  let teststr = "{{{+1 asdf}}}";
  let mut compiler = Compiler::from(teststr.to_owned());
  compiler.parse();
  println!("{:#?}", compiler.array)
}
