use std::env;
use std::process::Command;

fn pkgconf() {
    pkg_config::Config::new()
        .atleast_version("2.0.0")
        .probe("libsrtp2")
        .expect("pkg-config could not find libsrtp2!");
}

fn dynamic_linking(out_dir: &str) {
    pkgconf();

    bindgen::Builder::default()
        .whitelist_function("srtp_.*")
        .blacklist_function("srtp_crypto_policy_set_aes_cm_192_.*")
        .blacklist_function("srtp_crypto_policy_set_aes_gcm_.*")
        .header("wrapper.h")
        .generate()
        .expect("Failed to generate libsrtp binding")
        .write_to_file(format!("{}/bindings.rs", out_dir))
        .expect("Failed to write libsrtp binding");
}

fn static_linking(out_dir: &str) {
    bindgen::Builder::default()
        .whitelist_function("srtp_.*")
        .blacklist_function("srtp_crypto_policy_set_aes_cm_192_.*")
        .blacklist_function("srtp_crypto_policy_set_aes_gcm_.*")
        .clang_args(vec![
            "-I.",
            "-I./libsrtp/include",
            "-I./libsrtp/crypto/include",
        ])
        .header("libsrtp/include/srtp_priv.h")
        .generate()
        .expect("Failed to generate libsrtp binding")
        .write_to_file(format!("{}/bindings.rs", out_dir))
        .expect("Failed to write libsrtp binding");

    println!("cargo:rerun-if-changed=libsrtp");

    let crate_dir = &env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut configure = Command::new(format!("{}/libsrtp/configure", crate_dir));

    if cfg!(feature = "enable-debug-logging") {
        configure.arg("--enable-debug-logging");
    }

    if cfg!(feature = "enable-log-stdout") {
        configure.arg("--enable-log-stdout");
    }

    let out = configure
        .current_dir(out_dir)
        .output()
        .expect("Failed to execute `./configure` on libsrtp");
    assert!(
        out.status.success(),
        "`./configure` executed unsuccessfully on libsrtp"
    );

    let out = make_cmd::make()
        .current_dir(out_dir)
        .output()
        .expect("Failed to execute `make` on libsrtp");
    assert!(
        out.status.success(),
        "`make` executed unsuccessfully on libsrtp"
    );

    println!("cargo:rustc-link-lib=static=srtp2");
    println!("cargo:rustc-link-search={}", out_dir);
}

fn main() {
    if cfg!(feature = "pre-generated-bindings") {
        pkgconf();
        return;
    }

    let out_dir = &env::var("OUT_DIR").unwrap();
    let target = &env::var("TARGET").unwrap();

    if target.contains("msvc") {
        panic!("libsrtp doesn't support windows toolchain")
    }

    if cfg!(feature = "dynamic-linking") {
        dynamic_linking(&out_dir);
    } else {
        static_linking(&out_dir);
    }
}
