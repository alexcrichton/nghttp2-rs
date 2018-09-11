extern crate nghttp2_sys;

extern {
    fn nghttp2_version(input: i32) -> *const u8;
}

fn main() {
    unsafe {
        nghttp2_version(0);
    }
}
