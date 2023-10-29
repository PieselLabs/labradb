use std::{ffi::CString, marker::PhantomData};

use llvm_sys::{core::LLVMAppendBasicBlockInContext, prelude::LLVMBasicBlockRef};

use crate::{Context, Function};

#[derive(Copy, Clone)]
pub struct BasicBlock<'ctx> {
    pub(crate) handle: LLVMBasicBlockRef,
    _marker: PhantomData<&'ctx ()>,
}

impl<'ctx> BasicBlock<'ctx> {
    #[must_use]
    pub fn create(ctx: &'ctx Context, func: Function<'ctx>, name: &str) -> Self {
        unsafe {
            let name = CString::new(name).unwrap();

            let handle = LLVMAppendBasicBlockInContext(ctx.handle, func.handle, name.as_ptr());

            BasicBlock {
                handle,
                _marker: PhantomData,
            }
        }
    }
}
