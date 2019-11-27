use crate::engine::*;
use dyon;
fn run(module: dyon::Module) {
    let mut runtime = dyon::Runtime::new();

    if dyon::error(runtime.run(&std::sync::Arc::new(module))) {
        return;
    }
}

pub fn load(file: String) {
    let mut dyon_module = core::add_fun();
    core::load_file(file, &mut dyon_module);

    run(dyon_module);
}

pub fn load_code(code: String) {
    let mut dyon_module = core::add_fun();
    core::load_str(code, &mut dyon_module);
    run(dyon_module);
}
