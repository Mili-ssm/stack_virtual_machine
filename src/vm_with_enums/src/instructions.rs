use log::info;
use vm_lib::{BasicOp, Stack, StackMachine};

use crate::data_types::Data;

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Load(Data),
    Store(Data),
    Jump(Data),
    Print(Data),
    BinaryOps(BinaryOp),
    HALT,
}

impl BasicOp<Data> for Instruction {
    #[inline]
    fn execute(&self, vm: &mut StackMachine<Data, Self>) -> () {
        match self {
            Instruction::BinaryOps(op) => binary_op(vm, op),
            Instruction::Jump(arg) => jump(vm, arg),
            Instruction::Print(arg) => print(vm, arg),
            Instruction::Load(arg) => load(vm, arg),
            Instruction::Store(arg) => store(vm, arg),
            Instruction::HALT => vm.halt(),
            //_ => unimplemented!(),
        }
    }
}

#[inline]
fn binary_op(vm: &mut StackMachine<Data, Instruction>, op: &BinaryOp) -> () {
    let stack = &mut vm.stack;
    match op {
        BinaryOp::Add => add(stack),
        BinaryOp::Subtract => substract(stack),
        BinaryOp::Multiply => multiply(stack),
        BinaryOp::Divide => divide(stack),
    }
}

fn jump(vm: &mut StackMachine<Data, Instruction>, arg: &Data) -> () {
    let offset = match arg {
        Data::Pointer(name) => &vm.variable_table[*name],
        arg => arg,
    };
    if let &Data::Int(offset) = offset {
        vm.ipointer = offset as usize;
    } else {
        panic!("Jump offset must be an integer");
    }
}

fn print(vm: &mut StackMachine<Data, Instruction>, arg: &Data) -> () {
    let value = match arg {
        Data::Pointer(name) => &vm.variable_table[*name],
        arg => arg,
    };

    info!("\t PRINTING: {:?}", value);
    println!("{:?}", value);
}

fn load(vm: &mut StackMachine<Data, Instruction>, arg: &Data) -> () {
    let value = match arg {
        Data::Pointer(name) => &vm.variable_table[*name],
        arg => arg,
    };

    vm.stack.push(value.clone());
    info!("\t STACK: {:?}", vm.stack);
}

fn store(vm: &mut StackMachine<Data, Instruction>, arg: &Data) -> () {
    let name = match arg {
        Data::Pointer(name) => *name as usize,
        _ => panic!("Store argument must be a variable name"),
    };

    vm.variable_table[name] = vm.stack.pop();
    info!("\t HEAP: {:?}", vm.variable_table);
}

fn add(stack: &mut Stack<Data>) -> () {
    let [a, b] = stack.drop::<2>();

    let result = match (a, b) {
        (Data::Int(a), Data::Int(b)) => Data::Int(a + b),
        (Data::Float(a), Data::Float(b)) => Data::Float(a + b),
        (Data::Byte(a), Data::Byte(b)) => Data::Byte(a + b),
        (Data::ByteArray(mut a), Data::ByteArray(b)) => {
            a.extend(b.as_ref());
            Data::ByteArray(a)
        }
        (Data::String(mut a), Data::String(b)) => {
            a.push_str(&b);
            Data::String(a)
        }
        _ => panic!("Type mismatch"),
    };

    stack.push(result);
}

fn substract(stack: &mut Stack<Data>) -> () {
    let [a, b] = stack.drop::<2>();

    let result = match (a, b) {
        (Data::Int(a), Data::Int(b)) => Data::Int(a - b),
        (Data::Float(a), Data::Float(b)) => Data::Float(a - b),
        (Data::Byte(a), Data::Byte(b)) => Data::Byte(a - b),
        _ => panic!("Type mismatch"),
    };

    stack.push(result);
}

fn multiply(stack: &mut Stack<Data>) -> () {
    let [a, b] = stack.drop::<2>();

    let result = match (a, b) {
        (Data::Int(a), Data::Int(b)) => Data::Int(a * b),
        (Data::Float(a), Data::Float(b)) => Data::Float(a * b),
        (Data::Byte(a), Data::Byte(b)) => Data::Byte(a * b),
        _ => panic!("Type mismatch"),
    };

    stack.push(result);
}

fn divide(stack: &mut Stack<Data>) -> () {
    let [a, b] = stack.drop::<2>();

    let result = match (a, b) {
        (Data::Int(a), Data::Int(b)) => Data::Int(a / b),
        (Data::Float(a), Data::Float(b)) => Data::Float(a / b),
        (Data::Byte(a), Data::Byte(b)) => Data::Byte(a / b),
        _ => panic!("Type mismatch"),
    };

    stack.push(result);
}