
use crate::v8::*;

impl LocalName {
    pub fn new(str: &String) -> Self {
        let name = std::ffi::CString::new(str.as_str()).unwrap();
        let name_ptr = name.as_ptr();

        unsafe {
            return cpp!([name_ptr as "const char *"] -> LocalName as "v8::Local<v8::Name>" {
                auto isolate = v8::Isolate::GetCurrent();
                return  v8::Local<v8::Name>::New(isolate, 
                    v8::String::NewFromUtf8(isolate, name_ptr).ToLocalChecked());

               
            });
        }
    }
}
