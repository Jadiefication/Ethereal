#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: u32, // Byte index where token begins
    pub end: u32,   // Byte index where token ends
}
