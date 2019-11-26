use dyon;
use crate::engine::*;
pub fn run(matches: clap::ArgMatches){
    let mut dyon_runtime = dyon::Runtime::new();
    let mut dyon_module = core::add_fun();
    let input= matches.value_of("INPUT");
    if input.is_some() {
        core::load_file(&input.expect("sss").to_owned(), &mut dyon_module);
    } else{

        core::load_str("test()".to_owned(), &mut dyon_module);
    }

    if dyon::error(dyon_runtime.run(& std::sync::Arc::new(dyon_module))) {
        return
    }
}