#[macro_use]
extern crate nom;

mod diagram;
mod dot;
mod parser;

use diagram::*;
use parser::parse;
use std::io::Write;

pub fn render_dot<W: Write>(input: &str, target: &mut W) {
  let ast = parse(input).expect("Failed to parse UML input");
  let diagram = Diagram::from(ast);
  dot::render(&diagram, target).expect("Failed to render diagram");
}
