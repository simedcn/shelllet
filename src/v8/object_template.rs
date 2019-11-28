
//use crate::v8::function_template::LocalFunctionTemplate;
use crate::v8::data::LocalData;

cpp_class!(pub(crate) unsafe struct LocalObjectTemplate as "v8::Local<v8::ObjectTemplate>");




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
    pub fn set(&mut self, isolate: *mut u32,  name: String,  value: LocalData ){
        let c_to_print = std::ffi::CString::new(name).expect("CString::new failed");
        let cstring = c_to_print.as_ptr();
        unsafe {
            return cpp!([self as "v8::Local<v8::ObjectTemplate>", isolate as "v8::Isolate*",  cstring as "char*", value as "v8::Local<v8::Data>" ] {
                v8::Locker locker(isolate);
                v8::Isolate::Scope isolate_scope(isolate);
                v8::HandleScope handle_scope(isolate);
                self->Set(isolate, cstring, value);
            });
        }

    }
 


}