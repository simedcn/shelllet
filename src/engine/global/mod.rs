
use crate::v8::FunctionCalback;
use crate::engine::global::remove_file::RemoveFile;
use crate::engine::global::start::StartImpl;

mod remove_file;
mod start;
pub fn global_function() -> Vec<Box<dyn FunctionCalback>> {
    let mut vec: Vec<Box<dyn FunctionCalback>> = Vec::new();

    vec.push(Box::new(StartImpl {}));
    vec.push(Box::new(RemoveFile {}));

    vec
}
