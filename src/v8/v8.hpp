#pragma once

#include <iostream>
#include <string>
#include <v8.h>
#include <libplatform/libplatform.h>

v8::Isolate *create()
{

    std::unique_ptr<v8::Platform> platform = v8::platform::NewDefaultPlatform();

    v8::V8::InitializePlatform(platform.get());
    v8::V8::Initialize();

    v8::Isolate::CreateParams create_params;
    create_params.array_buffer_allocator = v8::ArrayBuffer::Allocator::NewDefaultAllocator();

    v8::Isolate *isolate = v8::Isolate::New(create_params);
    isolate->SetData(0xff, platform.release());

    return isolate;
}

void release(v8::Isolate *isolate)
{
    auto array_buffer_allocator = isolate->GetArrayBufferAllocator();
    auto platform = static_cast<v8::Platform *>(isolate->GetData(0xff));

    isolate->Dispose();
    v8::V8::Dispose();
    v8::V8::ShutdownPlatform();
    delete platform;
    delete array_buffer_allocator;
}

v8::ScriptOrigin module_script_origin(const char *resource_name, v8::Isolate *isolate)
{
    v8::ScriptOrigin origin(v8::String::NewFromUtf8(isolate, resource_name,
                                                    v8::NewStringType::kNormal)
                                .ToLocalChecked(),
                            v8::Integer::New(isolate, 0),
                            v8::Integer::New(isolate, 0),
                            v8::False(isolate),
                            v8::Local<v8::Integer>(),
                            v8::Local<v8::Value>(),
                            False(isolate),
                            False(isolate),
                            True(isolate));
    return origin;
}

class Wrapper {
    public:
    virtual std::string source() = 0;
    virtual std::string name() const = 0;

    virtual v8::Local<v8::ObjectTemplate> objtpl() = 0;

    virtual void gl(v8::Local<v8::Object> g) = 0;
};

std::string exception(v8::Isolate *isolate, v8::TryCatch* try_catch){
    v8::String::Utf8Value value(isolate, try_catch->Exception());
    return std::string(*value);
}

void createContext(v8::Isolate *isolate, v8::Global<v8::Context>* global_context,  Wrapper* wp)
{
    v8::Locker locker(isolate);
    v8::TryCatch try_catch(isolate);
    v8::Isolate::Scope isolate_scope(isolate);
    v8::HandleScope handle_scope(isolate);

    v8::Local<v8::Context> context = v8::Context::New(isolate, nullptr, wp->objtpl());
    v8::Context::Scope context_scope(context);
    global_context->Reset(isolate, context);

    wp->gl(context->Global());

    v8::ScriptOrigin origin = module_script_origin(wp->name().c_str(), isolate);

    std::cout << wp->name() << std::endl;
    std::cout << wp->source() << std::endl;
    //wp->source().c_str()
	v8::Local<v8::Module> module;
	v8::ScriptCompiler::Source source_text(v8::String::NewFromUtf8(isolate, "let x = 5;").ToLocalChecked(), origin);
	if (!v8::ScriptCompiler::CompileModule(isolate, &source_text).ToLocal(&module)) {
		//reportException(isolate_, &try_catch);
		//return std::string();
        std::string s= exception(isolate, &try_catch);
        std::cout << s;
        return;
	}

	if (module->InstantiateModule(context, [](v8::Local<v8::Context> context,
		v8::Local<v8::String> specifier,
		v8::Local<v8::Module> referrer)->v8::MaybeLocal<v8::Module> {
			v8::String::Utf8Value param(context->GetIsolate(), specifier);

			std::string filename = std::string(*param);

			v8::Isolate* isolate = context->GetIsolate();

			//const auto& itor = MODEL_WRAPPER->getModel()->files[filename];

			v8::ScriptOrigin origin = module_script_origin(filename.c_str(), context->GetIsolate());
			v8::ScriptCompiler::Source source(v8::String::NewFromUtf8(isolate, filename.c_str()).ToLocalChecked(), origin);
			return v8::ScriptCompiler::CompileModule(context->GetIsolate(), &source).ToLocalChecked();
		}).IsNothing()) {
	//	reportException(isolate_, &try_catch);
	//	return std::string();
	}
    v8::Local<v8::Value> value;
    module->Evaluate(context).ToLocal(&value);
}