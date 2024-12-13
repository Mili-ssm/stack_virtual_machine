use std::{default, fmt::Debug};

use log::info;

use crate::NativeType;

pub(crate) const STACK_SIZE: usize = 1024;

#[derive(Debug, Clone)]
pub struct Stack<T: NativeType> {
    pointer: usize,
    data: Vec<T>,
}

impl<T: NativeType> Stack<T> {
    pub fn new(stack_size: usize) -> Self {
        Stack {
            data: vec![T::default(); stack_size],
            pointer: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.pointer < STACK_SIZE {
            self.data[self.pointer] = item;
            self.pointer += 1;
        } else {
            panic!("Stack overflow");
        }

        info!("\t STACK: {:?}", self);
    }

    pub fn pop<const N: usize>(&mut self) -> [T; N] {
        let len = self.pointer;
        self.pointer -= N;

        let mut result = std::array::repeat(T::default());
        result.swap_with_slice(&mut self.data[self.pointer..len]);

        //info!("\t STACK: {:?}", self);

        result
    }
    pub fn peek(&self) -> Option<&T> {
        self.data.get(self.pointer - 1)
    }

    pub fn len(&self) -> usize {
        self.pointer
    }

    pub fn is_empty(&self) -> bool {
        self.pointer == 0
    }
}
