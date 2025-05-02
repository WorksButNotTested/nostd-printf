use std::{env, path::PathBuf};

fn compile(file: &str, output: &str) {
    cc::Build::new()
        .define("_GNU_SOURCE", None)
        .opt_level(3)
        .flag("-Werror")
        .flag("-fno-stack-protector")
        .flag("-U_FORTIFY_SOURCE")
        .flag("-D_FORTIFY_SOURCE=0")
        .flag("-ffunction-sections")
        .flag("-Wa,--noexecstack")
        .include("include/")
        .file(file)
        .compile(output);
}

fn build_bindings() {
    let bindings = bindgen::Builder::default()
        .use_core()
        .blocklist_type("max_align_t")
        .header("include/printf.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

fn main() {
    println!("cargo:rerun-if-changed=include");
    println!("cargo:rerun-if-changed=src");
    compile("src/printf.c", "printf");
    build_bindings();
}
