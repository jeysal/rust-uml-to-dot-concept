use std::boxed::Box;
use std::rc::Rc;

pub struct Class {
  pub name: String,
  pub attributes: Vec<Attribute>,
}
pub struct Attribute {
  pub name: String,
  pub typ: String,
}

#[derive(Clone)]
pub struct Relation {
  pub kind: Relationship,
  pub source: Rc<Class>,
  pub target: Rc<Class>,
}
#[derive(Clone)]
pub enum Relationship {
  Association,
  Inheritance,
}

pub struct Diagram<'a> {
  pub classes: &'a Vec<Rc<Class>>,
  pub relations: &'a Vec<Box<Relation>>,
}
