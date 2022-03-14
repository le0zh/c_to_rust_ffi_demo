#!/bin/bash

# export CPU_ARM64=aarch64-apple-darwin
export CPU_X86=x86_64-apple-darwin

rustup target add $CPU_X86

cbindgen -o lib/libc_to_rust_ffi_demo.h --lang c

cargo build  --target $CPU_X86 -v --release 

cp $(pwd)/target/$CPU_X86/release/libc_to_rust_ffi_demo.a $(pwd)/lib/libc_to_rust_ffi_demo.a
