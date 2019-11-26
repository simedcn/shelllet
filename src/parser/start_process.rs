

use std::fmt::Write;

pub fn name()-> &'static str{
   return "StartProcess";
  
}


pub fn start_process( attributes: Vec<xml::attribute::OwnedAttribute>){

  let mut index = false;
  let mut output  = String::new();
for x in &attributes {

   if !index {
    write!(&mut output, "let {} =  ", x.value).expect("Error occurred while trying to write in String");
      index = true;
   } else if index{
      writeln!(&mut output, "{:?}", x.value).expect("Error occurred while trying to write in String");
      index = false;
   }
  // println!("{:?}, {:?}", x.name.local_name, x.value);
}

  let aa= attributes.iter().find(|x| {
   println!("{}", x.name.local_name);  
   x.value == "DisplayName"});
  println!("{:?}", aa.unwrap().value);

   writeln!(&mut output, "start({}, {}, {})", "FileName", "Arguments", "WorkingDirectory").expect("error");
   println!("{}", output );
}