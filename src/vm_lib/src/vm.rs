use std::{process::exit, vec};

use log::{info, warn};

use crate::{BasicOp, FunctionOps, NativeType, Stack};
pub struct StackMachine<D: NativeType, Op: BasicOp<D>> {
    const_table: Vec<D>,
    variable_table: Vec<D>,
    //
    pub stack: Stack<D>,
    code: FunctionOps<Op>,
    //
    ipointer: usize,
    pub run_timer: std::time::Instant,
}

impl<D: NativeType, Op: BasicOp<D>> StackMachine<D, Op> {
    pub fn new(code: FunctionOps<Op>, stack_size: usize) -> Self {
        StackMachine {
            const_table: Vec::new(),
            variable_table: vec![D::default(); stack_size],
            //
            stack: Stack::<D>::new(stack_size),
            code,
            //
            ipointer: 0,
            run_timer: std::time::Instant::now(),
        }
    }

    pub fn run(&mut self) {
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

    pub fn halt(&mut self) {
        let elapsed = self.run_timer.elapsed();
        println!("Execution time: {:?}", elapsed);

        self.ipointer = usize::MAX;

        warn!("EXITING VM");
        exit(0);
    }

    pub fn store(&mut self, pointer: usize, arg: D) {
        self.variable_table[pointer] = arg;
        info!("\t HEAP: {:?}", self.variable_table);
    }

    pub fn load(&self, pointer: usize) -> &D {
        info!("\t HEAP: {:?}", self.variable_table);
        &self.variable_table[pointer]
    }

    pub fn free(&mut self, pointer: usize) -> D {
        let result = self.variable_table[pointer].clone();
        self.variable_table[pointer] = D::default();

        result
    }

    pub fn malloc(&mut self, size: usize) -> usize {
        let pointer = self.variable_table.len();

        self.variable_table.extend(vec![D::default(); size]);

        pointer
    }

    pub fn load_const(&self, pointer: usize) -> &D {
        &self.const_table[pointer]
    }

    pub fn print(&self, arg: &D) {
        info!("\t PRINTING: {:?}", arg);
        println!("{:?}", arg);
    }

    pub fn jump(&mut self, ipointer: usize) {
        self.ipointer = ipointer;
    }
}
