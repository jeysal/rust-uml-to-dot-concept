extern crate uml;

use self::uml::render_dot;
use std::collections::HashSet;
use std::env::var;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn run(name: &str) {
  let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  let test_resources_dir = root_dir.join(format!("resources/tests/{}", name));
  let in_file = test_resources_dir.join("in.uml");
  let out_file = test_resources_dir.join("out.dot");

  let mut input = String::new();
  File::open(in_file)
    .expect("Cannot open infile")
    .read_to_string(&mut input)
    .expect("Cannot read infile");

  let mut expected = String::new();
  File::open(out_file)
    .expect("Cannot open outfile")
    .read_to_string(&mut expected)
    .expect("Cannot read outfile");

  let mut actual_bytes = Vec::new();
  render_dot(&input, &mut actual_bytes);
  let actual = String::from_utf8(actual_bytes).expect("Rendered DOT is invalid UTF-8");

  if var("DEBUG").is_ok() {
    assert_eq!(expected, actual);
  } else {
    // The DOT output is not fully deterministic.
    // Asserting that actual and expected have the same lines (regardless of order)
    // is the best we can do with reasonable effort.
    let expected_lines: HashSet<&str> = expected.lines().collect();
    let actual_lines: HashSet<&str> = actual.lines().collect();
    assert_eq!(actual_lines, expected_lines)
  }
}
