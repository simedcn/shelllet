use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::iter::once;
use winapi::um::winuser::MessageBoxA;
use winapi::um::winuser::MB_OK;

use xml::reader::{EventReader, XmlEvent};

use crate::parser::start_process;

// impl std::iter::FromIterator<char> for xml::attribute::OwnedAttribute {
//     fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> String {
//         let mut buf = String::new();
//         buf.extend(iter);
//         buf
//     }
// }

lazy_static! {
    static ref ACTIVITY_REVIEWS: HashMap<&'static str, fn(&mut String, Vec<OwnedAttribute>)> = {
        let mut m = HashMap::new();
        m.insert(
            start_process::name(),
            start_process::start_process as fn(&mut String, Vec<OwnedAttribute>),
        );
        m
    };
}

#[derive(Clone)]
pub struct OwnedAttribute {
    pub name: String,
    pub value: String,
 }
 



fn convert(attributes: Vec<xml::attribute::OwnedAttribute>) -> Vec<OwnedAttribute>{
    let mut index = false;
 
    let mut result = Vec::new();
 
    let mut attribute = OwnedAttribute {name: "".to_string(), value: "".to_string()};
   // attribute.name =  String::from("anotheremail@example.com");

    for x in attributes {
 
       if !index {
          attribute.name = x.value.clone();
          index = true;
       } else if index {
        
          attribute.value = x.value.clone();
          result.push(OwnedAttribute {..attribute.clone()});
          index = false;
       }
 
    }
 
    result
 
 }

pub fn parse(file_name: String) ->String{
    use std::fmt::Write;
    let wide = std::ffi::CString::new("aa").unwrap();
    let mut output = String::new();
    let mut stream = String::new();
    unsafe {

        // MessageBoxA(std::ptr::null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK);
    };

    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    let mut parser = EventReader::new(file);
    let mut depth = 0;
    loop {
        let mut e = parser.next();
        match e {
            Ok(XmlEvent::StartElement {
               ref name, ref mut attributes, ..
            }) => {
              //  println!("{}+{}", indent(depth), name);
                if ACTIVITY_REVIEWS.contains_key(&name.local_name.as_str()) {
                    loop {
                        let mut ee = parser.next();
                        match ee {
                            Ok(XmlEvent::StartElement {
                               attributes: ref mut _attr,
                                ..
                            }) => {
                                attributes.append(_attr);
                            }

                            Ok(XmlEvent::EndElement { name : _name }) => {
                                depth -= 1;
                               // println!("{}-{}", indent(depth), name);
                               if *name == _name {
                                   break;
                               }
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                                break;
                            }
                            _ => {}
                        }
                    }

                    ACTIVITY_REVIEWS[&name.local_name.as_str()](&mut stream, convert(attributes.to_vec()));
                }

                depth += 1;
            }
            Ok(XmlEvent::EndDocument) => break,
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    writeln!(&mut output, "fn main() {{").expect("ss");
    writeln!(&mut output, "  {}", stream).expect("ss");
    writeln!(&mut output, "}}").expect("ss");

    output
}
