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
    Some(Objects::Char(ch)) => {
      if parsing_close() { false }
      if compiler.peak("'''") {
        compiler.index += 3;
        compiler.expexted.push(Expect::Bold);
        thisparsing = parse_third(compiler, Expect::Bold);
      }
      if compiler.peak("''") {
        compiler.index += 2;
        compiler.expexted.push(Expect::Itelic);
        thisparsing = parse_third(compiler, Expect::Itelic);
      }
      if compiler.peak("~~") {
        compiler.index += 2;
        compiler.expexted.push(Expect::DelTidal);
        thisparsing = parse_third(compiler, DelTidal);
      }
      if compiler.peak("--") {
        compiler.index += 2;
        compiler.expexted.push(Expect::DelBar);
        thisparsing = parse_third(compiler, Expect::DelBar);
      }
      if compiler.peak("__") {
        compiler.index += 2;
        compiler.expexted.push(Expect::Bold);
        thisparsing = parse_third(compiler, Expect::Bold);
      }
      if compiler.peak("^^") {
        compiler.index += 2;
        compiler.expexted.push(Expect::Upper);
        thisparsing = parse_third(compiler, Expect::Upper);
      }
      if compiler.peak(",,") {
        compiler.index += 2;
        compiler.expexted.push(Expect::Lower);
        thisparsing = parse_third(compiler, Expect::Lower);
      }
namumarkresult.push(Objects::Char(ch))
    },
    Some(Objects::RenderObject(rdobj)) => {
namumarkresult.push(Objects::RenderObject::(rdobj));
    },
    None => {
      return a_whole_my_vec()
    }
  }
}
fn parsing_close() -> bool {
  if compiler.peak("'''") {
    if *close == Expect::Bold {
      if let Objects::RenderObject(RenderObject::Bold(bd)) = *result {
      bd.content = namumarkresult.to_vec();
      }
      return false;
    }
    if compiler.expected.contains(Expect::Bold) {
      *result = RenderObject(RenderObject::Earlyparse((Expect::Bold, a_whole_my_vec(Expect::Bold, namumarkresult.to_vec()))))
      return false;
    }
    return true;
  }
  if compiler.peak("''") {
    if *close == Expect::Bold {
      if let Objects::RenderObject(RenderObject::Bold(bd)) = *result {
      bd.content = namumarkresult.to_vec();
      }
      return false;
    }
    if compiler.expected.contains(Expect::Bold) {
      *result = RenderObject(RenderObject::Earlyparse((Expect::Bold, a_whole_my_vec(Expect::Bold, namumarkresult.to_vec()))))
      return false;
    }
    return true;
  }
  if compiler.peak("~~") {
    if *close == Expect::Bold {
      if let Objects::RenderObject(RenderObject::Bold(bd)) = *result {
      bd.content = namumarkresult.to_vec();
      }
      return false;
    }
    if compiler.expected.contains(Expect::Bold) {
      *result = RenderObject(RenderObject::Earlyparse((Expect::Bold, a_whole_my_vec(Expect::Bold, namumarkresult.to_vec()))))
      return false;
    }
    return true;
  }
  if compiler.peak("--") {
    if *close == Expect::Bold {
      if let Objects::RenderObject(RenderObject::Bold(bd)) = *result {
      bd.content = namumarkresult.to_vec();
      }
      return false;
    }
    if compiler.expected.contains(Expect::Bold) {
      *result = RenderObject(RenderObject::Earlyparse((Expect::Bold, a_whole_my_vec(Expect::Bold, namumarkresult.to_vec()))))
      return false;
    }
    return true;
  }
  if compiler.peak("__") {
    if *close == Expect::Bold {
      if let Objects::RenderObject(RenderObject::Bold(bd)) = *result {
      bd.content = namumarkresult.to_vec();
      }
      return false;
    }
    if compiler.expected.contains(Expect::Bold) {
      *result = RenderObject(RenderObject::Earlyparse((Expect::Bold, a_whole_my_vec(Expect::Bold, namumarkresult.to_vec()))))
      return false;
    }
    return true;
  }
  if compiler.peak(",,") {
    if *close == Expect::Bold {
      if let Objects::RenderObject(RenderObject::Bold(bd)) = *result {
      bd.content = namumarkresult.to_vec();
      }
      return false;
    }
    if compiler.expected.contains(Expect::Bold) {
      *result = RenderObject(RenderObject::Earlyparse((Expect::Bold, a_whole_my_vec(Expect::Bold, namumarkresult.to_vec()))))
      return false;
    }
    return true;
  }
  if compiler.peak("^^") {
    if *close == Expect::Bold {
      if let Objects::RenderObject(RenderObject::Bold(bd)) = *result {
      bd.content = namumarkresult.to_vec();
      }
      return false;
    }
    if compiler.expected.contains(Expect::Bold) {
      *result = RenderObject(RenderObject::Earlyparse((Expect::Bold, a_whole_my_vec(Expect::Bold, namumarkresult.to_vec()))))
      return false;
    }
    return true;
  }
}
fn a_whole_my_vec() -> {
  match close {
    Expect::Bold => {
      let rst = slices
    }
    Expect::Itelic => {
    
    }
    Expect::DelTidal => {
    
    }
    Expect::DelBar => {
    
    }
    Expect::UnderLine => {
    
    }
    Expect::Upper => {
    
    }
    Expect::Lower => {
    
    }
  }
}
