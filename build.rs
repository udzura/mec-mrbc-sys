extern crate cc;
use std::process::Command;

use glob::glob;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./vendor/mruby/lib/libmruby_mrbc.a");
    println!("cargo:rustc-link-search=./vendor/mruby/lib");
    cc::Build::new()
        .files(
            glob("./vendor/mruby/mrbc-src/*.c")
                .expect("cannot find c source")
                .map(|x| x.unwrap())
        )
        .warnings(false)
        .define("MRB_NO_PRESYM", "")
        .include("./vendor/mruby/include")
        .flag("-fPIC")
        .flag("-c")
        .compile("mruby_mrbc");

    Command::new("cp")
        .args(&[
            &format!("{}/libmruby_mrbc.a", out_dir),
            "./vendor/mruby/lib/libmruby_mrbc.a",
        ])
        .output()
        .expect("failed to copy libmruby_mrbc.a");

    println!("cargo:rustc-link-lib=mruby_mrbc");
    let bindings = bindgen::Builder::default()
        .header("vendor/mruby/include/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("./src/bindings.rs")
        .expect("Couldn't write bindings!");
}