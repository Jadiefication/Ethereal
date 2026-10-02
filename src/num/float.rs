#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FloatSuffix {
    F32,
    F64,
    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FloatLiteral {
    pub value: f64,
    pub suffix: FloatSuffix,
}