#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Void,
    Boolean(bool),
    Number(f64),
}
