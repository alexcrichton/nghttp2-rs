# nghttp2-sys

A common library for linking `nghttp2` to rust programs (also known as
libnghttp2).

[![Build Status](https://travis-ci.com/alexcrichton/nghttp2-rs.svg?branch=master)](https://travis-ci.com/alexcrichton/nghttp2-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/6au13yb0fkfckpjn/branch/master?svg=true)](https://ci.appveyor.com/project/alexcrichton/nghttp2-rs/branch/master)

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

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `nghttp2-sys` by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
