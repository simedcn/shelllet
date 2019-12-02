use crate::v8::*;

impl FunctionCallbackInfo {
   pub fn length(self) -> i32 {
        unsafe {
            cpp!([self as "v8::FunctionCallbackInfo<v8::Value>"]-> i32 as "int"{
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);
                return self.Length();
            })
        }
    }

    pub fn at(self, index: i32) -> LocalValue {
        if index > self.length() {
            return LocalValue::new();
        }
        
        unsafe {
            cpp!([self as "v8::FunctionCallbackInfo<v8::Value>", index as "int"]-> LocalValue as "v8::Local<v8::Value>"{
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);

                v8::EscapableHandleScope handle_scope(isolate);

                return handle_scope.Escape(self[index]);
            })
        }
    }

   pub fn this(self) -> LocalObject {
        unsafe {
            cpp!([self as "v8::FunctionCallbackInfo<v8::Value>"]-> LocalObject as "v8::Local<v8::Object>"{
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);

                v8::EscapableHandleScope handle_scope(isolate);

                return handle_scope.Escape(self.This());
            })
        }
    }

    pub fn is_construct_call(self) -> bool {
        unsafe {
            cpp!([self as "v8::FunctionCallbackInfo<v8::Value>"]-> bool as "bool"{
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);

                return self.IsConstructCall();
            })
        }
    }

    fn get_return_value(self) -> ReturnValue {
        unsafe {
            cpp!([self as "v8::FunctionCallbackInfo<v8::Value>"]-> ReturnValue as "v8::ReturnValue<v8::Value>"{
                auto isolate = v8::Isolate::GetCurrent();
                v8::Locker locker(isolate);

                return self.GetReturnValue();
            })
        }
    }
}

// impl Index<i32> for FunctionCallbackInfo {
//     type Output = LocalValue;

//     fn index(&self, i: i32) -> &Self::Output {
//         unsafe {
//             return cpp!([self as "v8::FunctionCallbackInfo<v8::Value>", i as "int"]-> LocalValue as "v8::Local<v8::Value>"{
//                 auto isolate = v8::Isolate::GetCurrent();
//                 v8::Locker locker(isolate);

//                 v8::EscapableHandleScope handle_scope(isolate);

//                 return handle_scope.Escape(self[i]);
//             });
//         }
//     }
// }

// impl Iterator for FunctionCallbackInfo {
//     type Item = LocalValue;
//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }
