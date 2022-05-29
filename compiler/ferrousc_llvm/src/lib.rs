extern crate llvm_sys;

mod util;
mod wrapper;

use crate::util::*;
use crate::wrapper::*;

use llvm_sys::core::*;

use std::convert::TryInto;
use std::ffi::CString;
use std::fs;
use std::ptr;

use ferrousc_ast::nodes::*;
use ferrousc_lexer::tokenize;
use ferrousc_parser::generate_ast;

static TEST_CODE: &str = "./llvm_test.fe";

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).expect(&format!("could not read file with name: {}", filename))
}

pub fn run() {
    let test_code = read_file(TEST_CODE);

    let ast = generate_ast(tokenize(&test_code));

    unsafe {
        // setup            
        let mut context = Context::new();
        let mut module = Module::new("main", &context);
        let mut builder = Builder::new(&context);
        let functions = Functions::new();

        // declare an function without body to tell llvm to import this
        let puts_func_type = LLVMFunctionType(context.types.i32_type, [context.types.i8_ptr_type].as_ptr() as *mut _, 1, 0);
        let puts_func = LLVMAddFunction(module.module, c_str!("puts"), puts_func_type);        

        // the main function
        let main_func_type = LLVMFunctionType(context.types.i32_type, ptr::null_mut(), 0, 0);
        let main_func = LLVMAddFunction(module.module, c_str!("main"), main_func_type);
        let main_block = LLVMAppendBasicBlockInContext(context.context, main_func, c_str!("main"));
        builder.position_at_end(main_block);

        // main's function body
        let hello_world_str = LLVMBuildGlobalStringPtr(builder.builder, c_str!("hello, world."), c_str!(""));
        let puts_args = [hello_world_str].as_ptr() as *mut _;
        LLVMBuildCall(builder.builder, puts_func, puts_args, 1, c_str!(""));
        LLVMBuildRet(builder.builder, LLVMConstInt(context.types.i32_type, 0, 0));

        ast.walk(|st|{
            walk(st, &functions, &builder, &module, &context);
        });

        // export
        LLVMPrintModuleToFile(module.module, c_str!("main.ll"), ptr::null_mut());

        // cleanup
        builder.drop();
        module.drop();
        context.drop();
    }
}

unsafe fn walk(st: &Stat, functions: &mut Functions, builder: &Builder, module: &Module, context: &Context) {
    match &*st {
        Stat::FunctionDefinition {
            fn_token, 
            identifier, 
            parameter_list,
            return_type,
            body,
        } => {
            let func_ret_type = if return_type.is_some() { context.types.i32_type } else { context.types.void_type };
            let func_type = LLVMFunctionType(func_ret_type, ptr::null_mut(), parameter_list.parameters.len().try_into().unwrap(), 0);
            let func_name = CString::new(identifier.identifier.token.value.clone()).unwrap().into_raw();

            let func = LLVMAddFunction(module.module, func_name, func_type);

            functions.functions.insert(identifier.identifier.token.value, func);

            let func_block = LLVMAppendBasicBlockInContext(context.context, func, func_name);

            builder.position_at_end(func_block);
            LLVMBuildRetVoid(builder.builder);


            match body.as_ref() {
                FunctionBody::BlockStatement{ block } => {
                    walk(block, &functions, &builder, &module, &context);
                },
                FunctionBody::ExpressionBody{ fat_arrow_token, statement } => {
                    walk(statement, &functions, &builder, &module, &context);
                },
            }
        },
        Stat::Expr { expr, semicolon_token } => {
            match expr {
                Expr::Call { identifier, argument_list } => {
                    let func_name = CString::new(identifier.identifier.token.value.clone()).unwrap().into_raw();
                    
                    LLVMBuildCall(builder.builder, functions.functions.get(&identifier.identifier.token.value).unwrap().to_owned(), ptr::null_mut(), 3, func_name);
                }
            }
        }
        #[allow(unreachable_patterns)]
        stat => println!("unknown statement! {:?}", stat),
    }
}