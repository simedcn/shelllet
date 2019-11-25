
cpp!{{
    #include "src/v8/myv8.h"
    using namespace std;
}}

//mod kill;

//use crate::v8::handle_scope;


use crate::v8::object_template::LocalObjectTemplate;
use crate::v8::function_template;
use crate::v8::object_template;

pub trait MyTrait {
    fn compute_value(&self, x: i32) -> i32;
}

cpp!{{
  

    void createContext(v8::Isolate* isolate, MyClass *callback){
        v8::TryCatch try_catch(isolate);
	v8::Locker locker(isolate);
	v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);
    v8::Local<v8::ObjectTemplate> object_template =  callback.computeValue();

	v8::Local<v8::Context> local = v8::Context::New(isolate, nullptr, object_template);
	v8::Context::Scope context_scope(local);
	context_.Reset(isolate, local);
	{
	//	std::unique_ptr<wrapped::Console> console = std::make_unique<wrapped::Console>();
	//	v8::Local<v8::Name> name = v8::Local<v8::Name>::New(isolate_, v8::String::NewFromUtf8(isolate_, console->name().c_str()).ToLocalChecked());
	//	local->Global()->DefineOwnProperty(local, name, console->newInstance(isolate_, context_));
	}

    }

}}

cpp! {{
    class MyClassImpl : public MyClass {
      public:
        TraitPtr2 m_trait;
        v8::Local<v8::ObjectTemplate>  computeValue() const /*override*/ {
           return rust!(MCI_computeValue [m_trait : &dyn MyTrait as "TraitPtr2"]
               -> LocalObjectTemplate as "v8::Local<v8::ObjectTemplate>" {
              m_trait.compute_value()
           });
       }
   };
}}


pub fn main3(){
   unsafe{
        cpp!([] {

        v8::Isolate* isolate = create();
    
        MyClassImpl mci;
        mci.m_trait = inst_ptr;

        createContext(isolate, &mci);
    
        release(isolate);
       
    })
};
}


struct MyTraitImpl {
   
}
impl MyTrait for MyTraitImpl {
    fn compute_value(&self, x: i32) -> LocalObjectTemplate {
        let mut f = function_template::LocalFunctionTemplate::new(inst_ptr);
        let mut o = object_template::LocalObjectTemplate::new();
        o.set2("sss".to_owned(), f);
        
    }
}


pub fn createTempl(){
    let mut inst = function_template::CC{};
   let mut inst_ptr = &mut inst;

   let mut f = function_template::LocalFunctionTemplate::new(inst_ptr);

  
    //let handle_scope = handle_scope:: HandleScope::new(isolate);
   // eval( o);
}
pub fn main2() {
    let name = std::ffi::CString::new("World").unwrap();
    let name_ptr = name.as_ptr();


    let isolate = unsafe {
        cpp!([name_ptr as "const char *"] -> *mut u32 as "v8::Isolate *" {
            std::cout << "Hello, " << name_ptr << std::endl;

            std::unique_ptr<v8::Platform> platform = v8::platform::NewDefaultPlatform();

            v8::V8::InitializePlatform(platform.get());
            v8::V8::Initialize();
            
            v8::Isolate::CreateParams create_params;
	        create_params.array_buffer_allocator = v8::ArrayBuffer::Allocator::NewDefaultAllocator();

            v8::Isolate* isolate = v8::Isolate::New(create_params);
            isolate->SetData(0xff, platform.release());
            
            
            return isolate;
        })
    };
   // assert_eq!(r, 42);

    run_code(isolate);
   unsafe{
    cpp!([isolate as "v8::Isolate*"] {

        auto array_buffer_allocator = isolate->GetArrayBufferAllocator();
        auto platform = static_cast<v8::Platform*>(isolate->GetData(0xff));

        isolate->Dispose();
        v8::V8::Dispose();
        v8::V8::ShutdownPlatform();
        delete platform;
        delete array_buffer_allocator;
    } )
   };



cpp! {{
    struct MyClass {
        virtual int computeValue(int) const = 0;
    };
    int operate123(MyClass *callback) { return callback->computeValue(123); }

    struct TraitPtr2 {
         void *a;
         void*b;
         };
}}

struct MyTraitImpl {
    x: i32,
}
impl MyTrait for MyTraitImpl {
    fn compute_value(&self, x: i32) -> i32 {
        println!("---------------{}", self.x);
        self.x + x
    }
}
let inst = MyTraitImpl { x: 333 };
let inst_ptr: &dyn MyTrait = &inst;
let i = unsafe {
    cpp!([inst_ptr as "TraitPtr2"] -> u32 as "int" {
        MyClassImpl mci;
        mci.m_trait = inst_ptr;
        return operate123(&mci);
    })
};
assert_eq!(i, 123 + 333);


println!("############{}", std::mem::size_of_val(&inst_ptr));

}

