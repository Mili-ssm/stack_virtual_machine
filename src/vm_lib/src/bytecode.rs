use log::debug;

use crate::{Executable, NativeType};

pub struct ProgramCode<Op: Executable<D>, D: NativeType> {
    instructions: Vec<Op>,
    constants: Vec<D>,
}

pub struct ByteCode<Op: Executable<D>, D: NativeType> {
    instructions: Box<[Op]>,
    constants: Box<[D]>,
}

/// ------------------------
/// MARK: IMPLEMENTS
/// ------------------------

impl<Op, D> ProgramCode<Op, D>
where
    Op: Executable<D>,
    D: NativeType,
{
    pub const fn new(instructions: Vec<Op>, constants: Vec<D>) -> Self {
        ProgramCode {
            instructions,
            constants,
        }
    }

    pub fn compile(&self) -> ByteCode<Op, D> {
        ByteCode {
            instructions: self.instructions.clone().into_boxed_slice(),
            constants: self.constants.clone().into_boxed_slice(),
        }
    }
}

impl<Op, D> ByteCode<Op, D>
where
    Op: Executable<D>,
    D: NativeType,
{
    pub fn new(instructions: Box<[Op]>, constants: Box<[D]>) -> Self {
        ByteCode {
            instructions,
            constants,
        }
    }

    pub const fn get(&self) -> &[Op] {
        &self.instructions
    }
    pub const fn get_at(&self, ipointer: usize) -> &Op {
        //debug!("IP {:?}  ", ipointer);
        &self.instructions[ipointer]
    }
    pub const fn get_constants(&self) -> &[D] {
        &self.constants
    }
}
