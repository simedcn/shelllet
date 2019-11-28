cpp_class!(pub unsafe struct StdString as "std::string");

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
}
