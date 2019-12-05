use std::collections::HashMap;
use xml::attribute::OwnedAttribute;

use minidom::Element;
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

use crate::parser::sequence;
use crate::parser::start_process;

lazy_static! {
    pub static ref ACTIVITY_REVIEWS: HashMap<&'static str, fn(&mut String, &mut EventReader<BufReader<File>>)> = {
        let mut m = HashMap::new();
        m.insert(
            start_process::name(),
            start_process::f_start_process as fn(&mut String, &mut EventReader<BufReader<File>>),
        );
        m.insert(
            sequence::name(),
            sequence::f_sequence as fn(&mut String, &mut EventReader<BufReader<File>>),
        );
        m
    };
}

pub fn handle(
    output: &mut String,
    parser: &mut EventReader<BufReader<File>>,
    item_name: &str,
    start: fn(output: &mut String, attributes: &Vec<OwnedAttribute>),
    end: fn(output: &mut String),
) {
    let mut parser = parser;
    let mut output = output;

    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement {
                ref name,
                ref attributes,
                ..
            }) => {
                if ACTIVITY_REVIEWS.contains_key(&name.local_name.as_str()) {
                    ACTIVITY_REVIEWS[&name.local_name.as_str()](&mut output, &mut parser);
                } else if name.local_name == "VALUE" {
                    if attributes.first().expect("name").value == "displayName" {
                        writeln!(output, "// {}", attributes.last().expect("value").value);
                        continue;
                    }

                    start(&mut output, &attributes);
                }
            }

            Ok(XmlEvent::EndElement { name: _name }) => {
                if _name.local_name == item_name {
                    end(&mut output);
                    break;
                }
            }
            Ok(XmlEvent::EndDocument) => break,
            Err(e) => {
                println!("Error: {}", e);
            }
            _ => {}
        }
    }
}

fn walk(root: &Element, code: &mut String) {
    let mut code = code;
    for child in root.children() {
        if ACTIVITY_REVIEWS.contains_key(&child.name()) {
            writeln!(&mut code, "function {}(){{", child.name()).expect("code");

            writeln!(&mut code, "}}").expect("code");
        }
        walk(&child, &mut code);
    }
}

pub fn parse(file_name: String) -> String {
    let mut output = String::new();
    // let mut code = String::new();

    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    let mut parser = EventReader::new(file);
   

    ACTIVITY_REVIEWS[sequence::name()](&mut output, &mut parser);
    // let root: Element = std::fs::read_to_string(&file_name)
    //     .expect("config file")
    //     .parse()
    //     .unwrap();

    // writeln!(&mut output, "{}", code).expect("code");

    output
}
