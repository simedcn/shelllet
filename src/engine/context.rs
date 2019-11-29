

use crate::v8::object_template::LocalObjectTemplate;
use crate::v8::function_template::LocalFunctionTemplate;
use crate::v8::object::LocalObject;
use crate::v8::core::WrapperTrait;
use crate::v8::function_template::FunctionCalback;
use crate::v8::function_callback_Info::FunctionCallbackInfo;
use crate::v8::string::StdString;

use crate::v8::context::GlobalContext;
pub fn load(source: String) {

    let contents = std::fs::read_to_string(&source)
    .expect("Something went wrong reading the file");
    let path = std::path::Path::new(&source);


    load_code(path.file_stem().unwrap().to_string_lossy().to_string(), contents);
}

pub struct FunctionCalbackImpl {}

impl FunctionCalback for FunctionCalbackImpl {
    fn callback(&self, info: FunctionCallbackInfo) {
        println!("called");
    }
}


struct WrapperTraitImpl {
    name:String,
    src: String,
    cb: FunctionCalbackImpl
}
impl WrapperTrait for WrapperTraitImpl {
    fn source(&self) -> StdString{
       StdString::new(self.src.clone())
    }
    fn name(&self) -> StdString{
        StdString::new(self.name.clone())
    }

    fn objtpl(&mut self) ->LocalObjectTemplate{

      let mut obj =  LocalObjectTemplate::new();

     

      let f = LocalFunctionTemplate::new( &mut self.cb);
      obj.set2("dd".to_string(), f);
      obj


    }
    fn gl(&self, obj : LocalObject){

    }
}


pub fn load_code(name:String, src: String) {
   let cb = FunctionCalbackImpl{};
  
    let wrapper = WrapperTraitImpl{ name, src, cb };
    let wrapper_ptr: &dyn WrapperTrait = &wrapper;
use crate::v8::core::run;
let  mut global_context: GlobalContext = GlobalContext::new();
    run(&mut global_context, wrapper_ptr);
}
