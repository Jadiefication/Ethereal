#[derive(Debug, PartialEq, Clone, Copy)]
pub enum IntSuffix {
    U8, U16, U32, U64,
    I8, I16, I32, I64,
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct IntLiteral {
    pub value: i64,
    pub suffix: IntSuffix,
}