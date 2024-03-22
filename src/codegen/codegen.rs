//! Handles the main code-generation logic.

use crate::ast::*;
use anyhow::{Result, Error, anyhow};

use cranelift::{codegen::{
    ir::{types::I64, AbiParam, Function, Signature, UserFuncName, self},
    isa::CallConv,
}, prelude::{InstBuilder, Block, types::{I8, self}, MemFlags, Variable, EntityRef, Value, IntCC}};
use cranelift::frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::Module;
use cranelift_object::{ObjectBuilder, ObjectModule};

use crate::ast;

fn to_primitive_kind(ty: &ast::Type) -> Result<ast::PrimitiveKind> {
    match ty {
        ast::Type::Primitive(p) => Ok(p.kind.clone()),
        _ => Err(anyhow!("Expected a primitive type")),
    }
}

/// Only supports primitive types for now
fn to_cranelift_type(ty: &ast::Type) -> Result<types::Type> {
    match to_primitive_kind(ty)? {
        ast::PrimitiveKind::Int => Ok(I64),
        ast::PrimitiveKind::Bool => Ok(I8),
        ast::PrimitiveKind::Float => unimplemented!(),
    }
}

/// Compiles a function declaration into a Cranelift IR function
pub fn compile_function(funcDecl: &ast::FunctionDecl, builder: &mut FunctionBuilder) -> Result<Signature> {
    let mut sig = Signature::new(CallConv::SystemV);

    // Parameters
    for param in funcDecl.parameters.iter() {
        let param_ty = to_cranelift_type(&param.ty)?;
        sig.params.push(AbiParam::new(param_ty));
    }

    // Return type
    let return_ty = to_cranelift_type(&funcDecl.ty)?;
    sig.returns.push(AbiParam::new(return_ty));

    // Build function
    let entry_block = builder.create_block();
    builder.switch_to_block(entry_block);
    builder.seal_block(entry_block);
    builder.append_block_params_for_function_params(entry_block);

    // Variables
    let mut variables = Vec::new();
    for (param) in funcDecl.parameters.iter() {
        let ty = to_cranelift_type(&param.ty)?;
        let var = Variable::new(variables.len());
        builder.declare_var(var, ty);
    }

    Ok(sig)
}