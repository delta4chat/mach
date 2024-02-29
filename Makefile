all:
	make no-std
	make std
	make post-run

no-std:
	cargo build --release

std:
	cargo rustc --release --features std --crate-type cdylib
	mv target/release/libmach_sys.dylib target/release/libmach_sys.cdylib || true
	
	cargo rustc --release --features std --crate-type dylib

	cargo rustc --release --features std --crate-type staticlib

post-run:
	bash --norc -x ./post-build.sh || true


