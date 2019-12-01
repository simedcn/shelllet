use crate::v8::function_template::FunctionCalback;
use crate::v8::*;


pub struct Console {

}



impl FunctionCalback for Console {
    fn callback(&self, info: FunctionCallbackInfo) {
        println!("called");
    }
}

impl Base for Console {
    fn created_object_template(& self, tpl: &mut LocalObjectTemplate){
    

        let f = LocalFunctionTemplate::new(self);
        tpl.set("log".to_string(), f.data());

    }
    fn create_function_template(&self, tpl: &mut LocalObjectTemplate){

    }

    fn name(&self) ->String{
        String::from("console")
    }
}