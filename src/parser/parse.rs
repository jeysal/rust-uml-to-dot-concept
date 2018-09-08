use super::ast::*;
use nom;
use nom::types::CompleteStr as In;
use nom::{alphanumeric1, space0, space1};

pub fn parse(input: &str) -> Result<Diagram, nom::Err<In>> {
  match diagram(In(input)) {
    Ok((In(""), diagram)) => Ok(diagram),
    Ok((rest, _)) => Err(nom::Err::Error(nom::Context::Code(
      rest,
      nom::ErrorKind::Eof,
    ))),
    Err(err) => Err(err),
  }
}

named!(
  diagram<In, Diagram>,
  map!(many0!(class), |classes| Diagram { classes })
);

named!(
  class<In, Class>,
  map!(
    preceded!(
      delimited!(space0, tag!("class"), space1),
      pair!(ws!(alphanumeric1), class_body)
    ),
    |(In(name), attributes)| Class {
      name,
      superclass: "", // TODO
      attributes,
    }
  )
);
named!(
  class_body<In, Vec<Attribute>>,
  delimited!(
    ws!(char!('{')),
    attributes,
    ws!(char!('}'))
  )
);

named!(
  attributes<In, Vec<Attribute>>,
  terminated!(
    separated_list!(ws!(char!(';')), attribute),
    opt!(ws!(char!(';')))
  )
);
named!(
  attribute<In, Attribute>,
  map!(
    separated_pair!(
      ws!(alphanumeric1),
      char!(':'),
      ws!(alphanumeric1)
    ),
    |(In(name), In(typ))| Attribute { name, typ }
  )
);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn attr_normal() {
    assert_eq!(
      attribute(In("a: A")),
      Ok((
        In(""),
        Attribute {
          name: "a",
          typ: "A",
        }
      ))
    )
  }

  #[test]
  fn attr_ws() {
    assert_eq!(
      attribute(In(" a : A ")),
      Ok((
        In(""),
        Attribute {
          name: "a",
          typ: "A",
        }
      ))
    )
  }

  #[test]
  fn attrs_normal() {
    assert_eq!(
      attributes(In("a: A; b: B")),
      Ok((
        In(""),
        vec![
          Attribute {
            name: "a",
            typ: "A",
          },
          Attribute {
            name: "b",
            typ: "B",
          },
        ]
      ))
    )
  }

  #[test]
  fn attrs_trailing_semi() {
    assert_eq!(
      attributes(In("a: A; b: B;")),
      Ok((
        In(""),
        vec![
          Attribute {
            name: "a",
            typ: "A",
          },
          Attribute {
            name: "b",
            typ: "B",
          },
        ]
      ))
    )
  }

  #[test]
  fn attrs_empty() {
    assert_eq!(attributes(In("")), Ok((In(""), Vec::new())))
  }

  #[test]
  fn attrs_ws() {
    assert_eq!(
      attributes(In(" a : A ; b : B ")),
      Ok((
        In(""),
        vec![
          Attribute {
            name: "a",
            typ: "A",
          },
          Attribute {
            name: "b",
            typ: "B",
          },
        ]
      ))
    )
  }

  #[test]
  fn class_body_normal() {
    assert_eq!(
      class_body(In("{ a: A; b: B }")),
      Ok((
        In(""),
        vec![
          Attribute {
            name: "a",
            typ: "A",
          },
          Attribute {
            name: "b",
            typ: "B",
          },
        ]
      ))
    )
  }

  #[test]
  fn class_body_ws() {
    assert_eq!(
      class_body(In(" { a : A ; b : B } ")),
      Ok((
        In(""),
        vec![
          Attribute {
            name: "a",
            typ: "A",
          },
          Attribute {
            name: "b",
            typ: "B",
          },
        ]
      ))
    )
  }

  #[test]
  fn class_normal() {
    assert_eq!(
      class(In("class A { a: A; b: B }")),
      Ok((
        In(""),
        Class {
          name: "A",
          superclass: "",
          attributes: vec![
            Attribute {
              name: "a",
              typ: "A",
            },
            Attribute {
              name: "b",
              typ: "B",
            },
          ],
        }
      ))
    )
  }

  #[test]
  fn class_ws() {
    assert_eq!(
      class(In(" class A { a : A ; b : B } ")),
      Ok((
        In(""),
        Class {
          name: "A",
          superclass: "",
          attributes: vec![
            Attribute {
              name: "a",
              typ: "A",
            },
            Attribute {
              name: "b",
              typ: "B",
            },
          ],
        }
      ))
    )
  }

  #[test]
  fn class_keyword_nows() {
    assert!(class(In(" classA { a : A ; b : B } ")).is_err())
  }

  #[test]
  fn parse_normal() {
    assert_eq!(
      parse("class A { a: A; b: B } class B { a: A }"),
      Ok(Diagram {
        classes: vec![
          Class {
            name: "A",
            superclass: "",
            attributes: vec![
              Attribute {
                name: "a",
                typ: "A",
              },
              Attribute {
                name: "b",
                typ: "B",
              },
            ],
          },
          Class {
            name: "B",
            superclass: "",
            attributes: vec![Attribute {
              name: "a",
              typ: "A",
            }],
          },
        ],
      })
    )
  }

  #[test]
  fn parse_nows() {
    assert_eq!(
      parse("class A{a:A;b:B}class B{a:A}"),
      Ok(Diagram {
        classes: vec![
          Class {
            name: "A",
            superclass: "",
            attributes: vec![
              Attribute {
                name: "a",
                typ: "A",
              },
              Attribute {
                name: "b",
                typ: "B",
              },
            ],
          },
          Class {
            name: "B",
            superclass: "",
            attributes: vec![Attribute {
              name: "a",
              typ: "A",
            }],
          },
        ],
      })
    )
  }

  #[test]
  fn parse_empty() {
    assert_eq!(
      parse(""),
      Ok(Diagram {
        classes: Vec::new()
      })
    )
  }

  #[test]
  fn invalid_input() {
    assert!(parse("asdf").is_err())
  }

  #[test]
  fn extraneous_input() {
    assert!(parse("class A {} asdf").is_err())
  }
}
