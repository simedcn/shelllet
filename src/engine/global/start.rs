use crate::v8::FunctionCalback;
use crate::v8::FunctionCallbackInfo;

pub struct StartImpl {}

impl FunctionCalback for StartImpl {
    fn callback(&self, info: &FunctionCallbackInfo) {
        use std::path::Path;

        let program = info.at(0).to_string();
        let result = which::which(&program).unwrap();

        let array = info.at(1).to_array();

        let mut args = Vec::new();

        for x in array {
            args.push(x.to_string());
        }

        let mut current_dir = result
            .parent()
            .expect("expcect parent")
            .canonicalize()
            .expect("abs");

        let p1 = info.at(2);
        if !p1.is_null_or_undefined() {
            let p = p1.to_string();
            current_dir = Path::new(&p).to_path_buf();
        }

        if let Err(e) = std::process::Command::new(&program)
            .args(args)
            .current_dir(&current_dir)
            .spawn()
        {
            println!("start failed: {}", e);
        }
        // .args(args)
        // .current_dir(if current_dir.is_empty() { ".".to_string()} else {current_dir})
        // .spawn()
        // .expect("failed to execute process");
    }

    fn fn_name(&self) -> &'static str {
        "start"
    }
}
