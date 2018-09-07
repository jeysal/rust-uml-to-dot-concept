extern crate dot;
extern crate itertools;

use std::borrow::Cow;
use std::boxed::Box;
use std::rc::Rc;

struct Class {
  name: String,
  attributes: Vec<Attribute>,
}
struct Attribute {
  name: String,
  typ: String,
}

#[derive(Clone)]
struct Relation {
  kind: Relationship,
  source: Rc<Class>,
  target: Rc<Class>,
}
#[derive(Clone)]
enum Relationship {
  Association,
  Inheritance,
}

struct Diagram<'a> {
  classes: &'a Vec<Rc<Class>>,
  relations: &'a Vec<Box<Relation>>,
}

impl<'a> dot::Labeller<'a, Rc<Class>, Box<Relation>> for Diagram<'a> {
  fn graph_id(&self) -> dot::Id<'a> {
    dot::Id::new("diagram").unwrap()
  }

  // classes

  fn node_id(&self, class: &Rc<Class>) -> dot::Id<'a> {
    dot::Id::new(format!("class_{}", class.name))
      .expect(format!("Invalid class name: '{}'", class.name).as_str())
  }

  fn node_label(&self, class: &Rc<Class>) -> dot::LabelText<'a> {
    let Class {
      ref name,
      ref attributes,
    } = **class;

    let attributes = attributes
      .iter()
      .map(|Attribute { name, typ }| format!("{}: {}", name, typ));
    let attributes = itertools::join(attributes, "\n");

    dot::LabelText::label(format!("{{ {} | {} }}", name, attributes))
  }

  fn node_shape(&self, _class: &Rc<Class>) -> Option<dot::LabelText<'a>> {
    Some(dot::LabelText::label("record"))
  }

  // relations

  fn edge_end_arrow(&self, rel: &Box<Relation>) -> dot::Arrow {
    let Relation { ref kind, .. } = **rel;
    dot::Arrow::from_arrow(match kind {
      Relationship::Association => dot::ArrowShape::Vee(dot::Side::Both),
      Relationship::Inheritance => dot::ArrowShape::Normal(dot::Fill::Open, dot::Side::Both),
    })
  }
}

impl<'a> dot::GraphWalk<'a, Rc<Class>, Box<Relation>> for Diagram<'a> {
  fn nodes(&self) -> dot::Nodes<'a, Rc<Class>> {
    Cow::Borrowed(&self.classes)
  }

  fn edges(&self) -> dot::Edges<'a, Box<Relation>> {
    Cow::Borrowed(self.relations)
  }

  fn source(&self, rel: &Box<Relation>) -> Rc<Class> {
    Rc::clone(&rel.source)
  }

  fn target(&self, rel: &Box<Relation>) -> Rc<Class> {
    Rc::clone(&rel.target)
  }
}

fn main() {
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
    &mut std::io::stdout(),
  ).expect("Failed to render diagram");
}