fn run_code(isolate : *mut u32){

    // unsafe{
    //     cpp!([isolate as "v8::Isolate*"]{
    //         v8::Locker locker(isolate);
	//         v8::Isolate::Scope isolate_scope(isolate);
    //      //   v8::HandleScope handle_scope(isolate);
            
    //         std::cout << "locker....." << std::endl;
    //         std::cout << "isolate:" << isolate << std::endl;
    //     })

      

    // };

    
    
}
cpp!{{
    v8::ScriptOrigin module_script_origin(const char* resource_name, v8::Isolate* isolate) {
        v8::ScriptOrigin origin(v8::String::NewFromUtf8(isolate, resource_name,
            v8::NewStringType::kNormal).ToLocalChecked(),
            v8::Integer::New(isolate, 0),
            v8::Integer::New(isolate, 0),
            v8::False(isolate),
            v8::Local<v8::Integer>(),
            v8::Local<v8::Value>(),
            False(isolate),
            False(isolate),
            True(isolate)
        );
        return origin;
    }
}}

cpp!{{

    void instantiateModule(v8::Local<v8::Module> module, v8::Local<v8::Context> local){
        // if (module->InstantiateModule(local, [](v8::Local<v8::Context> context,
        //     v8::Local<v8::String> specifier,
        //     v8::Local<v8::Module> referrer)->v8::MaybeLocal<v8::Module> {
        //         v8::String::Utf8Value param(context->GetIsolate(), specifier);
    
        //         std::string filename = std::string(*param);
    
        //         v8::Isolate* isolate = context->GetIsolate();
    
        //         //const auto& itor = MODEL_WRAPPER->getModel()->files[filename];
        //         v8::ScriptOrigin origin = module_script_origin( filename.c_str(), isolate);
    
        //         v8::ScriptCompiler::Source source(v8::String::NewFromUtf8(isolate, filename.c_str()).ToLocalChecked(), origin);
        //         return v8::ScriptCompiler::CompileModule(context->GetIsolate(), &source).ToLocalChecked();
        //     }).IsNothing()) {
        //         std::cout <<"err" << std::endl;
        //     }
    }

}}
cpp! {{

    void callJS(v8::Isolate* isolate, v8::Local<v8::Context> local){
        v8::Local<v8::Module> module;

        v8::ScriptOrigin origin = module_script_origin( "resource_name", isolate);
	v8::ScriptCompiler::Source source_text(v8::String::NewFromUtf8(isolate, "sss();").ToLocalChecked(), origin);
	if (!v8::ScriptCompiler::CompileModule(isolate, &source_text).ToLocal(&module)) {
	    std::cout <<"err" << std::endl;
        }
    
        instantiateModule(module, local);

	v8::Local<v8::Value> value;

    module->Evaluate(local).ToLocal(&value);
    
    }
}}

fn eval(isolate: *mut u32,obj : LocalObjectTemplate){
    unsafe{

	cpp!([isolate as "v8::Isolate*", obj as "v8::Local<v8::ObjectTemplate>"] {
        v8::Locker locker(isolate);
        v8::Isolate::Scope isolate_scope(isolate);
        v8::Local<v8::Context> local = v8::Context::New(isolate, nullptr, obj);
        v8::Context::Scope context_scope(local);

        callJS(isolate, local);
    })
 
            
    
    };
}