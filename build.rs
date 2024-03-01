fn main() {
    use std::time::{Instant, SystemTime};
    use std::collections::HashMap;
    use std::collections::hash_map::RandomState;
    use std::hash::{Hash, Hasher, BuildHasher};

    let env: HashMap<String, String> =
        std::env::vars().collect();

    let mut h = RandomState::new().build_hasher();
    h.write_i128(fastrand::i128(..));
    format!("{env:?}").hash(&mut h);
    format!("{:?}", Instant::now()).hash(&mut h);
    format!("{:?}", SystemTime::now()).hash(&mut h);
    h.write_u128(fastrand::u128(..));

    let nonce: u64 = h.finish();
    let nonce_file =
        format!("{}/nonce.txt", env["OUT_DIR"]);

    if let Err(e) = std::fs::write(&nonce_file, nonce.to_string()) {
        println!("cargo:warning=Cannot write nonce.txt={e:?}");
    } else {
        println!(
            "cargo:rustc-env=MACH_SYS_NONCE_RANDOM={}",
            nonce_file
        );
    }
}

