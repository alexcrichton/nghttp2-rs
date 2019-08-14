extern crate ctest;

use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let root = PathBuf::from(env::var_os("DEP_NGHTTP2_ROOT").unwrap());
    let mut cfg = ctest::TestGenerator::new();

    // Apparently MSVC doesn't have `ssize_t` defined as a type
    if target.contains("msvc") {
        match env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap().as_str() {
            "64" => { cfg.define("ssize_t", Some("int64_t")); }
            "32" => { cfg.define("ssize_t", Some("int32_t")); }
            s => panic!("unknown pointer size: {}", s),
        }
    }

    cfg.header("nghttp2/nghttp2.h")
        .include(root.join("include"))
        .type_name(|n, _is_struct, _is_union| {
            n.to_string()
        })
        .skip_struct(|name| {
            // TODO: dox
            name == "nghttp2_session" ||
            name == "nghttp2_rcbuf" ||
            name == "nghttp2_session_callbacks" ||
            name == "nghttp2_option" ||
            name == "nghttp2_hd_deflater" ||
            name == "nghttp2_hd_inflater" ||
            name == "nghttp2_stream"
        })
        .field_name(|_struct, field| {
            if field == "type_" {
                "type".to_string()
            } else {
                field.to_string()
            }
        })
        .skip_signededness(move |_ty| {
            // skip signededness checks on MSVC since lots of enums switch, and
            // it doesn't really matter that much anyway
            target.contains("msvc")
        });

    cfg.generate("../src/lib.rs", "all.rs");
}
