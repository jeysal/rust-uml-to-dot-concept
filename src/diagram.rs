use super::parser;
use std::boxed::Box;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::From;
use std::rc::Rc;

#[derive(Clone)]
pub struct Class<'a> {
  pub name: &'a str,
  pub attributes: Vec<Attribute<'a>>,
}
#[derive(Clone)]
pub struct Attribute<'a> {
  pub name: &'a str,
  pub typ: &'a str,
}

#[derive(Clone)]
pub struct Relation<'a> {
  pub kind: Relationship<'a>,
  pub source: Rc<Class<'a>>,
  pub target: Rc<Class<'a>>,
}
#[derive(Clone)]
pub enum Relationship<'a> {
  Association { attribute_name: &'a str },
  Inheritance,
}

pub struct Diagram<'a> {
  pub classes: Vec<Rc<Class<'a>>>,
  pub relations: Vec<Box<Relation<'a>>>,
}

impl<'a> From<parser::Diagram<'a>> for Diagram<'a> {
  fn from(ast: parser::Diagram<'a>) -> Diagram<'a> {
    // 1) Create a sets of class and superclass names
    // to establish an index of types we know
    // as opposed to types we assume must be primitives.
    let declared_class_names: HashSet<&str> = ast
      .classes
      .iter()
      .map(|parser::Class { name, .. }| *name)
      .collect();
    let superclass_names: HashSet<&str> = ast
      .classes
      .iter()
      .filter_map(|parser::Class { superclass, .. }| *superclass)
      .collect();
    let known_class_names: HashSet<&str> = declared_class_names
      .union(&superclass_names)
      .map(|name| name.clone())
      .collect();

    // 2) Create classes
    let class_map: HashMap<&str, Rc<Class>> = superclass_names
      .iter()
      .map(
        // 2.1) empty for classes only known as superclasses
        |name| {
          (
            *name,
            Rc::new(Class {
              name,
              attributes: Vec::new(),
            }),
          )
        },
      )
      .chain(ast.classes.iter().map(
        |parser::Class {
           name: class_name,
           attributes,
           ..
         }| {
          (
            *class_name,
            // 2.2) with their primitive attributes for declared classes
            Rc::new(Class {
              name: class_name,
              attributes: attributes
                .iter()
                .filter_map(
                  |parser::Attribute {
                     name: attribute_name,
                     typ,
                   }| {
                    if known_class_names.contains(typ) {
                      None // An association relation will be added instead
                    } else {
                      Some(Attribute {
                        name: attribute_name,
                        typ,
                      })
                    }
                  },
                )
                .collect(),
            }),
          )
        },
      ))
      .collect();

    // 3) Create relations
    let relations: Vec<Box<Relation>> = ast
      .classes
      .iter()
      .flat_map(
        |parser::Class {
           name: class_name,
           superclass,
           attributes,
         }| {
          let source = class_map.get(class_name).unwrap();
          attributes
            .iter()
            .filter_map(
              // 3.1) Association relations for attributes of types we know
              |parser::Attribute {
                 typ,
                 name: attribute_name,
               }| {
                if let Some(target) = class_map.get(typ) {
                  Some(Box::new(Relation {
                    kind: Relationship::Association { attribute_name },
                    source: Rc::clone(&source),
                    target: Rc::clone(&target),
                  }))
                } else {
                  None
                }
              },
            )
            .chain(superclass.map(
              // 3.2) Inheritance relations for superclasses
              |superclass| {
                let target = class_map.get(superclass).unwrap();
                Box::new(Relation {
                  kind: Relationship::Inheritance,
                  source: Rc::clone(&source),
                  target: Rc::clone(&target),
                })
              },
            ))
            .collect::<Vec<Box<Relation>>>()
        },
      )
      .collect();

    // 4. Collect the classes from the class_map into a plain vector
    let classes: Vec<Rc<Class>> = class_map.into_iter().map(|(_, class)| class).collect();

    Diagram { classes, relations }
  }
}
