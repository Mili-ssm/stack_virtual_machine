use core::panic;

use vm_lib::{Compilable, Executable, ProcessContext, Stack};

use crate::data_types::{Arg, Data};

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    GT,
    GET,
    LT,
    LET,
    EQ,
    NEQ,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    //Binary operations
    BinaryOp(BinaryOp, Arg, Arg),
    //Store a value in the stack
    Store(Arg),
    //Load a value to the Accumulator
    Load(Arg),
    //Copy a value into the Stack/Pointer direction
    Copy(Arg, Arg),
    //Free a number of values from the
    Free(u8),
    //Jump to a specific instruction
    Jump(Arg),
    //Jump to a specific instruction if the value is not 0
    JumpIf(Arg, Arg),
    //Print a value
    Print(Arg),
    //Finish the program
    HALT,
}

type OpProc = ProcessContext<Data>;

#[derive(Debug, Clone, PartialEq)]
pub enum SuperInstruction {
    IF(Arg, usize),
    Swap(Arg, Arg),
    Simple(Instruction),
}

impl Compilable<Data> for SuperInstruction {
    type Instruction = Instruction;

    fn compile(&self) -> Box<[Self::Instruction]> {
        let expanded_op = match self {
            SuperInstruction::IF(arg, lines) => vec![
                Self::Simple(Instruction::Jump(arg.clone())),
                Self::Simple(Instruction::Jump(Arg::Const(Data::Pointer(1)))),
                Self::Simple(Instruction::Jump(Arg::Const(Data::Pointer(*lines)))),
            ],

            SuperInstruction::Swap(arg1, arg2) => vec![
                Self::Simple(Instruction::Copy(arg1.clone(), Arg::Acc)),
                Self::Simple(Instruction::Copy(arg2.clone(), arg1.clone())),
                Self::Simple(Instruction::Copy(Arg::Acc, arg2.clone())),
            ],
            SuperInstruction::Simple(_) => vec![self.clone()],
        };

        let mut compiled = Vec::new();
        for op in expanded_op {
            if let Self::Simple(instruction) = op {
                compiled.push(instruction);
                continue;
            }
            compiled.extend_from_slice(&op.compile());
        }

        compiled.into_boxed_slice()
    }
}

impl Executable<Data> for Instruction {
    fn execute(&self, proc: &mut OpProc) -> () {
        match self {
            Instruction::BinaryOp(op, a, b) => op.execute(&mut proc.stack, a, b),
            Instruction::Store(arg) => Self::store(proc, arg),
            Instruction::Load(arg) => Self::load(proc, arg),
            Instruction::Copy(src, tgt) => Self::copy(proc, src, tgt),
            Instruction::Free(n) => Self::clean_stack(&mut proc.stack, *n as u64),
            Instruction::Jump(arg) => Self::jump(proc, arg),
            Instruction::JumpIf(cond, arg) => Self::jump_if(proc, cond, arg),
            Instruction::Print(arg) => Self::print(proc, arg),
            Instruction::HALT => proc.halt(),
            //_ => unimplemented!(),
        }
    }
}

impl Instruction {
    fn jump(proc: &mut OpProc, arg: &Arg) -> () {
        let arg = arg.deref(&proc.stack);

        match arg {
            Data::Byte(ipointer) => proc.goto(*ipointer as usize),
            Data::Pointer(ipointer) => proc.goto(*ipointer),

            Data::Int(ipointer) => proc.goto_rel(*ipointer as isize),
            Data::Bool(ipointer) => proc.goto_rel(*ipointer as isize),
            Data::None => {}
            _ => panic!("Jump offset must be an integer"),
        }
    }

    fn jump_if(proc: &mut OpProc, cond: &Arg, arg: &Arg) -> () {
        let cond = cond.deref(&proc.stack);

        match cond {
            Data::Bool(true) | Data::Int(1..) | Data::Float(1.0..) => Self::jump(proc, arg),
            Data::Bool(false) | Data::Int(0) | Data::Float(0.0) | Data::None => {}
            _ => panic!("Jump condition must be a boolean"),
        }
    }

    fn print(proc: &OpProc, arg: &Arg) -> () {
        let value = arg.deref(&proc.stack);

        proc.print(value);
    }

    fn copy(proc: &mut OpProc, src: &Arg, tgt: &Arg) -> () {
        let value = src.deref(&proc.stack);

        match tgt {
            Arg::Ref(name) => proc.stack.store_at(*name as usize, value.clone()),
            Arg::Acc => proc.stack.to_register(value.clone()),
            Arg::Const(_) => panic!("Cannot copy to a constant"),
        }
    }

    fn store(proc: &mut OpProc, arg: &Arg) -> () {
        let pointer = if let Arg::Ref(rel_pntr) = arg {
            proc.get_rel_ipntr(-(*rel_pntr as isize))
        } else {
            if let Arg::Const(data) = arg {
                proc.stack.to_register(data.clone());
            }
            proc.stack.store_register()
        };

        proc.stack.to_register(Data::Pointer(pointer));
    }

