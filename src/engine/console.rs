use crate::v8::function_template::FunctionCalback;
use crate::v8::*;


pub struct Console {

}



impl FunctionCalback for Console {
    fn callback(&self, info: &FunctionCallbackInfo) {
        println!("called");
        let mut message = String::new();

        for x in 0..info.length(){
            message = message + &StdString::new2(info.at(x)).to_string();
        }

        print!("{}", message);
    }
}

impl Base for Console {
    fn created_object_template(& self, tpl: &mut LocalObjectTemplate){
    

        let f = LocalFunctionTemplate::new(self);
        tpl.set2("log".to_string(), f);

    }
    fn create_function_template(&self, tpl: &mut LocalObjectTemplate){

    }

    fn name(&self) ->String{
        String::from("console")
    }
}