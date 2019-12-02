use crate::v8::{StdString, LocalValue};
use std::ffi::CStr;
use std::os::raw::c_char;

impl StdString {
    pub fn new(str: String) -> Self {
        let name = std::ffi::CString::new(str).unwrap();
        let name_ptr = name.as_ptr();

        unsafe {
            return cpp!([name_ptr as "const char *"] -> StdString as "std::string" {
                return std::string(name_ptr);
            });
        }
    }

    pub fn to_string(self) -> String {
        unsafe {
            let str = cpp!([self as "std::string"]-> *const c_char as "const char*"{
              return self.c_str();
            });

            CStr::from_ptr(str).to_string_lossy().into_owned()
        }
    }
   pub fn new2(value: LocalValue) -> Self {
        unsafe {
            return cpp!([value as "v8::Local<v8::Value>"] -> StdString as "std::string" {
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);

                v8::String::Utf8Value utf8(isolate, value);

                return std::string(*utf8? *utf8 : "<unknown>");
            });
        }
    }
}
