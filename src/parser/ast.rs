#[derive(Debug, PartialEq)]
pub struct Diagram<'a> {
  pub classes: Vec<Class<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Class<'a> {
  pub name: &'a str,
  pub superclass: Option<&'a str>,
  pub attributes: Vec<Attribute<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Attribute<'a> {
  pub name: &'a str,
  pub typ: &'a str,
}
