#!/bin/bash

build() {
	build_without_std $*
	build_with_std $*
	post_build $*
}

doc() {
	cargo doc --all-features --verbose $*
}
doc_open() {
	doc --open $*
}

build_without_std() {
	cargo build --release $*
}

build_with_std() {
	cargo rustc --release --features std --crate-type cdylib $*
	mv target/release/libmach_sys.dylib target/release/libmach_sys.cdylib || mv target/release/libmach_sys.so target/release/libmach_sys.cdylib.so || true
	
	cargo rustc --release --features std --crate-type dylib $*
	
	cargo rustc --release --features std --crate-type staticlib $*
}

post_build() {
	bash --norc -x ./post-build.sh $* || true
}

do_test() {
	# always need std
	cargo test --release --features std $*
	post_test $*
}

post_test() {
	echo test finish $*
}

ios_build() {
	bash --norc -x ./ios-build.sh $*
}

default() {
	build $*
}

trap exit SIGINT

set -e
set -x

case "$1" in
	build)    shift; build $*;;
	nostd)    shift; build_without_std $*;;
	std)      shift; build_with_std $*;;
	test)     shift; do_test $*;;
	ios)      shift; ios_build $*;;
	doc)      shift; doc $*;;
	doc-open) shift; doc_open $*;;
	*)        default $*;;
esac

