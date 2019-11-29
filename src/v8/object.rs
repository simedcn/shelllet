
use crate::v8::LocalName;
use crate::v8::*;
use crate::v8::bool;


impl LocalObject {
  pub  fn create_data_property(
        &self,
        gl_context: GlobalContext,
        name: LocalName,
        value: LocalValue,
    ) -> Result<bool, &str> {
        let result = unsafe {
            cpp!([self as "v8::Local<v8::Object>*", gl_context as "v8::Global<v8::Context>",
            name as "v8::Local<v8::Name>",
            value as "v8::Local<v8::Value>" ] -> MaybeBool as "v8::Maybe<bool>"{
                auto isolate = v8::Isolate::GetCurrent();
               //  v8::Locker locker(isolate);
               //  v8::HandleScope handle_scope(isolate);
                // std::cout << "#name:" << cstring << std::endl;

                v8::Local<v8::Context> context = v8::Local<v8::Context>::New(isolate, gl_context.Get(isolate));
                //v8::Context::Scope context_scope(context);


               return (*self)->CreateDataProperty(context, name, value);
            })
        };

        if result.is_nothing() {
            Err("Some error message")
        } else {
            Ok(result.to_checked())
        }
    }

    pub fn value(&self) -> LocalValue {
        unsafe {
            return cpp!([self as "v8::Local<v8::Object>"]-> LocalValue as "v8::Local<v8::Value>"{
               // auto isolate = v8::Isolate::GetCurrent();
              //  v8::EscapableHandleScope handle_scope(isolate);
            //    v8::Local<v8::Data> data = self;
                return self;
                //return handle_scope.Escape( self);
            });
        }
    }
}


impl MaybeLocalObject{
    pub fn is_empty(self) -> bool{
        unsafe {
            cpp!([self as "v8::MaybeLocal<v8::Object>"] -> bool as "bool" {
               return self.IsEmpty();
            })
        }
    }
    pub fn to_local_checked(&mut self) -> LocalObject{
        unsafe {
            cpp!([self as "v8::MaybeLocal<v8::Object>*"] -> LocalObject as "v8::Local<v8::Object>" {
               return (*self).ToLocalChecked();
            })
        }
    }
}