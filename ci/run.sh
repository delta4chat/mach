#!/usr/bin/env sh

set -ex

: "${TARGET?The `TARGET` environment variable must be set.}"
: "${RUST_VERSION?The `RUST_VERSION` environment variable must be set.}"

echo "Running tests for target: ${TARGET}"
export RUST_BACKTRACE=full
export RUST_TEST_THREADS=1
export RUST_TEST_NOCAPTURE=1
export CARGO_INCREMENTAL=0
export CARGO_CODEGEN_UNITS=1
export RUSTFLAGS="-C codegen-units=1 "

rustup override set "${RUST_VERSION}"
rustup target add "${TARGET}"

case "${TARGET}" in
    *"ios"*)
        export RUSTFLAGS="${RUSTFLAGS} -C link-args=-mios-simulator-version-min=7.0"
        rustc ./ci/deploy_and_run_on_ios_simulator.rs -o ios_cargo_runner --verbose
        if [ "${TARGET}" = "x86_64-apple-ios" ]; then
            export CARGO_TARGET_X86_64_APPLE_IOS_RUNNER="$(pwd)/ios_cargo_runner"
        fi
        if [ "${TARGET}" = "i386-apple-ios" ]; then
            export CARGO_TARGET_I386_APPLE_IOS_RUNNER="$(pwd)/ios_cargo_runner"
        fi
        ;;
    *)
        ;;
esac

sudo rm -rf /Library/Developer/CommandLineTools

# Build without std
cargo clean
cargo build --no-default-features --target "${TARGET}" -vv 2>&1 | tee build_no_std.txt

# Check that the no-std builds are not linked against a libc with default
# features or the std feature enabled:
! grep -q "default" build_no_std.txt
! grep -q "std" build_no_std.txt
# Make sure that the resulting build contains no std symbols
! find target/ -name "*.rlib" -exec nm {} \; | grep "std"

# Runs mach's run-time tests:
if [ -z "$NORUN" ]; then
    cargo test --target "${TARGET}" -vv
    cargo test --target "${TARGET}" -vv --features unstable
    cargo test --no-default-features --target "${TARGET}" -vv
fi

# Runs ctest to verify mach's ABI against the system libraries:
if [ -z "$NOCTEST" ]; then
    if [ "${RUST_VERSION}" = "nightly" ]; then
        cargo test --manifest-path mach-test/Cargo.toml --target "${TARGET}" -vv
        cargo test --no-default-features --manifest-path mach-test/Cargo.toml --target "${TARGET}" -vv
    fi
fi
