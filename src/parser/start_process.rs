use std::fmt::Write;

pub fn name() -> &'static str {
   return "StartProcess";
}

use crate::parser::parser;


pub fn start_process(output: &mut String, attributes: Vec<parser::OwnedAttribute>) {
  
   writeln!(output, "let x =5;");
   return;
   let mut index = false;

   for x in &attributes {
      if !index {
         write!(output, "const {} =  ", x.value)
            .expect("Error occurred while trying to write in String");
         index = true;
      } else if index {
         writeln!(output, "{:?}", x.value).expect("Error occurred while trying to write in String");
         index = false;
      }
   }

   //let find = attributes.iter().find(|x| x.value == "Arguments");

  
  // writeln!(output, "file_name := {}",  );

   writeln!(
      output,
      "start({}, {}, {})",
      "FileName", "Arguments", "WorkingDirectory"
   )
   .expect("error");
}
