extern crate uml;

use std::io::stdin;
use std::io::stdout;
use std::io::Read;

fn main() {
  let mut input_bytes = Vec::new();
  stdin()
    .read_to_end(&mut input_bytes)
    .expect("Failed to read stdin");
  let input = String::from_utf8(input_bytes).expect("Input is not valid UTF-8");

  uml::render_dot(input.as_str(), &mut stdout());
}
