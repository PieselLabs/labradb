use std::{ffi::CString, ops::Deref};

use llvm_sys::core::{LLVMAddFunction, LLVMGetParam};

use crate::{FunctionType, Module, Value};

#[derive(Copy, Clone)]
pub struct Function<'ctx> {
    value: Value<'ctx>,
}

impl<'ctx> Function<'ctx> {
    #[must_use]
    pub fn create(module: &'ctx Module<'ctx>, name: &str, signature: FunctionType<'ctx>) -> Self {
        unsafe {
            let name = CString::new(name).unwrap();

            let handle = LLVMAddFunction(module.handle, name.as_ptr(), signature.handle);

            Function {
                value: Value::from(handle),
            }
        }
    }

    #[must_use]
    pub fn get_arg(&self, idx: u32) -> Value<'ctx> {
        unsafe {
            let handle = LLVMGetParam(self.handle, idx);
            Value::from(handle)
        }
    }
}

impl<'ctx> Deref for Function<'ctx> {
    type Target = Value<'ctx>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
