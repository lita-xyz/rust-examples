#!/bin/bash

CC_delendum_unknown_baremetal_gnu=/valida-toolchain/bin/clang CFLAGS_delendum_unknown_baremetal_gnu="--sysroot=/valida-toolchain/ -isystem /valida-toolchain/include" RUSTFLAGS="-C linker=/valida-toolchain/bin/ld.lld -C link-args=/valida-toolchain/DelendumEntryPoint.o -C link-args=--script=/valida-toolchain/valida.ld -C link-args=/valida-toolchain/lib/delendum-unknown-baremetal-gnu/libc.a -C link-args=/valida-toolchain/lib/delendum-unknown-baremetal-gnu/libm.a -C link-args=--noinhibit-exec" cargo +valida build --target=delendum-unknown-baremetal-gnu --verbose
