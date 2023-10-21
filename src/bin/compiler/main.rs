use std::{ffi::CStr, mem};

use llvm_sys::{
    core::*, execution_engine::*, ir_reader::*, prelude::LLVMModuleRef,
    support::LLVMLoadLibraryPermanently, target::*, LLVMMemoryBuffer,
};

fn main() {
    unsafe {
        let mut out_buf = std::ptr::null_mut();
        let mut err_string = std::mem::zeroed();

        if LLVMCreateMemoryBufferWithContentsOfFile(
            b"/home/fexolm/git/labradb/codegen/main.bc\0".as_ptr() as *const _,
            &mut out_buf,
            &mut err_string,
        ) != 0
        {
            let msg = CStr::from_ptr(err_string).to_str().unwrap();
            println!("{msg}");
            panic!();
        }

        let ctx = LLVMContextCreate();
        let module = LLVMModuleCreateWithNameInContext("mod".as_ptr() as *const _, ctx);

        let mut out_mod = std::ptr::null_mut();

        if LLVMParseIRInContext(ctx, out_buf, &mut out_mod, &mut err_string) != 0 {
            panic!();
        }

        let module = out_mod;

        LLVMDumpModule(module);

        LLVMLinkInMCJIT();
        LLVM_InitializeNativeTarget();
        LLVM_InitializeNativeAsmPrinter();

        let ee = {
            let mut ee = mem::MaybeUninit::uninit();
            let mut err = mem::zeroed();

            // This moves ownership of the module into the execution engine.
            if LLVMCreateExecutionEngineForModule(ee.as_mut_ptr(), module, &mut err) != 0 {
                // In case of error, we must avoid using the uninitialized ExecutionEngineRef.
                assert!(!err.is_null());
                panic!(
                    "Failed to create execution engine: {:?}",
                    CStr::from_ptr(err)
                );
            }

            ee.assume_init()
        };

        if LLVMLoadLibraryPermanently(
            b"/home/fexolm/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libstd-7b9f6349d87c69a1.so\0".as_ptr() as *const _
        ) != 0{
            panic!()
        }

        let addr = LLVMGetFunctionAddress(ee, b"sum\0".as_ptr() as *const _);

        let f: fn(i32, i32) -> i32 = mem::transmute(addr);

        let res = f(5, 10);
        println!("{res}");

        LLVMDisposeExecutionEngine(ee);
        LLVMContextDispose(ctx);
    }
}
