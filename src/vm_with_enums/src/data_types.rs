use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Data {
    Int(i64),
    Float(f64),
    Bool(bool),
    Byte(u8),
    ByteArray(Box<Vec<u8>>),
    String(Box<String>),
    Function(Box<String>),
    Tuple(Box<Vec<Data>>),
    Class(Box<HashMap<String, Data>>),
    Pointer(usize),
    None,
}
