use std::{sync::{Arc, Mutex}, time::Instant};

use structs::Compiler;

use crate::structs::Objects;
mod renderobjs;
mod structs;
#[cfg(test)]
mod tests;

mod parser; //아 이거 복잡하다
fn main() {
    let teststr = "";

    let mut compiler = Compiler::from(teststr.to_owned());
    let start = Instant::now();
    compiler.parse();
    let es = start.elapsed();
    println!("parsed: {:#?}\nIn {:?}",compiler.array , es);
}
static mut CUSTOM_MACROS_NO_ARG: Option<Arc<Mutex<Vec<Objects>>>> = None;