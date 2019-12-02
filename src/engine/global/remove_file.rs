use crate::v8::*;


pub struct RemoveFile {}

impl FunctionCalback for RemoveFile {
    fn callback(&self, info: &FunctionCallbackInfo) {
        let x = info.at(0);

        if x.is_string(){
            let path = StdString::new2(x);

              std::fs::remove_file(path.to_string()).expect("ssss");

        }

     
    }

    fn fn_name(&self) -> &'static str {
        "removeFile"
    }
}

