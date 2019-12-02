use crate::v8::*;

impl LocalObjectTemplate {
    pub fn new() -> Self {
        unsafe {
            return cpp!([] -> LocalObjectTemplate as "v8::Local<v8::ObjectTemplate>" {
                // v8::Isolate::Scope isolate_scope(isolate);
               auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);
                //v8::HandleScope handle_scope(isolate);
                v8::EscapableHandleScope handle_scope(isolate);
                auto tpl = v8::ObjectTemplate::New(isolate);
                return handle_scope.Escape(tpl);
               //return object_template;
            });
        }
    }
    pub fn set(&mut self, name: String, value: LocalData) {
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
    pub fn new_instance(&mut self) -> LocalObject {
        unsafe {
            cpp!([self as "v8::Local<v8::ObjectTemplate>*"]
            -> LocalObject as "v8::Local<v8::Object>"{
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);
                v8::EscapableHandleScope handle_scope(isolate);
               v8::Local<v8::Context> context = v8::Local<v8::Context>::New(isolate, gl_context.Get(isolate));
               v8::Context::Scope context_scope(context);
                auto inst = (*self)->NewInstance(context).ToLocalChecked();
               return handle_scope.Escape(inst);
            })
        }
    }
    pub fn set2(&mut self, name: String, value: LocalFunctionTemplate) {
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
