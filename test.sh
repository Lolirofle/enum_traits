#!/bin/sh
(cd lib && cargo test && cargo test --features "no_std") &&
(cd macros && cargo test && cargo test --features "no_std") &&
(cd tests && cargo test && cargo test --features "no_std" && cargo test --features "no_std no_std_compile")
