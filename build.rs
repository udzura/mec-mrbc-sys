extern crate cc;
use glob::glob;

fn main() {
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
        .out_dir("./vendor/mruby/lib")
        .compile("mruby_mrbc");

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