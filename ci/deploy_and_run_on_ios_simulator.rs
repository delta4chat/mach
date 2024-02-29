// This is a script to deploy and execute a binary on an iOS simulator.
// The primary use of this is to be able to run unit tests on the simulator and
// retrieve the results.
//
// To do this through Cargo instead, use Dinghy
// (https://github.com/snipsco/dinghy): cargo dinghy install, then cargo dinghy
// test.
//
// Source: this script is part of libc
//
// https://github.com/rust-lang/libc/blob/master/ci/ios/deploy_and_run_on_ios_simulator.rs
//
// and should be sync'ed with it when ci breaks (or periodically).

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process;
use std::process::Command;
use std::time::{Duration, Instant};

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("{} failed with: {}", stringify!($e), e),
    })
}

// Step one: Wrap as an app
fn package_as_simulator_app(crate_name: &str, test_binary_path: &Path) {
    println!("== Packaging simulator app");
    drop(fs::remove_dir_all("ios_simulator_app"));
    t!(fs::create_dir("ios_simulator_app"));
    t!(fs::copy(test_binary_path,
                Path::new("ios_simulator_app").join(crate_name)));

    let mut f = t!(File::create("ios_simulator_app/Info.plist"));
    t!(f.write_all(format!(r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE plist PUBLIC
                "-//Apple//DTD PLIST 1.0//EN"
                "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
        <plist version="1.0">
            <dict>
                <key>CFBundleExecutable</key>
                <string>{}</string>
                <key>CFBundleIdentifier</key>
                <string>com.rust.unittests</string>
            </dict>
        </plist>
    "#, crate_name).as_bytes()));
}

#[derive(Debug, Clone)]
struct SimulatorStatus {
    simulator_exists: bool,
    simulator_booted: bool,
    found_rust_sim: bool,
    stdout: String,
    error: String,
}
impl SimulatorStatus {
    fn eq(&self, other: &Self) -> bool {
        self.simulator_booted == other.simulator_booted
        && self.simulator_exists==other.simulator_exists
        && self.found_rust_sim == other.found_rust_sim
    }
}

fn check_simulator() -> SimulatorStatus {
    let output = t!(
                    Command::new("xcrun")
                    .arg("simctl")
                    .arg("list")
                    .output()
                  );

    let mut simulator_exists = false;
    let mut simulator_booted = false;
    let mut found_rust_sim   = false;

    let mut error = String::new();
    let stdout =
        if output.status.success() {
            String::from_utf8_lossy(&output.stdout)
                .to_string()
        } else {
            error.extend(
                format!(
                    "Command Exec failed: {:?}",
                    output,
                ).chars()
            );
            String::new()
        };

    for line in stdout.lines() {
        if line.contains("rust_ios") {
            if found_rust_sim {
                error.push_str("Duplicate rust_ios simulators found. Please double-check xcrun simctl list.\n");
                break;
            }
            simulator_exists = true;
            simulator_booted = line.contains("(Booted)");
            found_rust_sim = true;
        }
    }

    SimulatorStatus {
        simulator_exists,
        simulator_booted,
        found_rust_sim,
        stdout,
        error,
    }
}

fn wait_for_simulator(
    maybe_timeout: Option<Duration>
) -> Result<SimulatorStatus, String> {
    const MAX: usize = 1024;

    let started = Instant::now();
    let mut status = check_simulator();
    if status.simulator_booted {
        return Ok(status);
    }
    for i in 1..=MAX {
        std::thread::sleep(Duration::from_secs(1));

        if status.simulator_booted {
            return Ok(status);
        }

        let cur = check_simulator();
        if cur.eq(&status) {
            eprintln!("iter({i}) = no change");
        } else {
            eprintln!("iter({i}) = {status:?}");
        }
        status = cur;

        if let Some(timeout) = maybe_timeout {
            if started.elapsed() > timeout && i > 5 {
                return Err(
                    format!("timed out. {timeout:?}")
                );
            }
        }
    }

    Err(format!("max iteration {MAX} reached"))
}

// Step two: Start the iOS simulator
fn start_simulator() {
    println!("== Looking for iOS simulator");
    let status = check_simulator();

    if status.simulator_exists == false {
        println!("== Creating iOS simulator");
        Command::new("xcrun")
                .arg("simctl")
                .arg("create")
                .arg("rust_ios")
                .arg("com.apple.CoreSimulator.SimDeviceType.iPhone-SE")
                .arg("com.apple.CoreSimulator.SimRuntime.iOS-10-2")
                .check_status();
    } else if status.simulator_booted == true {
        println!("Shutting down already-booted simulator");
        Command::new("xcrun")
                .arg("simctl")
                .arg("shutdown")
                .arg("rust_ios")
                .check_status();
    }

    println!("Starting iOS simulator");
    // We can't uninstall the app (if present) as that will hang if the
    // simulator isn't completely booted; just erase the simulator instead.
    Command::new("xcrun").arg("simctl").arg("erase").arg("rust_ios").check_status();
    Command::new("xcrun").arg("simctl").arg("boot").arg("rust_ios").check_status();

    wait_for_simulator(
        Some(Duration::from_secs(10))
    ).unwrap();
}

// Step three: Install the app
fn install_app_to_simulator() {
    println!("Installing app to simulator");
    Command::new("xcrun")
            .arg("simctl")
            .arg("install")
            .arg("booted")
            .arg("ios_simulator_app/")
            .check_status();
}

// Step four: Run the app
fn run_app_on_simulator() {
    println!("== Running app");
    let output = t!(Command::new("xcrun")
                    .arg("simctl")
                    .arg("launch")
                    .arg("--console")
                    .arg("booted")
                    .arg("com.rust.unittests")
                    .output()
                   );

    println!("status: {}", output.status);
    println!("stdout --\n{}\n", String::from_utf8_lossy(&output.stdout));
    println!("stderr --\n{}\n", String::from_utf8_lossy(&output.stderr));

    let stdout = String::from_utf8_lossy(&output.stdout);
    let passed: bool = stdout.lines()
                    .any(|l| {
                        (
                            l.contains("PASSED")
                            &&
                            l.contains("tests")
                        )
                        ||
                        l.contains("test result: ok")
                    });

    println!("Shutting down simulator");
    Command::new("xcrun")
        .arg("simctl")
        .arg("shutdown")
        .arg("rust_ios")
        .check_status();
    if !passed {
        panic!("tests didn't pass");
    }
}

trait CheckStatus {
    fn check_status(&mut self);
}

impl CheckStatus for Command {
    fn check_status(&mut self) {
        println!("\trunning: {:?}", self);
        assert!(t!(self.status()).success());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <executable>", args[0]);
        process::exit(-1);
    }

    let test_binary_path = Path::new(&args[1]);
    let crate_name = test_binary_path.file_name().unwrap();

    package_as_simulator_app(crate_name.to_str().unwrap(), test_binary_path);
    start_simulator();
    install_app_to_simulator();
    run_app_on_simulator();
}
