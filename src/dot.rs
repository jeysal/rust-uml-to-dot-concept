extern crate dot;
extern crate itertools;

use super::diagram::*;
use std::borrow::Cow;
use std::boxed::Box;
use std::rc::Rc;

pub use self::dot::render;

type ClassNode<'a> = Rc<Class<'a>>;
type RelationEdge<'a> = Box<Relation<'a>>;

impl<'a> dot::Labeller<'a, ClassNode<'a>, RelationEdge<'a>> for Diagram<'a> {
  fn graph_id(&self) -> dot::Id<'a> {
    dot::Id::new("diagram").unwrap()
  }

  // classes

  fn node_id(&self, class: &ClassNode) -> dot::Id<'a> {
    dot::Id::new(format!("class_{}", class.name))
      .expect(format!("Invalid class name: '{}'", class.name).as_str())
  }

  fn node_label(&self, class: &ClassNode) -> dot::LabelText<'a> {
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

  fn node_shape(&self, _class: &ClassNode) -> Option<dot::LabelText<'a>> {
    Some(dot::LabelText::label("record"))
  }

  // relations

  fn edge_label(&self, rel: &RelationEdge<'a>) -> dot::LabelText<'a> {
    match rel.kind {
      Relationship::Association { attribute_name } => dot::LabelText::label(attribute_name),
      Relationship::Inheritance => dot::LabelText::label(""),
    }
  }

  fn edge_end_arrow(&self, rel: &RelationEdge) -> dot::Arrow {
    let Relation { ref kind, .. } = **rel;
    dot::Arrow::from_arrow(match kind {
      Relationship::Association { .. } => dot::ArrowShape::Vee(dot::Side::Both),
      Relationship::Inheritance => dot::ArrowShape::Normal(dot::Fill::Open, dot::Side::Both),
    })
  }
}

impl<'a> dot::GraphWalk<'a, ClassNode<'a>, RelationEdge<'a>> for Diagram<'a> {
  fn nodes(&self) -> dot::Nodes<'a, ClassNode> {
    Cow::Borrowed(&self.classes)
  }

  fn edges(&self) -> dot::Edges<'a, RelationEdge> {
    Cow::Borrowed(&self.relations)
  }

  fn source(&self, rel: &RelationEdge<'a>) -> ClassNode<'a> {
    Rc::clone(&rel.source)
  }

  fn target(&self, rel: &RelationEdge<'a>) -> ClassNode<'a> {
    Rc::clone(&rel.target)
  }
}
