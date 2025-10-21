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

pub mod renderobjs;
pub mod structs;
#[cfg(test)]
mod tests;

mod parse_third;
mod parser_first;
