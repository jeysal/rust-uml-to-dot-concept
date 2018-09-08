mod diagram;
mod dot;

use diagram::*;
use std::boxed::Box;
use std::io;
use std::rc::Rc;

pub fn render_dot<W: io::Write>(_input: &[u8], target: &mut W) {
  let class_a = Rc::new(Class {
    name: String::from("A"),
    attributes: vec![
      Attribute {
        name: String::from("x"),
        typ: String::from("X"),
      },
      Attribute {
        name: String::from("y"),
        typ: String::from("Y"),
      },
    ],
  });

  let class_b = Rc::new(Class {
    name: String::from("B"),
    attributes: vec![],
  });

  let assoc_a_b = Box::new(Relation {
    kind: Relationship::Association,
    source: Rc::clone(&class_a),
    target: Rc::clone(&class_b),
  });
  let inherit_b_a = Box::new(Relation {
    kind: Relationship::Inheritance,
    source: Rc::clone(&class_b),
    target: Rc::clone(&class_a),
  });

  dot::render(
    &Diagram {
      classes: &vec![class_a, class_b],
      relations: &vec![assoc_a_b, inherit_b_a],
    },
    target,
  ).expect("Failed to render diagram");
}
