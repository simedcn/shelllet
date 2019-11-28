//cpp_class!(pub(crate) unsafe struct FunctionTemplate as "v8::FunctionTemplate");
use crate::v8::data::LocalData;
use crate::v8::function_callback_Info::FunctionCallbackInfo;

cpp_class!(pub(crate) unsafe struct LocalFunctionTemplate as "v8::Local<v8::FunctionTemplate>");


pub trait FunctionCalback {
    fn callback(&self, info: FunctionCallbackInfo);
}


cpp! {{
   class FunctionCalback {
       virtual void callback(const v8::FunctionCallbackInfo<v8::Value>& args) = 0;
   };

   //#include <iostream>
   struct FunctionCalbackHolder {void*a;  };

   class FunctionCalbackImpl : public FunctionCalback{
       public:
       FunctionCalbackHolder m_trait;
       virtual void callback(const v8::FunctionCallbackInfo<v8::Value>& args){
           return rust!(MCI_callback [m_trait : &dyn FunctionCalback as "FunctionCalbackHolder", args : FunctionCallbackInfo as "const v8::FunctionCallbackInfo<v8::Value>&"]
           {
              m_trait.callback(args);
       });
       }
   };

}}

//use LocalFunctionTemplate::v8::local::LocalFunctionTemplate;

impl LocalFunctionTemplate {
    pub fn new(callback: &mut impl FunctionCalback) -> LocalFunctionTemplate {
        unsafe {
            return cpp!([callback as "FunctionCalback*"] -> LocalFunctionTemplate as "v8::Local<v8::FunctionTemplate>" {
               auto isolate = v8::Isolate::GetCurrent();
               v8::HandleScope handle_scope(isolate);
               FunctionCalback*m_trait  = callback;

                auto ooo = v8::FunctionTemplate::New(isolate, [](const v8::FunctionCallbackInfo<v8::Value>& callback_info) {
                    v8::Local<v8::External> external = callback_info.Data().As<v8::External>();
                    FunctionCalback* callback = reinterpret_cast<FunctionCalback*>(external->Value());

                   rust!(MCI_computeValue [callback : &dyn FunctionCalback as "FunctionCalback*", callback_info : FunctionCallbackInfo as "const v8::FunctionCallbackInfo<v8::Value>&"] {
                    callback.callback(callback_info)});
               }, v8::External::New(isolate, m_trait));

                    return ooo;
            });
        }
    }

    pub fn data(&mut self) -> LocalData {
        unsafe {
            return cpp!([self as "v8::Local<v8::FunctionTemplate>"]-> LocalData as "v8::Local<v8::Data>"{
                return self;
            });
        }
    }
}
