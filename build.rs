use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=codegen");
    println!("cargo:rerun-if-env-changed=OUT_DIR");
    println!("cargo:rerun-if-env-changed=RUSTUP_HOME");
    println!("cargo:rerun-if-env-changed=RUSTUP_TOOLCHAIN");

    let target_dir = std::env::var("OUT_DIR").expect("Expected OUT_DIR env var to bet set");

    println!("cargo:rustc-env=LLVM_CODEGEN_PATH={target_dir}/lib.bc");

    println!("{target_dir}");

    let status = Command::new("rustc")
        .arg("codegen/lib.rs")
        .arg("--crate-type=lib")
        .arg("--emit=llvm-bc")
        .arg(format!("--out-dir={target_dir}"))
        .status()
        .unwrap();

    if !status.success() {
        panic!();
    }
}
