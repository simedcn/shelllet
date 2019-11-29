
use crate::v8::*;

impl LocalData {
    pub fn new() -> Self {
        unsafe {
            return cpp!([] -> LocalData as "v8::Local<v8::Data>" {
                return v8::Local<v8::Data>();
            });
        }
    }

}

