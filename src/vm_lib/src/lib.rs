#![feature(inherent_associated_types)]
#![feature(generic_const_exprs)]
#![feature(array_repeat)]
#![allow(dead_code)]
#![feature(associated_type_defaults)]
#![feature(random)]

mod bytecode;
mod stack;
mod traits;
mod vm;

pub use bytecode::*;
pub use stack::*;
pub use traits::*;
pub use vm::*;
