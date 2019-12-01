

use crate::v8::*;





impl LocalObjectTemplate {
    pub fn new() -> Self {
        unsafe {
            return cpp!([] -> LocalObjectTemplate as "v8::Local<v8::ObjectTemplate>" {
                // v8::Locker locker(isolate);
                // v8::Isolate::Scope isolate_scope(isolate);
                // v8::HandleScope handle_scope(isolate);
                auto isolate = v8::Isolate::GetCurrent();
                v8::EscapableHandleScope handle_scope(isolate);

                v8::Local<v8::ObjectTemplate> object_template = v8::ObjectTemplate::New(isolate);
                return handle_scope.Escape(object_template);
            });
        }
    }
    pub fn set(&mut self, name: String,  value: LocalData ){
        let c_to_print = std::ffi::CString::new(name).expect("CString::new failed");
        let cstring = c_to_print.as_ptr();
        unsafe {
            return cpp!([self as "v8::Local<v8::ObjectTemplate>*", cstring as "char*", value as "v8::Local<v8::Data>" ] {
              
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);
                v8::HandleScope handle_scope(isolate);
                std::cout << "#name:" << cstring << std::endl;
                (*self)->Set(isolate, cstring, value);
            });
        }

    }
    pub fn new_instance(&mut self, gl_context: &GlobalContext) -> Result<LocalObject, &str>{
       let result =  unsafe {
            cpp!([self as "v8::Local<v8::ObjectTemplate>", gl_context as "v8::Global<v8::Context>"]
            -> MaybeLocalObject as "v8::MaybeLocal<v8::Object>"{
                auto isolate = v8::Isolate::GetCurrent();
                v8::Local<v8::Context> context = v8::Local<v8::Context>::New(isolate, gl_context.Get(isolate));
                v8::Context::Scope context_scope(context);
               return self->NewInstance(context);
            })
        };

        if result.is_empty() {
            Err("errr")
        } else{
            Ok((& result).to_local_checked())
        }
    }
    pub fn set2(&mut self, name: String,  value: LocalFunctionTemplate ){
        let c_to_print = std::ffi::CString::new(name).expect("CString::new failed");
        let cstring = c_to_print.as_ptr();
        unsafe {
            return cpp!([self as "v8::Local<v8::ObjectTemplate>*", cstring as "char*", value as "v8::Local<v8::FunctionTemplate>" ] {
              
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);
                v8::HandleScope handle_scope(isolate);
                std::cout << "#name:" << cstring << std::endl;
                (*self)->Set(isolate, cstring, value);
            });
        }

    }


}