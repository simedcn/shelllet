cpp! {{
    #include "src/v8/v8.hpp"
}}

//mod kill;

//use crate::v8::handle_scope;

//use crate::v8::function_template::LocalFunctionTemplate;

use crate::v8::*;

//lazy_static! {

//   unsafe { std::mem::MaybeUninit::zeroed() };
//}

pub trait WrapperTrait {
  fn source(&self) -> StdString;
  fn name(&self) -> StdString;
  fn objtpl(&mut self) -> LocalObjectTemplate;
  fn gl(&self, obj: LocalObject);
}

cpp! {{
  struct Holder{void*p1, *p2;};

  class MyClassWrapperImpl : public Wrapper {
    public:
      Holder m_trait;

      std::string source() {
          return rust!(MCI_source [m_trait : &dyn WrapperTrait as "Holder"]
          -> StdString as "std::string" {
            m_trait.source()
           // let name = std::ffi::CString::new("World").unwrap();
          // return x
      });
        }

        std::string name() const{
          return rust!(MCI_name [m_trait : &dyn WrapperTrait as "Holder"]
          -> StdString as "std::string" {
             m_trait.name()
            //let name = std::ffi::CString::new("World").unwrap();
          //return  StdString::new(x)
      });
        }

      v8::Local<v8::ObjectTemplate> objtpl() {
          return rust!(MCI_ObjectTemplate [m_trait : &mut dyn WrapperTrait as "Holder"]
          -> LocalObjectTemplate as "v8::Local<v8::ObjectTemplate>" {
          return   m_trait.objtpl();
      });
        }

      void gl(v8::Local<v8::Object> g) {
         rust!(MCI_gl [m_trait : &mut dyn WrapperTrait as "Holder", g: LocalObject as "v8::Local<v8::Object>"] {
           m_trait.gl(g);
      });
        }
 };

 void native_run(const Holder& holder ){
  v8::Isolate* isolate = create();
  MyClassWrapperImpl mci;
  mci.m_trait = holder;

  create_context(isolate, &mci);
  gl_context.Reset();
  release(isolate);
 }
}}

pub fn run(wrapper_ptr: &dyn WrapperTrait) {
  //     let name = std::ffi::CString::new(name).unwrap();
  //   let name_ptr = name.as_ptr();

  //   let source = std::ffi::CString::new(source).unwrap();
  //   let source_ptr = source.as_ptr();
  unsafe {
   cpp!([wrapper_ptr as "Holder"] {
      native_run(wrapper_ptr);
   })
  

}
}
