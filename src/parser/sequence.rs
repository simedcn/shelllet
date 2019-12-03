use crate::parser::parser::handle;
use crate::parser::parser::ACTIVITY_REVIEWS;
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

pub fn name() -> &'static str {
   return "Sequence";
}

pub fn f_sequence(output: &mut String, parser: &mut EventReader<BufReader<File>>) {
   let mut index = false;
   let mut parser = parser;
   let mut output = output;

   writeln!(output, "let sequence = function() {{");

   handle(&mut output, &mut parser, name(), |_, _| {}, |_| {});
   // loop {
   //    match parser.next() {
   //       Ok(XmlEvent::StartElement {
   //          ref name,
   //          ref mut attributes,
   //          ..
   //       }) => {
   //          if ACTIVITY_REVIEWS.contains_key(&name.local_name.as_str()) {
   //             ACTIVITY_REVIEWS[&name.local_name.as_str()](&mut output, &mut parser);
   //          } else {
   //          }
   //       }
   //       Ok(XmlEvent::EndElement { name: _name }) => {
   //          if _name.local_name == name() {
   //             break;
   //          }
   //       }
   //       Ok(XmlEvent::EndDocument) => break,
   //       Err(e) => {
   //          panic!("Error: {}", e);
   //       }
   //       _ => {}
   //    }
   // }
   writeln!(output, "}}");

   writeln!(output, "sequence();");
}
