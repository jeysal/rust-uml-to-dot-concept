extern crate dot;
extern crate itertools;

use super::diagram::*;
use std::borrow::Cow;
use std::boxed::Box;
use std::io;
use std::rc::Rc;

pub fn render<W: io::Write>(diagram: &Diagram, target: &mut W) -> Result<(), io::Error> {
  dot::render(diagram, target)
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
