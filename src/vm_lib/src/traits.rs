use std::fmt::Debug;

use crate::StackMachine;

pub trait NativeType
where
    Self: Debug + Clone + Default,
{
}

pub trait BasicOp<D: NativeType>
where
    Self: Debug + Clone + Sized,
{
    fn execute(&self, vm: &mut StackMachine<D, Self>) -> ();
}

pub type FunctionOps<Op> = Vec<Op>;
