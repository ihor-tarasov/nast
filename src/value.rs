#[derive(Debug, Clone, Copy)]
pub enum Value {
    Void,
    Boolean(bool),
    Number(f64),
}
