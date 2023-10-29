use std::marker::PhantomData;

use llvm_sys::prelude::LLVMValueRef;

#[derive(Copy, Clone)]
pub struct Value<'m> {
    pub(crate) handle: LLVMValueRef,
    _marker: PhantomData<&'m ()>,
}

impl<'ctx> Value<'ctx> {
    pub(crate) fn from(handle: LLVMValueRef) -> Self {
        Value {
            handle,
            _marker: PhantomData,
        }
    }
}
