use std::time::Instant;

use structs::Compiler;
mod structs;

mod parser;
fn main() {
    let teststr = "[[wowimdebug|Not debug, debuff:skull:]]";
    let mut compiler = Compiler::from(teststr.to_owned());
    let start = Instant::now();
    compiler.parse();
    let es = start.elapsed();
    println!("parsed: {:?}\nIn {:?}", compiler.array, es)
}
