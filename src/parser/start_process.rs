use crate::parser::parser::handle;
use crate::parser::parser::ACTIVITY_REVIEWS;
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;
use xml::attribute::OwnedAttribute;
use xml::reader::{EventReader, XmlEvent};

pub fn name() -> &'static str {
   return "StartProcess";
}

pub fn f_start_process(output: &mut String, parser: &mut EventReader<BufReader<File>>) {
   let mut index = false;
   let mut parser = parser;
   let mut output = output;
  writeln!(output, "{{").unwrap();

   handle(
      &mut output,
      &mut parser,
      name(),
      |output: &mut String, attributes: &Vec<OwnedAttribute>| {
         write!(
            output,
            "const {} =  ",
            attributes.first().expect("name").value
         )
         .expect("expect name");

         if attributes.first().unwrap().value == "params" {
            let list = attributes.last().unwrap().value.split(",");

            write!(output, "[").unwrap();
            for x in list {
               write!(output, "\"{}\"", x.trim()).unwrap();
            }

            writeln!(output, "];").unwrap();
         } else {
            writeln!(output, "\"{}\";", attributes.last().expect("val").value).expect("expect val");
         }
      },
      |output: &mut String| {
         writeln!(
            output,
            "start({}, {}, {})",
            "fileName", "params", "workingDirectory"
         )
         .expect("error");
      },
   );
   writeln!(output, "}}").unwrap();
}
