#![crate_type = "bin"]

use std::{
    ffi::{CStr, CString},
    mem,
};

use llvm_sys::{
    core::*, execution_engine::*, ir_reader::*, support::LLVMLoadLibraryPermanently, target::*,
};

fn main() {
    let bitcode = include_bytes!(env!("LLVM_CODEGEN_PATH"));

    unsafe {
        let mut err_string = std::mem::zeroed();

        let buf = LLVMCreateMemoryBufferWithMemoryRange(
            bitcode.as_ptr() as *const _,
            bitcode.len(),
            b"bitcode_buf\0".as_ptr() as *const _,
            0,
        );

        let ctx = LLVMContextCreate();
        let _module = LLVMModuleCreateWithNameInContext("mod".as_ptr() as *const _, ctx);

        let mut out_mod = std::ptr::null_mut();

        if LLVMParseIRInContext(ctx, buf, &mut out_mod, &mut err_string) != 0 {
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

        if LLVMLoadLibraryPermanently(std::ptr::null()) != 0 {
            panic!()
        }

        let addr = LLVMGetFunctionAddress(ee, b"sum\0".as_ptr() as *const _);

        {
            let mut err = mem::zeroed();
            if LLVMExecutionEngineGetErrMsg(ee, &mut err) != 0 {
                panic!("Failed to execute kernel: {:?}", CStr::from_ptr(err));
            }
        }

        let f: fn(i32, i32) -> i32 = mem::transmute(addr);

        let res = f(5, 10);
        println!("{res}");

        LLVMDisposeExecutionEngine(ee);
        LLVMContextDispose(ctx);
    }
}
