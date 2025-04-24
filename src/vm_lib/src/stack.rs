use std::fmt::Debug;

use log::debug;

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

    pub fn to_register(&mut self, value: T) {
        self.store_at(self.pointer, value);
    }

    pub fn store_at(&mut self, pointer: usize, value: T) {
        self.data[pointer] = value;

        debug!("\t STACK: {:?}", self);
    }

    pub fn store_register(&mut self) -> usize {
        let pointer = self.pointer;
        self.pointer += 1;

        debug!("\t STACK: {:?}", self);
        pointer
    }

    pub fn pop<const N: usize>(&mut self) -> [T; N] {
        let len = self.pointer;
        self.pointer -= N;

        let mut result = std::array::repeat(T::default());
        result.swap_with_slice(&mut self.data[self.pointer..len]);

        //info!("\t STACK: {:?}", self);

        result
    }

    pub fn swap(&mut self) {
        self.data.swap(self.pointer - 1, self.pointer);

        debug!("\t STACK: {:?}", self);
    }

    pub fn peek<const N: usize>(&self) -> [T; N] {
        let len = self.pointer;
        let pointer = self.pointer - N;

        let mut result = std::array::repeat(T::default());
        result.clone_from_slice(&self.data[pointer..len]);

        //info!("\t STACK: {:?}", self);

        result
    }
    pub fn peek_at(&self, pointer: usize) -> &T {
        &self.data[pointer]
    }

    pub fn peek_register(&self, pointer: usize) -> &T {
        let pointer = self.pointer - pointer;
        &self.data[pointer]
    }
    pub fn len(&self) -> usize {
        self.pointer
    }

    pub fn is_empty(&self) -> bool {
        self.pointer == 0
    }
}
