extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::string::String;

fn main() {
    let epics_base = PathBuf::from(env::var("EPICS_BASE").unwrap_or("/usr/local/epics/base".into()));

    let mut epics_include = epics_base.clone();
    epics_include.push("include");

    let mut epics_include_comp = epics_include.clone();
    epics_include_comp.push("compiler");
    epics_include_comp.push("clang");

    let mut epics_include_os = epics_include.clone();
    epics_include_os.push("os");
    epics_include_os.push("Linux");

    let mut sub_record = epics_include.clone();
    sub_record.push("subRecord.h");

    let mut registry_function = epics_include.clone();
    registry_function.push("registryFunction.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(sub_record.to_str().unwrap())
        .header(registry_function.to_str().unwrap())
        // The include directory
        .clang_arg(epics_include.to_str().unwrap())
        .clang_arg(String::from("-I") + epics_include_comp.to_str().unwrap())
        //.clang_arg(String::from("-I") + "-I/home/niklas/git/epics-base/include/os/default")
        .clang_arg(String::from("-I") + epics_include_os.to_str().unwrap())
        // long doubles cannot be converted with bindgen see #550 @ rust-lang-nursury/rust-bindgen
        .blacklist_type("max_align_t")
        .trust_clang_mangling(false)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
