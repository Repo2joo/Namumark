use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

use structs::Compiler;

use crate::structs::Objects;
mod renderobjs;
mod structs;
#[cfg(test)]
mod tests;

mod parser; //아 이거 복잡하다
fn main() {
    let teststr = "{{{
# redirect asdf}}}";
    let mut compiler = Compiler::from(teststr.to_owned());
    let start = Instant::now();
    compiler.parse();
    let es = start.elapsed();
    println!("In {:?}\nfixed:{:?}\nredirect:{:?}\n parsed: {:#?}\n", es, compiler.fixed_comments, compiler.redirect, compiler.array);
}
