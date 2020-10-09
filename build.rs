use std::env;

fn main() {
    let out_dir = &env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=wrapper.h");

    let mut bindgen_builder = bindgen::Builder::default()
        .clang_args(&["-I./libsrtp/include"])
        .header("wrapper.h")
        .whitelist_function("(srtp|SRTP|srtcp|SRTCP)_.*")
        .whitelist_type("(srtp|SRTP|srtcp|SRTCP)_.*")
        .whitelist_var("(srtp|SRTP|srtcp|SRTCP)_.*");

    if !cfg!(feature = "enable-openssl") {
        bindgen_builder = bindgen_builder.blacklist_item(".*(192|gcm|GCM).*")
    }

    bindgen_builder
        .generate()
        .expect("Failed to generate libsrtp2 binding")
        .write_to_file(format!("{}/bindings.rs", out_dir))
        .expect("Failed to write libsrtp2 binding");

    find_libsrtp2(out_dir);
}

#[cfg(all(target_env = "msvc", feature = "build"))]
fn find_libsrtp2(_out_dir: &str) {
    compile_error!("building libsrtp2 from source is not supported on windows");
}

#[cfg(all(target_env = "msvc", not(feature = "build")))]
fn find_libsrtp2(_out_dir: &str) {
    vcpkg::find_package("libsrtp")
        .expect("Failed to find libsrtp via vcpkg");
}

#[cfg(all(not(target_env = "msvc"), not(feature = "build")))]
fn find_libsrtp2(_out_dir: &str) {
    pkg_config::Config::new()
        .atleast_version("2.3.0")
        .statik(true)
        .probe("libsrtp2")
        .expect("Failed to find libsrtp2 via pkg-config");
}

#[cfg(all(not(target_env = "msvc"), feature = "build"))]
fn find_libsrtp2(out_dir: &str) {
    use std::process::Command;

    let crate_dir = &env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut configure = Command::new(format!("{}/libsrtp/configure", crate_dir));

    if std::env::var_os("SRTP2_SYS_DEBUG_LOGGING").is_some() {
        configure.arg("--enable-debug-logging");

        match std::env::var("SRTP2_SYS_DEBUG_LOG_FILE") {
            Ok(path) => configure.arg(format!("--with-log-file={}", path)),
            Err(_) => configure.arg("--enable-log-stdout"),
        };
    }

    #[cfg(feature = "enable-openssl")]
    {
        let openssl_include = env::var("DEP_OPENSSL_INCLUDE").unwrap();

        configure
            .arg("--enable-openssl")
            .env("crypto_CFLAGS", format!("-I{}", openssl_include))
            // Below are to fake the libsrtp build system
            // so it believes we have proper openssl library.
            // The library itself will be provided by the `openssl-sys` crate
            // but at this point we can't know where it is.
            .env("crypto_LIBS", " ")
            .env("ac_cv_search_EVP_EncryptInit", " ")
            .env("ac_cv_search_EVP_aes_128_ctr", " ")
            .env("ac_cv_search_EVP_aes_128_gcm", " ");
    }

    let out = configure
        .current_dir(out_dir)
        .output()
        .expect("Failed to execute `./configure` on libsrtp");
    assert!(
        out.status.success(),
        "`./configure` executed unsuccessfully on libsrtp\nSTDOUT: {}\nSTDERR: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    let out = make_cmd::make()
        .arg("libsrtp2.a")
        .current_dir(out_dir)
        .output()
        .expect("Failed to execute `make` on libsrtp");
    assert!(
        out.status.success(),
        "`make` executed unsuccessfully on libsrtp\nSTDOUT: {}\nSTDERR: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    println!("cargo:rerun-if-changed=libsrtp");
    println!("cargo:rustc-link-lib=static=srtp2");
    println!("cargo:rustc-link-search={}", out_dir);
}
