#![crate_type = "bin"]

use anyhow::Result;

fn main() -> Result<()> {
    let bitcode = include_bytes!(env!("LLVM_CODEGEN_PATH"));

    let ctx = llvm::Context::default();

    let module = llvm::Module::from_bytes(&ctx, bitcode)?;

    let mut builder = llvm::IRBuilder::new(&ctx);

    let i32_type = llvm::Type::get_i32(&ctx);

    let signature = llvm::FunctionType::get(i32_type, &[i32_type, i32_type, i32_type]);

    let func = llvm::Function::create(&module, "sum_codegen", signature);

    let bb = llvm::BasicBlock::create(&ctx, func, "bb1");

    builder.set_position_to_end(&bb);

    let res = builder.create_add(func.get_arg(0), func.get_arg(1), "res");

    builder.create_ret(res);

    module.dump();

    let ee = llvm::ExecutionEngine::create_for_module(&module)?;

    let f: fn(i32, i32) -> i32 = ee.find_function("sum")?;

    let res = f(5, 10);
    println!("{res}");

    Ok(())
}
