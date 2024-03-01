# mach-sys: forked from original "mach", and merge from "mach2" & "machx".

[![github](https://img.shields.io/badge/GitHub-mach-sys?logo=github)](https://github.com/delta4chat/mach)
[![docs.rs](https://img.shields.io/docsrs/mach-sys/latest)](https://docs.rs/mach-sys)
[![crates.io](https://img.shields.io/crates/v/mach-sys.svg)](https://crates.io/crates/mach-sys)

A Rust interface to the **user-space** API of the Mach 3.0 kernel exposed in
`/usr/include/mach` that underlies macOS and is linked via `libSystem` (and
`libsystem_kernel`).

This library does not expose the **kernel-space** API of the Mach 3.0 kernel
exposed in
`SDK/System/Library/Frameworks/Kernel.framework/Versions/A/Headers/mach`. 

That is, if you are writing a kernel-resident device drivers or some other
kernel extensions you have to use something else. The user-space kernel API is
often API-incompatible with the kernel space one, and even in the cases where
they match, they are sometimes ABI incompatible such that using this library
would have **undefined behavior**.

```
(notes from machx developers:)
1. due to the lack of maintenance of `mach2` and `mach`, a lot of APIs are missing, so I maintain this version for my own use.
2. via bindgen as well as manual fixes.
```

## Usage

Add the following to your `Cargo.toml` to conditionally include mach on those
platforms that support it.

```toml
[target.'cfg(target_vendor="apple")'.dependencies]

# use alias for migration from exists project:
mach = { package = "mach-sys", version = "0.5" }

# or without alias:
mach-sys = "0.5"
```

Available crate feature:

* **unstable** (disabled by default): Exposes newly changed APIs. Enabling this may
  bring breaking changes (see the breaking change policy).

### Breaking change policy

We do the following steps when an item is changed/removed on latest toolchain:

1. Flagging an existing one as **Deprecated**.
2. Declare a new one (by default).
3. Forever kept older one.

4. if two items have equivalents value, then type alias to new one, or internal function calling new one.
5. otherwise, if two items have different value or behavior, then both will exists together.
6. or, in cases, if the older one is _really_ sucks, then After a month or more since releasing a new version that contains that _remove an older one_.

For instance (equivalent), if function `FUNC` will changes it's _return type_ from `u32` to `u64`, a new function will be created, then older function just call it and convert it's return type (`u64 as u32`). in most of cases, no more changes will be taken, and both of these will be kept forever.

For instance (non-equivalent), if const `FOO` value is changed from `3` to `4`,
we expose the newer one, i.e. `4`.
So the users should notice the change on the first release since deprecating.
Or, After a month or more, **may or may not**, all the users should migration/following this change.

## Examples

Examples can be found in the [examples](./examples) directory of this repository.

Since [`examples/dump_process_registers.rs`](./examples/dump_process_registers.rs) makes use of the `task_for_pid()` function, which requires elevated privileges, it is necessary to disable System Integrity Protection (SIP) and to be part of the `admin` or `_developer` group in order to run the example. However, do note that disabling SIP is in no way encouraged, and should only be done for development/debugging purposes.

1. Reboot macOS in recovery mode.
2. Click on `Options`.
3. Log in to your user.
4. In the menu click on `Utilities` and then `Terminal`.
5. In the terminal type the following command to disable SIP: `csrutil disable` (`csrutil enable` to re-enable SIP).
6. Reboot your machine.

To run the example, build it as follows:

```
cargo b --example dump_process_registers
```

Then run it using `sudo`:

```
sudo ./target/debug/examples/dump_process_registers
```

## Platform support

The following table describes the current CI set-up:

| Target                  | Min. Rust | XCode           | build | ctest | run |
|-------------------------|-----------|-----------------|-------|-------|-----|
| `x86_64-apple-darwin`   | 1.33.0    | 10.3.0 - 13.1.0 | ✓     | ✓     | ✓   |
| `aarch64-apple-darwin`  | nightly   | 13.1.0          | ✓     | -     | -   |
| `aarch64-apple-ios`     | nightly   | 13.1.0          | ✓     | -     | -   |
| `aarch64-apple-ios-sim` | nightly   | 13.1.0          | ✓     | -     | -   |
| `x86_64-apple-ios`      | nightly   | 13.1.0          | ✓     | -     | -   |

## License

This project is licensed under either of

* Option 1: [GNU General Public License Version 3.0](https://www.gnu.org/licenses/gpl-3.0.html), or
* Option 2: [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in `mach-sys` by you,
as defined in the dual licensing under `GPL-3.0` OR `Apache-2.0`, shall be
both licensed as above, without any additional terms or conditions.

To locally test the library, run:

```
$ ./tomake.sh nostd --target=x86_64-apple-darwin --any-args-accept-by-rustc
$ ./tomake.sh test --target=<your_platform> --any-args-accept-by-cargo
$ echo "for other args please see the bash script named tomake.sh"
```

where you can replace the `--target` with the target you
want to test (e.g. `aarch64-apple-darwin`) and use [Rustup](https://rustup.rs/) if you need to use different Rust version, (e.g. `stable`, `1.33.0`, etc.).

[crates.io]: https://crates.io/crates/mach-sys
[Latest Version]: https://img.shields.io/crates/v/mach-sys.svg
[docs]: https://docs.rs/mach-sys/badge.svg
[docs.rs]: https://docs.rs/mach-sys

