use crate::module::Module;
use anyhow::{bail, Result};
use llvm_sys::{
    execution_engine::{
        LLVMCreateExecutionEngineForModule, LLVMDisposeExecutionEngine,
        LLVMExecutionEngineGetErrMsg, LLVMExecutionEngineRef, LLVMGetFunctionAddress,
        LLVMLinkInMCJIT,
    },
    support::LLVMLoadLibraryPermanently,
    target::{LLVM_InitializeNativeAsmPrinter, LLVM_InitializeNativeTarget},
};
use std::{ffi::CStr, ffi::CString, marker::PhantomData, mem};

pub struct ExecutionEngine<'a> {
    handle: LLVMExecutionEngineRef,
    module: PhantomData<&'a Module<'a>>,
}

impl<'a> ExecutionEngine<'a> {
    pub fn create_for_module(module: &'a Module<'a>) -> Result<Self> {
        unsafe {
            LLVMLinkInMCJIT();
            LLVM_InitializeNativeTarget();
            LLVM_InitializeNativeAsmPrinter();

            if LLVMLoadLibraryPermanently(std::ptr::null()) != 0 {
                bail!("Failed to link current executable as library");
            }

            let mut handle = mem::MaybeUninit::uninit();
            let mut err = mem::zeroed();

            if LLVMCreateExecutionEngineForModule(handle.as_mut_ptr(), module.handle, &mut err) != 0
            {
                let err = CStr::from_ptr(err).to_str()?;
                bail!("Failed to create execution engine: {err}");
            }

            let handle = handle.assume_init();

            Ok(ExecutionEngine {
                module: PhantomData,
                handle,
            })
        }
    }

    pub fn find_function<F: Sized>(&self, name: &str) -> Result<F> {
        unsafe {
            let name = CString::new(name)?;
            let addr = LLVMGetFunctionAddress(self.handle, name.as_ptr().cast());

            let mut err = mem::zeroed();

            if LLVMExecutionEngineGetErrMsg(self.handle, &mut err) != 0 {
                let err = CStr::from_ptr(err).to_str()?;
                bail!("Failed to get function address: {err}");
            }

            let res = mem::transmute_copy::<u64, F>(&addr);

            Ok(res)
        }
    }
}

impl<'a> Drop for ExecutionEngine<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeExecutionEngine(self.handle);
        }
    }
}
