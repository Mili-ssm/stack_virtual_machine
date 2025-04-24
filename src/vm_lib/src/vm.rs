use std::random::random;

use log::{debug, trace, warn};

use crate::{Executable, NativeType, ProgramCode, Runnable, Stack, bytecode::ByteCode};

// ------------------------
// MARK: TYPES
//------------------------

pub struct ProcessContext<D: NativeType> {
    pub stack: Stack<D>,
    ipointer: usize,
    calls_history: Vec<usize>,
    is_finished: bool,
    run_timer: std::time::Instant,
}

pub struct Process<Op, D>
where
    Op: Executable<D>,
    D: NativeType,
{
    pid: usize,
    //
    code: ByteCode<Op, D>,
    context: ProcessContext<D>,
}

pub struct StackMachine<D: NativeType> {
    //
    pub heap: Stack<D>,
    pub proceses: Vec<Box<dyn Runnable<D>>>,
}

// ------------------------
// MARK: IMPLEMENTS
//------------------------

impl<D: NativeType + 'static> StackMachine<D> {
    pub fn new() -> Self {
        StackMachine {
            heap: Stack::<D>::new(1024),
            proceses: vec![],
        }
    }

    pub fn run(&mut self) {
        let mut running_process = 0;
        while self.proceses.len() > 0 {
            if running_process >= self.proceses.len() {
                running_process = 0;
            }

            let process = self.proceses[running_process].as_mut();
            process.run();

            if process.is_finished() {
                self.proceses.remove(running_process);
                continue;
            }

            running_process += 1;
        }
    }

    pub fn add_process<Op: Executable<D>>(&mut self, program_code: ProgramCode<Op, D>) {
        let bytecode = program_code.compile();
        let process = Box::new(Process::new(64, bytecode));
        self.proceses.push(process);
    }
}

impl<Op: Executable<D>, D: NativeType> Process<Op, D> {
    pub fn new(stack_size: usize, code: ByteCode<Op, D>) -> Self {
        Process {
            pid: random(),
            code,
            //
            context: ProcessContext {
                stack: Stack::<D>::new(stack_size),
                run_timer: std::time::Instant::now(),
                ipointer: 0,
                calls_history: vec![],
                is_finished: false,
            },
        }
    }
}

impl<D: NativeType> ProcessContext<D> {
    pub fn goto(&mut self, ipntr: usize) {
        self.ipointer = ipntr;
    }

    pub fn get_ipntr(&self) -> usize {
        self.ipointer
    }

    pub fn goto_rel(&mut self, offset: isize) {
        self.goto(self.get_rel_ipntr(offset));
    }

    pub fn get_rel_ipntr(&self, offset: isize) -> usize {
        self.get_ipntr().overflowing_add_signed(offset).0
    }

    pub fn print(&self, arg: &D) {
        trace!("\t PRINTING: {:?}", arg);
        println!("{:?}", arg);
    }

    pub fn halt(&mut self) {
        println!("Execution time: {:?}", self.run_timer.elapsed());

        warn!("EXITING VM");

        debug!("STACK: {:?}", self.stack);
        debug!("IP: {:?}", self.ipointer);

        self.is_finished = true;

        //self.context.ipointer = usize::MAX;
        //exit(0)
    }
}

impl<Op: Executable<D>, D: NativeType> Runnable<D> for Process<Op, D> {
    #[inline]
    fn run(&mut self) {
        loop {
            self.code
                .get_at(self.context.ipointer)
                .execute(&mut self.context);

            self.context.ipointer += 1;
            if self.context.is_finished {
                break;
            }
        }
    }

    #[inline]
    fn is_finished(&self) -> bool {
        self.context.is_finished
    }
}
