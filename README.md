# Simplify rust usage in EPICS

The full example can be found on [rust-in-epics](https://github.com/nickez/rust-in-epics)

## Step 0: Initialize project

### Create new crate

```
cargo new
...
```

### Update Cargo.toml

```
[lib]
crate-type = ["dylib", "staticlib"]

[dependencies]
epics-sys = "0.1"

[build-dependencies]
bindgen = "0.32.1"
```

## Step 1: Generate bindings for the record

### Switch EPICS to clang

Go into `configure/os/CONFIG_SITE.Common.linux-x86_64` and uncomment

```
GNU         = NO
CMPLR_CLASS = clang
CC          = clang
CCC         = clang++
```

If `compilerSpecific.h` is missing you might want to add the following line into your `RULES` file:

```
USR_CPPFLAGS = -I${EPICS_BASE}/include/compiler/clang
```

### Create build.rs script

```rust
extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::string::String;

fn main() {
    let epics_base = PathBuf::from(env::var("EPICS_BASE").unwrap_or("/home/niklas/git/epics-base".into()));

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
```

### Add the following to the top of your `lib.rs` file.

```rust
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
```

## Step 2: Create subRecord function and register it with EPICS

Add the following to your `lib.rs` file.

```rust
#[macro_use]
extern crate epics_sys;

use epics_sys::str_from_epics;

epics_register_function!(
    mySubProcess,
    mySubProcess_priv,
    subRecord,
    register_func_mySubProcess,
    pvar_func_register_func_mySubProcess
);

fn mySubProcess_priv(record: &mut subRecord) -> Result<(), ()> {
    match str_from_epics(record.name.as_ref()) {
        Ok(name) => println!("Hello from rust! name={:?}", name),
        _ => println!("Invalid UTF8 in name"),
    }
    println!("A={:.2}", record.a);
    record.val = quad(record.a);
    Ok(())

}
```

## Step 3: Configure EPICS application

Modify `Makefile` to link to crate. In this example I've put the rust crate in the Application src folder.

```
<APPName>_LDFLAGS += -pthread
<APPName>_SYS_LIBS += dl
<APPName>_LIBS += <crate-name>

<crate-name>_DIR = ${TOP}/<APPName>/src/<crate-name>/target/debug
```

Add dbd file with content:

```
function(mySubProcess)
```

Add db file using your function

```
record(sub, "HELLO") {
    field(SNAM, "mySubProcess")
}
```
