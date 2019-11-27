dyon_fn!{fn start(program: String, current_dir: String, args: Vec<String> ) {
    std::process::Command::new(program)
    .args(args)
    .current_dir(if current_dir.is_empty() { ".".to_string()} else {current_dir})
    .spawn()
    .expect("failed to execute process");

}}
