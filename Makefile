all:
	make build-without-std
	make build-with-std
	make post-build

build-without-std:
	cargo build --release

build-with-std:
	cargo rustc --release --features std --crate-type cdylib
	mv target/release/libmach_sys.dylib target/release/libmach_sys.cdylib || mv target/release/libmach_sys.so target/release/libmach_sys.cdylib.so || true
	
	cargo rustc --release --features std --crate-type dylib
	
	cargo rustc --release --features std --crate-type staticlib

post-build:
	bash --norc -x ./post-build.sh || true

test:
	# no need to use std
	cargo test --release
	make post-test

post-test:
	echo test finish

ios-build:
	bash --norc -x ./ios-build.sh

