use std::ffi::CString;

#[allow(clippy::wildcard_imports)]
use llvm_sys::{core::*, prelude::LLVMBuilderRef};

use crate::{BasicBlock, Context, Value};

#[allow(clippy::module_name_repetitions)]
pub struct IRBuilder<'a> {
    pub(crate) handle: LLVMBuilderRef,
    ctx: &'a Context,
}

impl<'a> IRBuilder<'a> {
    #[must_use]
    pub fn new(ctx: &'a Context) -> Self {
        unsafe {
            let handle = LLVMCreateBuilderInContext(ctx.handle);
            IRBuilder { handle, ctx }
        }
    }

    #[must_use]
    pub fn get_context(&self) -> &Context {
        self.ctx
    }

    pub fn set_position_to_end<'b>(&mut self, bb: &'b BasicBlock<'a>) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.handle, bb.handle);
        }
    }

    pub fn create_add(&mut self, lhs: Value<'a>, rhs: Value<'a>, twine: &str) -> Value<'a> {
        unsafe {
            let twine = CString::new(twine).unwrap();
            let handle = LLVMBuildAdd(self.handle, lhs.handle, rhs.handle, twine.as_ptr());
            Value::from(handle)
        }
    }

    pub fn create_ret(&mut self, res: Value<'a>) -> Value<'a> {
        unsafe {
            let handle = LLVMBuildRet(self.handle, res.handle);
            Value::from(handle)
        }
    }
}

impl<'a> Drop for IRBuilder<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.handle);
        }
    }
}
