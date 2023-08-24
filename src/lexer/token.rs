#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Tok {
    Int64(i64),
    Add,
    Sub,
    Mul,
    Div
}