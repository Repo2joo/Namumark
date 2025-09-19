pub fn parse_third(compiler:&mut Compiler, close:Expect) -> RenderObject {
    let result = RenderObject::NopNopNop;
    prepare_result(compiler, &mut result, &close);
    let namumarkresult:Vec<Objects> = Vec::new();
    while namumarker(compiler, &close, &mut result, namumarkresult) {}
  return result;
}
fn prepare_result (compiler:&mut Compiler, result:&mut result, close:&close) {
  match close {
    Expect::Bold => {
      *result = RenderObject::Bold {
        content:Vec::new();
      }
    },
    Expect::Itelic => {
      *result = Itelic {
        content:Vec::new();
      }
    }
    Expect::DelTidal => {
      *result = RenderObject::Bold {
        content:Vec::new()
      }
    },
    Expect::DelBar => {
      *result = RenderObject::Bold {
        content:Vec::new()
      }
    }
    Expect::UnderLine => {
      *result = RenderObject::Bold {
        content:Vec::new()
      }
    },
    Expect::Upper => {
      *result = RenderObject::Bold {
        content:Vec::new()
      }
    },
    Expect::Lower => {
      *result = RenderObject::Bold {
        content:Vec::new()
      }
    }
  }
}
fn namumarker (compiler:&mut Compiler, close:&Expect, result:&mut RenderObject) ->bool {
  match compiler.current() {
    Some(Char) => {
    
    },
    Some(RenderObject) => {
    
    },
    None => {
    
    }
  }
  if parsing_close() { false }
}
fn parsing_close() -> bool {
  
}
