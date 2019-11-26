dyon_fn!{fn start(program: String, current_dir: String, args: Vec<String> ) {
   
    std::process::Command::new(program)
    .args(args)
    .current_dir(current_dir)
    .spawn()
    .expect("failed to execute process");

}}
