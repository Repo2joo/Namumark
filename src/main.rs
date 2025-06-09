use std::time::Instant;

use structs::Compiler;
mod renderobjs;
mod structs;
#[cfg(test)]
mod tests;
mod traitt;

mod parser; //아 이거 복잡하다
fn main() {
    let teststr = "{{{#!wiki asdf}}}
ghjk";

    let mut compiler = Compiler::from(teststr.to_owned());
    let start = Instant::now();
    compiler.parse();
    let es = start.elapsed();
    println!("parsed: {:#?}\nIn {:?}", compiler.array, es);
}
