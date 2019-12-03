use crate::v8::*;

impl Uint8Array {}

impl Array {
    pub fn get(self, index: i32) -> LocalValue {
        unsafe {
            cpp!([self as "v8::Local<v8::Array>", index as "uint32_t"]-> LocalValue as "v8::Local<v8::Value>"{
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);
                v8::EscapableHandleScope handle_scope(isolate);
                v8::Local<v8::Context> context = v8::Local<v8::Context>::New(isolate, gl_context.Get(isolate));
                v8::Context::Scope context_scope(context);
                return handle_scope.Escape(self->Get(context, index).ToLocalChecked());
            })
        }
    }

    pub fn length(self) -> i32 {
        unsafe {
            cpp!([self as "v8::Local<v8::Array>"]-> i32 as "int"{
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);
                return self->Length();
            })
        }
    }

   pub fn to_array(self) -> Vec<Box<LocalValue>> {
        let mut vec: Vec<Box<LocalValue>> = Vec::new();
        
        for x in 0.. self.length(){
            vec.push(Box::new(self.get(x)));
        }
        
        vec
    }
}
