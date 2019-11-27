use crate::engine::*;
use dyon::*;

pub fn add_fun() ->   dyon::Module {
    use dyon::{error, load, Dfn, Module};
    use dyon::Type::*;

    let mut module = Module::new();
    module.add_str("remove_file", fs::remove_file, Dfn::nl(vec![dyon::Type::Str], Void));
    module.add_str("start", ps::start, Dfn::nl(vec![dyon::Type::Str,dyon::Type::Str,dyon::Type::Str], Void));
   
   return module;
}


pub fn load_file(file: String, module: &mut Module) -> std::option::Option<String>{
    if dyon::error(dyon::load(&file, module)) {
        None
    } else {
        Some("".to_owned())
    }
}

pub fn load_str(code: String, module: &mut Module) -> std::option::Option<String>{
    if dyon::error(dyon::load_str("noname", std::sync::Arc::new(code), module)) {
        None
    } else {
        Some("".to_owned())
    }
}