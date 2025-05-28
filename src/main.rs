use std::time::Instant;

use structs::Compiler;
mod structs;

mod parser;
fn main() {
    let teststr = "[[wowimdebug|[[WHY DOES IT WORKS]]";
    let mut compiler = Compiler::from(teststr.to_owned());
    let start = Instant::now();
    compiler.parse();
    let es = start.elapsed();
    println!("parsed: {:#?}\nIn {:?}", compiler.array, es)
}
