use xml::attribute::OwnedAttribute;
use crate::parser::parser::handle;
use crate::parser::parser::ACTIVITY_REVIEWS;
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

pub fn name() -> &'static str {
   return "StartProcess";
}

pub fn f_start_process(output: &mut String, parser: &mut EventReader<BufReader<File>>) {
   let mut index = false;
   let mut parser = parser;
   let mut output = output;
   writeln!(output, "let startProcess = function() {{");


   handle(&mut output, &mut parser, name(), |output: &mut String, attributes: &Vec<OwnedAttribute>|{
      write!(
         output,
         "const {} =  ",
         attributes.first().expect("name").value
      )
      .expect("expect name");

      writeln!(output, "\"{}\";", attributes.last().expect("val").value)
         .expect("expect val");

   }, |output: &mut String|{
      writeln!(
         output,
         "start({}, {}, {})",
         "FileName", "Arguments", "WorkingDirectory"
      )
      .expect("error");
      
   });
   // loop {
   //    match parser.next() {
   //       Ok(XmlEvent::StartElement {
   //          ref name,
   //          ref mut attributes,
   //          ..
   //       }) => {
   //          if ACTIVITY_REVIEWS.contains_key(&name.local_name.as_str()) {
   //             ACTIVITY_REVIEWS[&name.local_name.as_str()](&mut output, &mut parser);
   //          } else if name.local_name == "VALUE" {
   //             write!(
   //                output,
   //                "const {} =  ",
   //                attributes.first().expect("name").value
   //             )
   //             .expect("expect name");

   //             writeln!(output, "\"{}\";", attributes.last().expect("val").value)
   //                .expect("expect val");
   //          }
   //       }

   //       Ok(XmlEvent::EndElement { name: _name }) => {
   //          if _name.local_name == name() {
   //             writeln!(
   //                output,
   //                "start({}, {}, {})",
   //                "FileName", "Arguments", "WorkingDirectory"
   //             )
   //             .expect("error");
   //             break;
   //          }
   //       }
   //       Ok(XmlEvent::EndDocument) => break,
   //       Err(e) => {
   //          println!("Error: {}", e);
   //       }
   //       _ => {}
   //    }
   // }
   //let find = attributes.iter().find(|x| x.value == "Arguments");

   // writeln!(output, "file_name := {}",  );

   writeln!(output, "}}");

   writeln!(output, "startProcess();");
}
