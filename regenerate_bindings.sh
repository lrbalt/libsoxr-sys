#!/bin/sh
bindgen --size_t-is-usize --no-prepend-enum-name /usr/local/include/soxr.h -o src/generated.rs