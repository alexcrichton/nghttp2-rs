extern crate cc;
extern crate ctest;

use ctest::{Field, Struct};

use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let root = PathBuf::from(env::var_os("DEP_NGHTTP2_ROOT").unwrap());
    let include = root.join("include");

    let mut cfg = ctest::TestGenerator::new();

    cfg.include(&include)
        .header("nghttp2/nghttp2.h")
        .header("nghttp2/nghttp2ver.h");

    // MSVC doesn't have `ssize_t` defined as a type.
    if target.contains("msvc") {
        match env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap().as_str() {
            "64" => cfg.define("ssize_t", Some("int64_t")),
            "32" => cfg.define("ssize_t", Some("int32_t")),
            s => panic!("unknown pointer size: {}", s),
        };
    }

    // Some nghttp2 structs have a field named `type`.
    // This is a reserved keyword in Rust, which bindgen renames to `type_`.
    cfg.rename_struct_field(|_st: &Struct, field: &Field| match field.ident() {
        "type_" => Some("type".to_string()),
        _ => None,
    });

    // Skip struct tests for opaque types.
    cfg.skip_struct(|st: &Struct| match st.ident() {
        "nghttp2_session" => true,
        "nghttp2_rcbuf" => true,
        "nghttp2_session_callbacks" => true,
        "nghttp2_option" => true,
        "nghttp2_hd_deflater" => true,
        "nghttp2_hd_inflater" => true,
        "nghttp2_stream" => true,
        _ => false,
    });

    // Skip signededness tests on MSVC since lots of enums switch,
    // and it doesn't really matter that much anyway.
    cfg.skip_signededness(move |_ty| target.contains("msvc"));

    // Generate the test files.
    let mut path = cfg.generate_files("../src/lib.rs", "ctest.rs").unwrap();

    // Path points to `ctest.rs`. We want `ctest.c`.
    path.set_file_name("ctest.c");

    // Build the generated C code.
    cc::Build::new()
        .include(&include)
        .file(&path)
        .compile("ctest");
}
