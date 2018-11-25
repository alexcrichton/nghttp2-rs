# `libnghttp2-sys`

...

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
