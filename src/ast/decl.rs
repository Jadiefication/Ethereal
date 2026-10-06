use crate::ast::stmt::Block;
use crate::ast::types::{Attribute, Type};
use lasso::Spur;
use logos::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct ImportDecl {
    pub path: Spur,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldDecl {
    pub name: Spur,
    pub ty: Type,
    pub attributes: Vec<Attribute>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub has_hash: bool,
    pub name: Spur,
    pub fields: Vec<FieldDecl>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Param {
    pub is_val: bool,
    pub has_hash: bool,
    pub name: Spur,
    pub ty: Option<Type>,
    pub attributes: Vec<Attribute>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub is_pub: bool,
    pub is_static: bool,
    pub attributes: Vec<Attribute>,
    pub name: Spur,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KernelDecl {
    pub attributes: Vec<Attribute>,
    pub grid_id_name: Spur,
    pub local_id_name: Spur,
    pub workgroup_id_name: Spur,
    pub name: Spur,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImplBlock {
    pub has_hash: bool,
    pub target: Spur,
    pub methods: Vec<FunctionDecl>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Import(ImportDecl),
    Struct(StructDecl),
    Impl(ImplBlock),
    Function(FunctionDecl),
    Kernel(KernelDecl),
}
