extern crate uml;

use std::io::stdout;

fn main() {
  uml::render_dot(&[], &mut stdout());
}
