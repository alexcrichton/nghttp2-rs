extern crate ctest;

use std::env;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env::var_os("DEP_NGHTTP2_ROOT").unwrap());
    let mut cfg = ctest::TestGenerator::new();
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
        });

    cfg.generate("../src/lib.rs", "all.rs");
}
