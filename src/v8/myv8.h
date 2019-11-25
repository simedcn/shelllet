#pragma once
    
    #include <iostream>
    #include <v8.h>
    #include <libplatform/libplatform.h>
  
  v8::Global<v8::Context> context_;
    v8::Isolate* create(){
  
        std::unique_ptr<v8::Platform> platform = v8::platform::NewDefaultPlatform();

        v8::V8::InitializePlatform(platform.get());
        v8::V8::Initialize();
        
        v8::Isolate::CreateParams create_params;
        create_params.array_buffer_allocator = v8::ArrayBuffer::Allocator::NewDefaultAllocator();

        v8::Isolate* isolate = v8::Isolate::New(create_params);
        isolate->SetData(0xff, platform.release());

        return isolate;
    }

    void release(v8::Isolate* isolate){
        auto array_buffer_allocator = isolate->GetArrayBufferAllocator();
        auto platform = static_cast<v8::Platform*>(isolate->GetData(0xff));

        isolate->Dispose();
        v8::V8::Dispose();
        v8::V8::ShutdownPlatform();
        delete platform;
        delete array_buffer_allocator;
    }
