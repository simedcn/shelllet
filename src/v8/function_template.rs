//cpp_class!(pub(crate) unsafe struct FunctionTemplate as "v8::FunctionTemplate");


cpp_class!(pub(crate) unsafe struct LocalFunctionTemplate as "v8::Local<v8::FunctionTemplate>");



use crate::v8::data::LocalData;


pub trait FunCallback{
    fn callback(&self);
}

pub struct FnCC{

}

impl FunCallback for FnCC{
      fn callback(&self){
        println!("called");
    }
}

cpp!{{
    struct MyFunClass {
        virtual void computeValue(const v8::FunctionCallbackInfo<v8::Value>& args) = 0;
    };

    //#include <iostream>
    struct FunCallPtr {void*a;  };

//        TraitPtr m_trait;
    //    int x = 5;

        // int callRust1(int x)  {
        //     return rust!(addTwoCallback [x : i32 as "int"] -> i32 as "int" { add_two(x) });
        // }
      
      //  rust!(addTwoCallback [x: i32 as "int"] -> i32 as "int" { add_two(x) });
//            rust!(MCI_computeValue [m_trait : &Summary as "TraitPtr", x : i32 as "int32_t"]
//                -> i32 as "int" {
//                m_trait.summarize(x)   });
 }}


//use LocalFunctionTemplate::v8::local::LocalFunctionTemplate;

impl LocalFunctionTemplate {
    pub fn new(cb: &mut impl Callback) -> LocalFunctionTemplate {
        unsafe {
            return cpp!([cb as "FunCallPtr*"] -> LocalFunctionTemplate as "v8::Local<v8::FunctionTemplate>" {
               std::cout << "isolate:" << isolate << std::endl;
               auto isolate = v8::Isolate::GetCurrent();
               v8::Isolate::Scope isolate_scope(isolate); 
               v8::HandleScope handle_scope(isolate);
                
               FunCallPtr*m_trait  = cb;

                auto ooo = v8::FunctionTemplate::New(isolate, [](const v8::FunctionCallbackInfo<v8::Value>& callback_info) {
                    v8::Local<v8::External> external = callback_info.Data().As<v8::External>();
                    FunCallPtr callback = *reinterpret_cast<FunCallPtr*>(external->Value());
                  //  callback callback(&callback_info);
                   rust!(MCI_computeValue [callback : &dyn Callback as "TraitPtr"] {
                    callback.callback()});
               }, v8::External::New(isolate, m_trait));
               
            
                    return ooo;
            });
        }
    }

    pub fn data(&mut self) -> LocalData{
        unsafe {
                return cpp!([self as "v8::Local<v8::FunctionTemplate>"]-> LocalData as "v8::Local<v8::Data>"{
                    return self;
                });
        }
    }
}
// impl Drop for FunctionTemplate {
//     fn drop(&mut self){
//         println!("Shape dropping!");
//         cpp!([self as "v8::FunctionTemplate*"] {
//             delete self;
//         } );
//     }

// }
