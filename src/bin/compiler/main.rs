#![crate_type = "bin"]

use anyhow::Result;

fn main() -> Result<()> {
    let bitcode = include_bytes!(env!("LLVM_CODEGEN_PATH"));

    let ctx = llvm::Context::default();

    let module = llvm::Module::from_bytes(&ctx, bitcode)?;

    let ee = llvm::ExecutionEngine::create_for_module(&module)?;

    let f: fn(i32, i32) -> i32 = ee.find_function("sum")?;

    let res = f(5, 10);
    println!("{res}");

    Ok(())
}
