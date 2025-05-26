use std::time::Instant;

use structs::Compiler;
mod structs;

mod parser;
fn main() {
    let teststr = "wowimperson";
    let mut compiler = Compiler::from(teststr.to_owned());
    let start = Instant::now();
    compiler.parse();
    println!("parsed: {:?}\nIn {:?}", compiler.array, start.elapsed())
}
