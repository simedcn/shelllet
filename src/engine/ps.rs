
// pub struct Ps {}

// impl FunctionCalback for Console {
//     fn callback(&self, info: &FunctionCallbackInfo) {
//         let mut message = String::new();

//         for x in 0..info.length() {
//             message = message + &StdString::new2(info.at(x)).to_string();
//         }

//         println!("{}", message);
//     }
// }

// impl Base for Console {
//     fn created_object_template(&self, tpl: &mut LocalObjectTemplate) {
//         let f = LocalFunctionTemplate::new(self);
//         tpl.set2("log".to_string(), f);
//     }
//     fn create_function_template(&self, tpl: &mut LocalObjectTemplate) {}

//     fn name(&self) -> String {
//         String::from("console")
//     }
// }



// fn start(program: String, current_dir: String, args: Vec<String> ) {
//     std::process::Command::new(program)
//     .args(args)
//     .current_dir(if current_dir.is_empty() { ".".to_string()} else {current_dir})
//     .spawn()
//     .expect("failed to execute process");
// }
