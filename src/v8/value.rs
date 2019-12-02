use crate::v8::*;

impl LocalValue {
    pub fn new() -> Self {
        unsafe {
            cpp!([] -> LocalValue as "v8::Local<v8::Value>" {
                return v8::Local<v8::Value>();
            })
        }
    }
    pub fn is_string(self) -> bool {
        unsafe {
            cpp!([self as "v8::Local<v8::Value>"] -> bool as "bool" {
               return self->IsString();
            })
        }
    }

    pub fn is_uint32(self) -> bool {
        unsafe {
            cpp!([self as "v8::Local<v8::Value>"] -> bool as "bool" {
               return self->IsUint32();
            })
        }
    }
}
