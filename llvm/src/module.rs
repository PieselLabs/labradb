use crate::context::Context;
use anyhow::{bail, Result};
use llvm_sys::{
    core::{LLVMCreateMemoryBufferWithMemoryRange, LLVMDisposeModule, LLVMDumpModule},
    ir_reader::LLVMParseIRInContext,
    prelude::*,
};
use std::ffi::CStr;

pub struct Module<'ctx> {
    pub(crate) handle: LLVMModuleRef,
    _ctx: &'ctx Context,
}

impl<'ctx> Module<'ctx> {
    pub fn from_bytes(ctx: &'ctx Context, bytes: &[u8]) -> Result<Self> {
        unsafe {
            let buf = LLVMCreateMemoryBufferWithMemoryRange(
                bytes.as_ptr().cast(),
                bytes.len(),
                b"bitcode_buf\0".as_ptr().cast(),
                0,
            );

            let mut handle = std::ptr::null_mut();
            let mut err_string = std::mem::zeroed();

            if LLVMParseIRInContext(ctx.handle, buf, &mut handle, &mut err_string) != 0 {
                let err = CStr::from_ptr(err_string).to_str()?;
                bail!("Failed to parse IR: {err}");
            }

            Ok(Module { handle, _ctx: ctx })
        }
    }

    pub fn dump(&self) {
        unsafe {
            LLVMDumpModule(self.handle);
        }
    }
}

impl<'ctx> Drop for Module<'ctx> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.handle);
        }
    }
}
