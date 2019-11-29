pub mod core;
pub mod function_template;
pub mod object_template;
pub mod object;
pub mod context;
pub mod data;
pub mod function_callback_Info;
pub mod string;
mod name;
mod value;

mod bool;

cpp_class!(pub unsafe struct MaybeBool as "v8::Maybe<bool>");

cpp_class!(pub unsafe struct LocalContext as "v8::Local<v8::Context>");

cpp_class!(pub unsafe struct LocalName as "v8::Local<v8::Name>");

cpp_class!(pub unsafe struct MaybeLocalObject as "v8::MaybeLocal<v8::Object>");
cpp_class!(pub unsafe struct LocalObject as "v8::Local<v8::Object>");

cpp_class!(pub unsafe struct LocalObjectTemplate as "v8::Local<v8::ObjectTemplate>");
cpp_class!(pub unsafe struct LocalFunctionTemplate as "v8::Local<v8::FunctionTemplate>");
cpp_class!(pub unsafe struct GlobalContext as "v8::Global<v8::Context>");
cpp_class!(pub unsafe struct LocalValue as "v8::Local<v8::Value>");
cpp_class!(pub unsafe struct LocalData as "v8::Local<v8::Data>");

pub trait Base{

    fn created_object_template(self, tpl: &mut LocalObjectTemplate);
    fn create_function_template(self, tpl: &mut LocalObjectTemplate);

    fn name(self) ->String;

}