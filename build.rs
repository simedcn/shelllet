extern crate cpp_build;
#[macro_use]
use std::fmt::*;

fn main() {
    let v8_sdk = std::env::var("V8_SDK").unwrap();

    let include_path = format!("{}/include", v8_sdk);

    //println!("-----------------------------------------------");
    //println!("{}", include_path);

    let lib_path = format!("{}/out.gn/x64.debug", v8_sdk);
    cpp_build::Config::new().include(include_path).build("src/v8/mod.rs");

    println!("cargo:rustc-link-search={}", lib_path);
    println!("cargo:rustc-link-lib=v8.dll");
    println!("cargo:rustc-link-lib=v8_libbase.dll");
    println!("cargo:rustc-link-lib=v8_libplatform.dll");

}