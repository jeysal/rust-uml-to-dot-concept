# rust-uml-to-dot-concept

Small experiment / proof of concept on generating
[DOT](<https://en.wikipedia.org/wiki/DOT_(graph_description_language)>)
output from a [PlantUML](http://plantuml.com/)-like description language,
ready to be processed by [GraphViz](http://graphviz.org/)
e.g. to render SVG files containing the UML diagrams.

See [this test case](resources/tests/example) for example input and output.

Written in Rust, using
[nom](https://github.com/Geal/nom) to parse input and
[dot](https://crates.io/crates/dot) to generate DOT output.
