mod common;

use common::run;

#[test]
fn class() {
  run("class")
}

#[test]
fn superclass() {
  run("superclass")
}

#[test]
fn unknown_superclass() {
  run("unknown_superclass")
}

#[test]
fn attr() {
  run("attr")
}

#[test]
fn assoc() {
  run("assoc")
}

#[test]
fn example() {
  run("example")
}
