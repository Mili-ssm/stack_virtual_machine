use std::collections::HashMap;

use vm_lib::NativeType;

#[derive(Debug, Clone, Default)]
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

    #[default]
    None,
}

impl NativeType for Data {}
