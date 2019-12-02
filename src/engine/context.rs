use crate::v8::core::WrapperTrait;
use crate::v8::*;

use crate::engine::console::Console;
use crate::engine::global;

pub fn load(source: String) {
    let contents = std::fs::read_to_string(&source).expect("Something went wrong reading the file");
    let path = std::path::Path::new(&source);

    load_code(
        path.file_stem().unwrap().to_string_lossy().to_string(),
        contents,
    );
}

struct WrapperTraitImpl {
    name: String,
    src: String,
    //  global_context: GlobalContext,
    // cb: ConsoleImpl,
    console: Console,
}
impl WrapperTrait for WrapperTraitImpl {
    fn source(&self) -> StdString {
        StdString::new(self.src.clone())
    }
    fn name(&self) -> StdString {
        StdString::new(self.name.clone())
    }

    fn objtpl(&mut self) -> LocalObjectTemplate {
        let mut obj = LocalObjectTemplate::new();
        let g = global::global_function();

        for x in g.iter() {
            let f = LocalFunctionTemplate::new(x.as_ref());
            obj.set(x.fn_name().to_string(), f.data());
        }
        obj
    }
    fn gl(&self, obj: LocalObject) {
        let mut tpl = LocalObjectTemplate::new();
        self.console.created_object_template(&mut tpl);

        let inst = tpl.new_instance();

        obj.create_data_property(LocalName::new(&self.console.name()), inst.value());
    }
}

pub fn load_code(name: String, src: String) {
    //  let cb = ConsoleImpl {};
    //let global_context: GlobalContext = GlobalContext::new();

    let console = Console {};
    let wrapper = WrapperTraitImpl {
        name,
        src,
        //   global_context,
        //   cb,
        console,
    };
    let wrapper_ptr: &dyn WrapperTrait = &wrapper;
    use crate::v8::core::run;
    run(wrapper_ptr);
}
