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
