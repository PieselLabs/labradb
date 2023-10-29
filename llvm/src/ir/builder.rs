use std::ffi::CString;

#[allow(clippy::wildcard_imports)]
use llvm_sys::{core::*, prelude::LLVMBuilderRef};

use crate::{BasicBlock, Context, Value};

#[allow(clippy::module_name_repetitions)]
pub struct IRBuilder<'a> {
    pub(crate) handle: LLVMBuilderRef,
    ctx: &'a Context,
}

macro_rules! binary_op {
    ($name:tt, $func:tt) => {
        #[must_use]
        pub fn $name(&mut self, lhs: Value<'a>, rhs: Value<'a>, twine: &str) -> Value<'a> {
            unsafe {
                let twine = CString::new(twine).unwrap();
                let handle = $func(self.handle, lhs.handle, rhs.handle, twine.as_ptr());
                Value::from(handle)
            }
        }
    };
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

    binary_op!(create_add, LLVMBuildAdd);
    binary_op!(create_fadd, LLVMBuildFAdd);

    binary_op!(create_sub, LLVMBuildSub);
    binary_op!(create_fsub, LLVMBuildFSub);

    binary_op!(create_mul, LLVMBuildMul);
    binary_op!(create_fmul, LLVMBuildFMul);

    binary_op!(create_udiv, LLVMBuildUDiv);
    binary_op!(create_sdiv, LLVMBuildSDiv);
    binary_op!(create_fdiv, LLVMBuildFDiv);

    binary_op!(create_urem, LLVMBuildURem);
    binary_op!(create_srem, LLVMBuildSRem);
    binary_op!(create_frem, LLVMBuildFRem);

    binary_op!(create_shl, LLVMBuildShl);
    binary_op!(create_lshr, LLVMBuildLShr);
    binary_op!(create_ashr, LLVMBuildAShr);
    binary_op!(create_and, LLVMBuildAnd);
    binary_op!(create_or, LLVMBuildOr);
    binary_op!(create_xor, LLVMBuildXor);

    pub fn create_ret(&mut self, res: Value<'a>) -> Value<'a> {
        unsafe {
            let handle = LLVMBuildRet(self.handle, res.handle);
            Value::from(handle)
        }
    }

    pub fn create_br(&mut self, bb: BasicBlock<'a>) -> Value<'a> {
        unsafe {
            let handle = LLVMBuildBr(self.handle, bb.handle);
            Value::from(handle)
        }
    }

    pub fn create_cond_br(
        &mut self,
        cond: Value<'a>,
        then_bb: BasicBlock<'a>,
        else_bb: BasicBlock<'a>,
    ) -> Value<'a> {
        unsafe {
            let handle = LLVMBuildCondBr(self.handle, cond.handle, then_bb.handle, else_bb.handle);
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
