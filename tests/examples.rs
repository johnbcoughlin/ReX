extern crate rex;
extern crate toml;

use std::fs::File;
use std::io::Read;
use std::fmt;

use toml::Value;

macro_rules! expect_string {
    ($value:expr) => ({
        match $value {
            Value::String(ref value) => value,
            _ => panic!("Expected a string value in toml field!"),
        }
    })
}

#[test]
fn generate_examples() {
    let mut file  = File::open("tests/examples.toml")
        .expect("Unable to open 'examples.toml'");

    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Unable to read 'examples.toml'");

    let mut parser = toml::Parser::new(&input);
    let toml = match parser.parse() {
            Some(value) => value,
            None => {
                println!("Parse Errors: {:?}", parser.errors);
                panic!("Failed to parse 'examples.toml'");
            }
        };

    let examples = match toml["example"] {
            Value::Array(ref values) => values,
            _ => {
                println!("Expected an array of values from `examples` table!");
                panic!();
            }
        };

    println!("");   // remove indent from debugging in panics.
    let svg = rex::SVGRenderer::new().font_size(96.0).debug(false);
    for example in examples {
        if let &Value::Table(ref table) = example {
            let name = expect_string!(table["name"]);
            let tex  = expect_string!(table["tex"]);

            let filename = format!("samples/{}.svg", name.replace(" ", "_"));
            let filename_png = format!("samples/{}.png", name.replace(" ", "_"));

            println!(r"### {}", name);
            println!(r"`{}`", tex);
            println!(r"");
            println!(r"![Example]({})", filename_png);
            println!(r"");

            svg.render_to_file(filename, &tex);
        }
    }
    panic!()
}