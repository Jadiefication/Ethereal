use crate::ast::stmt::Block;
use crate::ast::types::Type;
use crate::num::float::FloatLiteral;
use crate::num::int::IntLiteral;
use lasso::Spur;
use logos::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    // --- Literals ---
    Int(IntLiteral),
    Float(FloatLiteral),
    String(Spur),
    Bool(bool),
    EmptySlice(Type),
    ArrayLit(Vec<Expr>),

    // --- Identifiers ---
    Variable {
        name: Spur,
        has_hash: bool,
    },
    This,

    // --- Unary & Binary ---
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    // --- Access Operators ---
    FieldAccess {
        target: Box<Expr>,
        field: Spur,
    },
    IndexAccess {
        target: Box<Expr>,
        index: Box<Expr>,
    },

    // --- Control Flow ---
    If {
        condition: Box<Expr>,
        then_branch: Block,
        else_branch: Option<Block>,
    },
    Block(Block),

    // --- Struct Instantiation ---
    StructInit {
        name: Spur,
        has_hash: bool,
        fields: Vec<StructFieldInit>,
    },

    // --- Invocations & Piping ---
    Call {
        callee: Box<Expr>,
        generic_args: Vec<Type>,
        args: Vec<Expr>,
    },
    MethodCall {
        receiver: Box<Expr>,
        method: Spur,
        args: Vec<Expr>,
    },
    Pipe {
        value: Box<Expr>,
        target: Box<Expr>,
    },
    Dispatch {
        kernel_name: Spur,
        has_hash: bool,
        grid: Vec<Expr>,
        args: Vec<Expr>,
        is_sync_fence: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructFieldInit {
    pub name: Spur,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
    Range,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}