    fn load(proc: &mut OpProc, arg: &Arg) -> () {
        let value = match arg {
            Arg::Const(data) => data,
            Arg::Ref(name) => proc.stack.peek_register(*name as usize),
            Arg::Acc => {
                if let Data::Pointer(pntr) = proc.stack.peek_register(0) {
                    proc.stack.peek_at(*pntr)
                } else {
                    panic!("Cannot \"Load\" from Accumulator")
                }
            }
        };
        proc.stack.to_register(value.clone());
    }

    fn clean_stack(stack: &mut Stack<Data>, n: u64) -> () {
        for _ in 0..n {
            stack.pop::<1>();
        }
    }
}

impl BinaryOp {
    fn execute(&self, stack: &mut Stack<Data>, a: &Arg, b: &Arg) -> () {
        let value_a = a.deref(stack);
        let value_b = b.deref(stack);

        let result = match self {
            BinaryOp::Add => Self::add(value_a, value_b),
            BinaryOp::Subtract => Self::substract(value_a, value_b),
            BinaryOp::Multiply => Self::multiply(value_a, value_b),
            BinaryOp::Divide => Self::divide(value_a, value_b),
            BinaryOp::GT => Self::gt(value_a, value_b),
            BinaryOp::GET => Self::gte(value_a, value_b),
            BinaryOp::LT => Self::gte(value_b, value_a),
            BinaryOp::LET => Self::gt(value_b, value_a),
            BinaryOp::EQ => Self::eq(value_a, value_b),
            BinaryOp::NEQ => Self::neq(value_a, value_b),
        };

        stack.to_register(result);
    }

    fn add(a: &Data, b: &Data) -> Data {
        let result = match (a, b) {
            (Data::Int(a), Data::Int(b)) => Data::Int(a + b),
            (Data::Float(a), Data::Float(b)) => Data::Float(a + b),
            (Data::Byte(a), Data::Byte(b)) => Data::Byte(a + b),
            (Data::ByteArray(a), Data::ByteArray(b)) => {
                let value = Box::new([*a.clone(), *b.clone()].concat().into_boxed_slice());
                Data::ByteArray(value)
            }
            (Data::String(a), Data::String(b)) => {
                let mut value = a.to_string();
                value.push_str(b);
                Data::String(Box::new(value))
            }
            _ => panic!("Type mismatch"),
        };

        result
    }

    fn substract(a: &Data, b: &Data) -> Data {
        let result = match (a, b) {
            (Data::Int(a), Data::Int(b)) => Data::Int(a - b),
            (Data::Float(a), Data::Float(b)) => Data::Float(a - b),
            (Data::Byte(a), Data::Byte(b)) => Data::Byte(a - b),
            _ => panic!("Type mismatch"),
        };

        result
    }

    fn multiply(a: &Data, b: &Data) -> Data {
        let result = match (a, b) {
            (Data::Int(a), Data::Int(b)) => Data::Int(a * b),
            (Data::Float(a), Data::Float(b)) => Data::Float(a * b),
            (Data::Byte(a), Data::Byte(b)) => Data::Byte(a * b),
            _ => panic!("Type mismatch"),
        };

        result
    }

    fn divide(a: &Data, b: &Data) -> Data {
        let result = match (a, b) {
            (Data::Int(a), Data::Int(b)) => Data::Int(a / b),
            (Data::Float(a), Data::Float(b)) => Data::Float(a / b),
            (Data::Byte(a), Data::Byte(b)) => Data::Byte(a / b),
            _ => panic!("Type mismatch"),
        };

        result
    }

    fn gt(a: &Data, b: &Data) -> Data {
        let result = match (a, b) {
            (Data::Int(a), Data::Int(b)) => Data::Bool(a > b),
            (Data::Float(a), Data::Float(b)) => Data::Bool(a > b),
            (Data::Byte(a), Data::Byte(b)) => Data::Bool(a > b),
            _ => panic!("Type mismatch"),
        };

        result
    }

    fn gte(a: &Data, b: &Data) -> Data {
        let result = match (a, b) {
            (Data::Int(a), Data::Int(b)) => Data::Bool(a >= b),
            (Data::Float(a), Data::Float(b)) => Data::Bool(a >= b),
            (Data::Byte(a), Data::Byte(b)) => Data::Bool(a >= b),
            _ => panic!("Type mismatch"),
        };

        result
    }

    fn eq(a: &Data, b: &Data) -> Data {
        let result = match (a, b) {
            (Data::Int(a), Data::Int(b)) => Data::Bool(a == b),
            (Data::Float(a), Data::Float(b)) => Data::Bool(a == b),
            (Data::Byte(a), Data::Byte(b)) => Data::Bool(a == b),
            _ => panic!("Type mismatch"),
        };

        result
    }

    fn neq(a: &Data, b: &Data) -> Data {
        let result = match (a, b) {
            (Data::Int(a), Data::Int(b)) => Data::Bool(a != b),
            (Data::Float(a), Data::Float(b)) => Data::Bool(a != b),
            (Data::Byte(a), Data::Byte(b)) => Data::Bool(a != b),
            _ => panic!("Type mismatch"),
        };

        result
    }
}
