all:
	make no-std
	make std
	make post-run

no-std:
	cargo build --release

std:
	cargo rustc --release --features std --crate-type cdylib --crate-type staticlib

post-run:
	bash --norc -x ./post-build.sh || true


