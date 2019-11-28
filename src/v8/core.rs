cpp! {{
    #include "src/v8/v8.hpp"
    using namespace std;
}}

//mod kill;

//use crate::v8::handle_scope;

use crate::v8::context::GlobalContext;
use crate::v8::function_template::LocalFunctionTemplate;
use crate::v8::object::LocalObject;
use crate::v8::object_template::LocalObjectTemplate;

//lazy_static! {
 
  //   unsafe { std::mem::MaybeUninit::zeroed() };
//}

fn test2() {
  //let global_context :GlobalContext;
}

cpp! {{
    struct Holder{void*p1, *p2;};
    class MyClassWrapperImpl : public Wrapper {
      public:
      Holder m_trait;

          std::string source() {
            return rust!(MCI_source [m_trait : &dyn WrapperTrait as "Holder"]
            -> String as "std::string" {
               let x = m_trait.source();
              let name = std::ffi::CString::new("World").unwrap();
             return x
        });
          }

          std::string name() {
            return rust!(MCI_name [m_trait : &dyn WrapperTrait as "Holder"]
            -> String as "std::string" {
               let x = m_trait.name();
              let name = std::ffi::CString::new("World").unwrap();
            return  x
        });
          }

          v8::Local<v8::ObjectTemplate> objtpl() {
            return rust!(MCI_ObjectTemplate [m_trait : &dyn WrapperTrait as "Holder"]
            -> LocalObjectTemplate as "v8::Local<v8::ObjectTemplate>" {
            return   m_trait.objtpl();
        });
          }

        void gl(v8::Local<v8::Object> g) {}
   };
}}

pub trait WrapperTrait {
  fn source(&self) -> String;
  fn name(&self) -> String;
  fn objtpl(&self) -> LocalObjectTemplate;
  fn gl(&self, obj: LocalObject);
}

pub fn run(wrapper_ptr: &dyn WrapperTrait) {
  //     let name = std::ffi::CString::new(name).unwrap();
  //   let name_ptr = name.as_ptr();

  //   let source = std::ffi::CString::new(source).unwrap();
  //   let source_ptr = source.as_ptr();
  let global_context: GlobalContext = GlobalContext::new();
  unsafe {
    cpp!([global_context as "v8::Global<v8::Context>", wrapper_ptr as "Holder"] {

        v8::Isolate* isolate = create();
        MyClassWrapperImpl mci;
        mci.m_trait = wrapper_ptr;

        createContext(isolate, const_cast<v8::Global<v8::Context> *>(&global_context), &mci);
        release(isolate);
    })
  };
}
