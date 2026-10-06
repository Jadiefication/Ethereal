use crate::ast::expr::Expr;
use crate::ast::types::Type;
use lasso::Spur;
use logos::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
    /// `val x = expr` or `val #x: u32 = expr`
    ValDecl {
        has_hash: bool,
        name: Spur,
        ty: Option<Type>,
        init: Expr,
    },
    /// `var #stride: u32 = 128u32` or `var #shared_cache: @workgroup [256]f64`
    VarDecl {
        has_hash: bool,
        name: Spur,
        ty: Option<Type>,
        init: Option<Expr>,
    },
    /// `while (cond) { ... }`
    While {
        condition: Expr,
        body: Block,
    },
    /// `for (i in 0..N)` or `for (sum in reduced_partials)`
    For {
        iter_var: Spur,
        iterable: Expr,
        body: Block,
    },
    /// `#barrier();`
    Barrier,
    /// An expression executed as a statement (e.g. `pair |> host_pairs.push()`)
    Expr(Expr),
}
