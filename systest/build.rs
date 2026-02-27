extern crate cc;
extern crate ctest;

use ctest::{Field, Struct};

use std::{
    env,
    path::{Path, PathBuf},
};

fn generate_ctest(target: &str, root: &Path) -> PathBuf {
    let include = root.join("include");
    let target = target.to_string();

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

    // The C library may use typedefs with anonymous structs, but all structs will
    // have a typedef that allows us to elide the `struct` keyword from the C type.
    // Call `rename_struct_ty` etc. to remove the `struct` prefix in the generated tests.
    cfg.rename_struct_ty(move |ty| Some(ty.to_string()));
    cfg.rename_union_ty(move |ty| Some(ty.to_string()));

    // Skip signededness tests on MSVC since lots of enums switch,
    // and it doesn't really matter that much anyway.
    cfg.skip_signededness(move |_ty| target.contains("msvc"));

    // Generate the test files.
    let mut path = cfg.generate_files("../src/lib.rs", "ctest.rs").unwrap();
    path.pop(); // Path ends with `ctest.rs`. Pop the filename to get the directory.
    path
}

fn compile_ctest(target: &str, root: &Path, out_dir: &Path) {
    let include = root.join("include");
    let path = out_dir.join("ctest.c");

    let mut cfg = cc::Build::new();
    cfg.include(&include).file(&path);

    // MSVC doesn't have `ssize_t` defined as a type.
    if target.contains("msvc") {
        match env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap().as_str() {
            "64" => cfg.define("ssize_t", Some("int64_t")),
            "32" => cfg.define("ssize_t", Some("int32_t")),
            s => panic!("unknown pointer size: {}", s),
        };
    }

    cfg.compile("ctest")
}

fn main() {
    let target = env::var("TARGET").unwrap();
    let root = PathBuf::from(env::var_os("DEP_NGHTTP2_ROOT").unwrap());
    let out_dir = generate_ctest(&target, &root);
    compile_ctest(&target, &root, &out_dir)
}
