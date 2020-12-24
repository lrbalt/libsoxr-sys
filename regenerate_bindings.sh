#!/bin/sh
bindgen --size_t-is-usize --no-recursive-whitelist --no-prepend-enum-name --no-layout-tests \
/usr/local/include/soxr.h -o src/generated.rs
# --generate "functions,types" \