#![feature(inherent_associated_types)]
#![feature(generic_const_exprs)]
#![allow(dead_code)]

use std::{fmt::Debug, process::exit};

use log::{info, warn};

#[derive(Debug, Clone)]
pub struct Stack<T: Debug>(Vec<T>);
impl<T: Debug> Stack<T> {
    pub fn new() -> Self {
        Stack(Vec::new())
    }
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    pub fn pop(&mut self) -> T {
        if let Some(item) = self.0.pop() {
            item
        } else {
            panic!("Stack underflow");
        }
    }

    pub fn drop<const N: usize>(&mut self) -> [T; N] {
        let mut result = Vec::new();
        for _ in 0..N {
            if let Some(item) = self.0.pop() {
                result.push(item);
            } else {
                panic!("Stack underflow");
            }
        }
        result.reverse();
        result.try_into().unwrap()
    }
    pub fn peek(&self) -> Option<&T> {
        self.0.last()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub trait BasicOp<D>: Sized + Clone + Debug
where
    D: Debug,
{
    fn execute(&self, vm: &mut StackMachine<D, Self>) -> ();
}

pub type FunctionOps<Op> = Vec<Op>;
pub struct StackMachine<D: Debug, Op: BasicOp<D>> {
    pub const_table: Vec<D>,
    pub variable_table: Vec<D>,
    //
    pub stack: Stack<D>,
    code: FunctionOps<Op>,
    //
    pub ipointer: usize,
    pub run_timer: std::time::Instant,
}

impl<D: Debug, Op: BasicOp<D>> StackMachine<D, Op> {
    pub fn new(code: FunctionOps<Op>) -> Self {
        StackMachine {
            const_table: Vec::new(),
            variable_table: Vec::new(),
            //
            stack: Stack::new(),
            code,
            //
            ipointer: 0,
            run_timer: std::time::Instant::now(),
        }
    }
    pub fn run(&mut self) -> () {
        let code = self.code.clone();
        self.run_timer = std::time::Instant::now();

        loop {
            let op = unsafe { code.get_unchecked(self.ipointer) };
            //println!("");
            info!("Executing IP: {}  OP {:?}", self.ipointer, op);

            self.ipointer += 1;
            op.execute(self);
        }
    }

    pub fn halt(&mut self) -> () {
        let elapsed = self.run_timer.elapsed();
        println!("Execution time: {:?}", elapsed);

        self.ipointer = usize::MAX;

        warn!("EXITING VM");
        exit(0);
    }
}
