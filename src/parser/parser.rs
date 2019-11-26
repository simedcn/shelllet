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
    static ref BOOK_REVIEWS: HashMap<&'static str, fn(Vec<xml::attribute::OwnedAttribute>)> = {
        let mut m = HashMap::new();
        m.insert(
            start_process::name(),
            start_process::start_process as fn(Vec<xml::attribute::OwnedAttribute>),
        );
        m
    };
}

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size)
        .map(|_| INDENT)
        .fold(String::with_capacity(size * INDENT.len()), |r, s| r + s)
}

pub fn parse(file_name: String) {
    let wide = std::ffi::CString::new("aa").unwrap();

    unsafe {

        // MessageBoxA(std::ptr::null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK);
    };

    let file = File::open("C:\\Users\\Admin\\shelllet.com\\Projects\\dd\\main.xml").unwrap();
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
                if BOOK_REVIEWS.contains_key(&name.local_name.as_str()) {
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

                    BOOK_REVIEWS[&name.local_name.as_str()](attributes.to_vec());
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
}
