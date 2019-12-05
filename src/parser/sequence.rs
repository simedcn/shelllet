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
   let mut parser = parser;
   let mut output = output;

   writeln!(output, "{{").unwrap();


   handle(&mut output, &mut parser, name(), |_, _| {}, |_| {});

   writeln!(output, "}}").unwrap();
}
