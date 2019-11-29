use crate::v8::*;


pub struct Console {

}

impl Base for Console {
    fn created_object_template(self, tpl: &mut LocalObjectTemplate){

    }
    fn create_function_template(self, tpl: &mut LocalObjectTemplate){

    }

    fn name(self) ->String{
        String::from("console")
    }
}