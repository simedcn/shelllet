cpp_class!(pub(crate) unsafe struct LocalContext as "v8::Local<v8::Context>");


impl LocalContext{

}

cpp_class!(pub(crate) unsafe struct GlobalContext as "v8::Global<v8::Context>");


impl GlobalContext{
    pub fn new() -> Self {
        unsafe {
            return cpp!([] -> GlobalContext as "v8::Global<v8::Context>" {
                return v8::Global<v8::Context>();
            });
        }
    }
}

