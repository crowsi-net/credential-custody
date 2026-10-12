//! Compile a socket-free native fixture without a scripting runtime.

pub fn build(path: &std::path::Path) {
    let compiler = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let source =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/custody_fixture.rs");
    assert!(
        std::process::Command::new(compiler)
            .arg("--edition=2024")
            .arg(source)
            .arg("-o")
            .arg(path)
            .status()
            .expect("compile native test helper")
            .success()
    );
}
