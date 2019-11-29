

use crate::v8::*;

impl LocalContext{

}



impl GlobalContext{
    pub fn new() -> Self {
        unsafe {
            return cpp!([] -> GlobalContext as "v8::Global<v8::Context>" {
                return v8::Global<v8::Context>();
            });
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            cpp!([self as "v8::Global<v8::Context>*"] {
               self->Reset();
            });
        }
    }
}

