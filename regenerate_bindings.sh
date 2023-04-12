#!/bin/sh
bindgen --size_t-is-usize --no-prepend-enum-name /usr/include/soxr.h -o src/generated.rs
