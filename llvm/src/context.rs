use llvm_sys::{
    core::{LLVMContextCreate, LLVMContextDispose},
    prelude::*,
};

pub struct Context {
    pub(crate) handle: LLVMContextRef,
}

impl Default for Context {
    fn default() -> Self {
        unsafe {
            let handle = LLVMContextCreate();
            Context { handle }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            LLVMContextDispose(self.handle);
        }
    }
}
