use crate::v8::FunctionCalback;
use crate::v8::FunctionCallbackInfo;


pub struct RemoveFile {}

impl FunctionCalback for RemoveFile {
    fn callback(&self, info: &FunctionCallbackInfo) {
        println!("removed");
    }

    fn fn_name(&self) -> &'static str {
        "remove"
    }
}

