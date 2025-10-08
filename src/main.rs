//! A Namumark parser<br />
//! To start, make [Compiler] with [Compiler::from]<br />
//! Use [Compiler::parse] to parse.<br />
//! # Custom Macro
//! You can add Custon Macro using [Compiler::add_custom_macros]
//! # Example
//! ```rust
//!fn main() {
//!    let compiler = Compiler::from("Hello, Namumark!");
//!    println!("{:#?}", compiler.array);
//!}
//!```
use structs::Compiler;

pub mod renderobjs;
pub mod structs;
#[cfg(test)]
mod tests;

mod parse_third;
mod parser_first;
fn main() {
  let teststr = "[datetime]adsf";
  let mut compiler = Compiler::from(teststr.to_owned());
  compiler.parse();
  println!("{:#?}", compiler.array)
}
