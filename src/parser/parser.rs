use std::collections::HashMap;
use xml::attribute::OwnedAttribute;

use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

use crate::parser::sequence;
use crate::parser::start_process;

// impl std::iter::FromIterator<char> for xml::attribute::OwnedAttribute {
//     fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> String {
//         let mut buf = String::new();
//         buf.extend(iter);
//         buf
//     }
// }

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

pub fn parse(file_name: String) -> String {
    use std::fmt::Write;
    let mut output = String::new();
    let mut stream = String::new();

    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    let mut parser = EventReader::new(file);


    //handle(&mut output, &mut parser, name(), |_, _| {}, |_| {});


    ACTIVITY_REVIEWS[sequence::name()](&mut stream, &mut parser);

    // loop {
    //     match parser.next() {
    //         Ok(XmlEvent::StartElement {
    //             ref name,
    //             ref mut attributes,
    //             ..
    //         }) => {
    //             if ACTIVITY_REVIEWS.contains_key(&name.local_name.as_str()) {
    //                 ACTIVITY_REVIEWS[&name.local_name.as_str()](&mut stream, &mut parser);
    //             }
    //         }
    //         Ok(XmlEvent::EndDocument) => break,
    //         Err(e) => {
    //             panic!("Error: {}", e);
    //         }
    //         _ => {}
    //     }
    // }
    writeln!(&mut output, "{}", stream).expect("stream");

    output
}
