//cpp_class!(pub(crate) unsafe struct FunctionTemplate as "v8::FunctionTemplate");
use crate::v8::*;

pub trait FunctionCalback {
    fn callback(&self, info: &FunctionCallbackInfo);
}

cpp! {{
   class FunctionCalback {
       virtual void callback(const v8::FunctionCallbackInfo<v8::Value>& args) = 0;
   };

   //#include <iostream>
   struct FunctionCalbackHolder {void*a, *b;  };

   class FunctionCalbackImpl : public FunctionCalback{
       public:
       FunctionCalbackHolder m_trait;
       virtual void callback(const v8::FunctionCallbackInfo<v8::Value>& args){
            rust!(MCI_callback [m_trait : &dyn FunctionCalback as "FunctionCalbackHolder", args : FunctionCallbackInfo as "v8::FunctionCallbackInfo<v8::Value>"]
            {
                m_trait.callback(&args);
            });
       }
   };
   void native_function_callback(const v8::FunctionCallbackInfo<v8::Value>& args){
        v8::Local<v8::External> external = args.Data().As<v8::External>();
        FunctionCalbackImpl* imp = reinterpret_cast<FunctionCalbackImpl*>(external->Value());
        imp->callback(args);
   }
}}

//use LocalFunctionTemplate::v8::local::LocalFunctionTemplate;

impl LocalFunctionTemplate {
    pub fn new(inst_ptr: &dyn FunctionCalback) -> LocalFunctionTemplate {
        unsafe {
            return cpp!([inst_ptr as "FunctionCalbackHolder"] -> LocalFunctionTemplate as "v8::Local<v8::FunctionTemplate>" {
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);
                v8::EscapableHandleScope handle_scope(isolate);

                FunctionCalbackImpl* impl = new FunctionCalbackImpl();
                impl->m_trait = inst_ptr;
                auto ooo = v8::FunctionTemplate::New(isolate, native_function_callback, v8::External::New(isolate, impl));
                 return handle_scope.Escape( ooo);
            });
        }
    }

    pub fn data(&self) -> LocalData {
        unsafe {
            return cpp!([self as "v8::Local<v8::FunctionTemplate>*"]-> LocalData as "v8::Local<v8::Data>"{
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);
                v8::EscapableHandleScope handle_scope(isolate);
                //v8::Local<v8::Data> data = self;
                //return self;
                return handle_scope.Escape(*self);
            });
        }
    }
}
