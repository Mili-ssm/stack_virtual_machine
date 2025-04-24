use std::fmt::Debug;

use crate::ProcessContext;

// ------------------------
// MARK: TYPES
//------------------------

pub type FunctionOps<Op> = Vec<Op>;

// ------------------------
// MARK: TRAITS
//------------------------

pub trait NativeType
where
    Self: Debug + Clone + Default + PartialEq,
{
}

pub trait Executable<D: NativeType>
where
    Self: Debug + Clone + Sized + PartialEq + 'static,
{
    fn execute(&self, proc: &mut ProcessContext<D>) -> ();
}

pub trait Compilable<D: NativeType>
where
    Self: Debug + Clone + PartialEq,
{
    type Instruction: Executable<D>;
    fn compile(&self) -> Box<[Self::Instruction]>;
}

pub trait OpManager<D: NativeType> {
    fn goto(&mut self, ipntr: usize) -> ();

    fn get_ipntr(&self) -> usize;
}

pub trait Runnable<D: NativeType> {
    fn run(&mut self) -> ();

    fn is_finished(&self) -> bool;
}
