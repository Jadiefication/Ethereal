use lasso::Spur;
use logos::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: Spur,
    pub sub_path: Option<Spur>,
    pub args: Vec<Spur>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub kind: TypeKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    /// Primitive types: `u32`, `i32`, `f64`, `bool`, etc.
    Primitive(Spur),
    /// User defined named type: `VectorPair`, `#VectorPair`
    Named {
        name: Spur,
        has_hash: bool,
    },
    /// Dynamic slice: `[]#VectorPair`, `[]f64`
    Slice(Box<Type>),
    /// Fixed-size array: `[256]f64`
    Array {
        element_ty: Box<Type>,
        size: usize,
    },
    /// Attribute-annotated type: `@gpu::f64`, `@storage []#VectorPair`, `@workgroup [256]f64`
    Annotated {
        attributes: Vec<Attribute>,
        inner: Box<Type>,
    },
}
