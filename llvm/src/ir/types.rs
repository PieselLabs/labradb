use std::marker::PhantomData;

use llvm_sys::{
    core::{
        LLVMDoubleTypeInContext, LLVMFloatTypeInContext, LLVMFunctionType, LLVMInt16TypeInContext,
        LLVMInt1TypeInContext, LLVMInt32TypeInContext, LLVMInt64TypeInContext,
        LLVMInt8TypeInContext, LLVMVoidTypeInContext,
    },
    prelude::*,
};

use crate::Context;

#[derive(Copy, Clone)]
pub struct Type<'ctx> {
    pub(crate) handle: LLVMTypeRef,
    _marker: PhantomData<&'ctx ()>,
}

macro_rules! type_constructor {
    ($name:tt, $func:tt) => {
        #[must_use]
        pub fn $name(ctx: &'ctx Context) -> Self {
            unsafe {
                let handle = $func(ctx.handle);
                Type {
                    handle,
                    _marker: PhantomData,
                }
            }
        }
    };
}

impl<'ctx> Type<'ctx> {
    type_constructor!(get_void, LLVMVoidTypeInContext);
    type_constructor!(get_i1, LLVMInt1TypeInContext);
    type_constructor!(get_i8, LLVMInt8TypeInContext);
    type_constructor!(get_i16, LLVMInt16TypeInContext);
    type_constructor!(get_i32, LLVMInt32TypeInContext);
    type_constructor!(get_i64, LLVMInt64TypeInContext);
    type_constructor!(get_f32, LLVMFloatTypeInContext);
    type_constructor!(get_f64, LLVMDoubleTypeInContext);
}

#[derive(Copy, Clone)]
pub struct FunctionType<'a> {
    pub(crate) handle: LLVMTypeRef,
    ctx: PhantomData<&'a Context>,
}

impl<'a> FunctionType<'a> {
    #[must_use]
    pub fn get(res: Type<'a>, args: &[Type<'a>]) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        unsafe {
            let mut args_handles: Vec<LLVMTypeRef> = args.iter().map(|a| a.handle).collect();
            let handle = LLVMFunctionType(
                res.handle,
                args_handles.as_mut_ptr(),
                args_handles.len() as u32,
                0,
            );
            FunctionType {
                handle,
                ctx: PhantomData,
            }
        }
    }
}
