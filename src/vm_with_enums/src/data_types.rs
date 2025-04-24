use std::collections::BTreeMap;

use vm_lib::{NativeType, Stack};

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Data {
    Int(i64),
    Float(f64),
    Bool(bool),
    Byte(u8),
    ByteArray(Box<Box<[u8]>>),
    String(Box<String>),
    Tuple(Box<Box<[Data]>>),
    List(Box<Vec<Data>>),
    Dict(Box<BTreeMap<Data, Data>>),
    Pointer(usize),
    Function(Box<String>),

    #[default]
    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    Const(Data),
    Ref(usize),
    Acc,
}

impl NativeType for Data {}

impl Default for Arg {
    fn default() -> Self {
        Arg::Acc
    }
}
impl Arg {
    #[inline]
    pub fn deref<'a>(&'a self, stack: &'a Stack<Data>) -> &'a Data {
        match self {
            Arg::Const(data) => data,
            Arg::Ref(name) => stack.peek_register((*name + 1) as usize),
            Arg::Acc => stack.peek_register(0),
        }
    }
}
