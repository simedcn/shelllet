use crate::v8::FunctionCalback;
use crate::v8::FunctionCallbackInfo;

pub struct StartImpl {}

impl FunctionCalback for StartImpl {
    fn callback(&self, info: &FunctionCallbackInfo) {
        println!("start");
    }

    fn fn_name(&self) -> &'static str {
        "start"
    }
}

pub struct RemoveFile {}

impl FunctionCalback for RemoveFile {
    fn callback(&self, info: &FunctionCallbackInfo) {
        println!("removed");
    }

    fn fn_name(&self) -> &'static str {
        "remove"
    }
}

pub fn global_function() -> Vec<Box<dyn FunctionCalback>> {
    let mut vec: Vec<Box<dyn FunctionCalback>> = Vec::new();

    vec.push(Box::new(StartImpl {}));
    vec.push(Box::new(RemoveFile {}));

    vec
}
