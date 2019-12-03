use crate::v8::*;

pub struct RemoveFile {}

impl FunctionCalback for RemoveFile {
    fn callback(&self, info: &FunctionCallbackInfo) {
        let p = info.at(0);

        if p.is_string() {
            let path = StdString::new2(p);

            if let Err(e) = std::fs::remove_file(path.to_string()) {
                println!("remove failed: {}", e);
            }
        }
    }

    fn fn_name(&self) -> &'static str {
        "removeFile"
    }
}
