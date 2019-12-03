use crate::v8::*;

impl LocalValue {
    pub fn new() -> Self {
        unsafe {
            cpp!([] -> LocalValue as "v8::Local<v8::Value>" {
                return v8::Local<v8::Value>();
            })
        }
    }
    pub fn null() -> Self{
        unsafe {
            cpp!([] -> LocalValue as "v8::Local<v8::Value>" {
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);
                return v8::Null(isolate);
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
    pub fn is_empty(self) -> bool {
        unsafe {
            cpp!([self as "v8::Local<v8::Value>"] -> bool as "bool" {
               return self.IsEmpty();
            })
        }
    }

    pub fn is_null_or_undefined(self) -> bool {
        unsafe {
            cpp!([self as "v8::Local<v8::Value>"] -> bool as "bool" {
               return self->IsNullOrUndefined();
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

    pub fn to_string(self) -> String {
        let s = unsafe {
            cpp!([self as "v8::Local<v8::Value>"] -> StdString as "std::string" {
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);
                if (!self->IsString()) {
                    rust!( panic []{ panic!("value is not array") });
                }
                v8::String::Utf8Value v(isolate, self);
                return std::string(*v ? *v : unknown);
            })
        };

        s.to_string()
    }


    pub fn to_array(self) -> Vec<Box<LocalValue>> {
        let s = unsafe {
            cpp!([self as "v8::Local<v8::Value>"] -> Array as "v8::Local<v8::Array>" {
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);

                if (!self->IsArray()) {
                    //return v8::Array::New(isolate, 0);
                    rust!( panic []{ panic!("value is not array") });
                }
                v8::Local<v8::Array> array = v8::Local<v8::Array>::Cast(self);
        
                v8::EscapableHandleScope handle_scope(isolate);
                return handle_scope.Escape(array);
            })
        };

       s.to_array()
    }
}
