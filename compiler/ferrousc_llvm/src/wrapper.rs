use std::collections::HashMap;
use std::ffi::CString;

use llvm_sys::core::*;
use llvm_sys::prelude::*;

const LLVM_FALSE: LLVMBool = 0;
const LLVM_TRUE: LLVMBool = 1;

pub(crate) struct Functions {
    pub(crate) functions: HashMap<String, LLVMValueRef>
}

impl Functions{
    pub(crate) fn new() -> Self {
        Functions { functions: HashMap::new() }
    }
}

pub(crate) trait Droppable {
    fn drop(&mut self);
}

pub(crate) struct BuildInTypes {
    pub(crate) void_type: LLVMTypeRef,
    pub(crate) i8_type: LLVMTypeRef,
    pub(crate) i8_ptr_type: LLVMTypeRef,
    pub(crate) i16_type: LLVMTypeRef,
    pub(crate) i32_type: LLVMTypeRef,
    pub(crate) i64_type: LLVMTypeRef,
}

pub(crate) struct Context {
    pub(crate) context: *mut llvm_sys::LLVMContext,
    pub(crate) types: BuildInTypes,
}

impl Context {
    pub(crate) fn new() -> Self {
        unsafe {
            let ctx = LLVMContextCreate();
            let i8_type = LLVMInt8TypeInContext(ctx);

            let types = BuildInTypes {
                void_type: LLVMVoidTypeInContext(ctx),
                i8_type: i8_type,
                i8_ptr_type: LLVMPointerType(i8_type, 0),
                i16_type: LLVMInt16TypeInContext(ctx),
                i32_type: LLVMInt32TypeInContext(ctx),
                i64_type: LLVMInt64TypeInContext(ctx),
            };

            Context {
                context: ctx,
                types,
            }
        }
    }
}

impl Droppable for Context {
    fn drop(&mut self) {
        unsafe { LLVMContextDispose(self.context) }
    }
}

pub(crate) struct Builder {
    pub(crate) builder: *mut llvm_sys::LLVMBuilder,
}

impl Builder {
    pub(crate) fn new(context: &Context) -> Self {
        unsafe {
            Builder {
                builder: LLVMCreateBuilderInContext(context.context)
            }
        }
    }    
    
    pub(crate) fn position_at_end(&self, basic_block: LLVMBasicBlockRef) {
        unsafe { LLVMPositionBuilderAtEnd(self.builder, basic_block) }
    }
}

impl Droppable for Builder {
    fn drop(&mut self) {
        unsafe { LLVMDisposeBuilder(self.builder) }
    }
}

pub(crate) struct Module {
    pub(crate) module: *mut llvm_sys::LLVMModule,
}

impl Module {
    pub(crate) fn new(name: &str, context: &Context) -> Self {
        unsafe {
            let c_str_name = CString::new(name).unwrap();
            Module {
                module: LLVMModuleCreateWithNameInContext(c_str_name.as_ptr(), context.context)
            }
        }
    }
}

impl Droppable for Module {
    fn drop(&mut self) {
        unsafe { LLVMDisposeModule(self.module); }
    }
}