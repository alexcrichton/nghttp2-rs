# nghttp2-sys

A common library for linking `nghttp2` to rust programs (also known as
libnghttp2).  By default, it uses a bundled copy of `libnghttp2`.  If that
is not desired, you can use the system's `libnghttp2` instead by not enabling
the default `vendored` feature.

## Generating bindings

Before `bindgen`:

* Copy `nghttp2ver.h.in` to `nghttp2ver.h`
* Edit `nghttp2ver.h` to remove `@FOO@`, replacing with 0

```sh
$ bindgen \
  ./nghttp2/lib/includes/nghttp2/nghttp2.h \
  -o src/lib.rs \
  --no-layout-tests \
  --distrust-clang-mangling \
  --no-prepend-enum-name \
  --rustfmt-bindings \
  --whitelist-function '.*nghttp2.*' \
  --whitelist-type '.*nghttp2.*' \
  --whitelist-var '.*nghttp2.*' \
  -- \
  -I ./nghttp2/lib/includes
```

Afterwards

* Remove `*vprintf*`
* Remove `va_list`-related things
* Add `#![allow(bad_style)]`

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

Modified by Maxime Devos (2022) (see 4(b) in LICENSE-APACHE) (TODO: is there a less intrusive way to do this?)
### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `nghttp2-sys` by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
