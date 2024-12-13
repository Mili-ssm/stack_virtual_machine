use crate::stack_vm::{BasicOp, Stack, StackMachine};
use log::info;
use std::{borrow::Borrow, collections::HashMap, io::Write};

#[derive(Debug, Clone)]
enum Data {
    Int(i64),
    Float(f64),
    Bool(bool),
    Byte(u8),
    ByteArray(Box<Vec<u8>>),
    String(Box<String>),
    Function(Box<String>),
    Tuple(Box<Vec<Data>>),
    Class(Box<HashMap<String, Data>>),
    Puntero(usize),
    None,
}

//const type_size: usize = std::mem::size_of::<Data>();

#[derive(Debug, Clone)]
enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
enum Instruction {
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
        Data::Puntero(name) => &vm.variable_table[*name],
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
        Data::Puntero(name) => &vm.variable_table[*name],
        arg => arg,
    };

    info!("\t PRINTING: {:?}", value);
    println!("{:?}", value);
}

fn load(vm: &mut StackMachine<Data, Instruction>, arg: &Data) -> () {
    let value = match arg {
        Data::Puntero(name) => &vm.variable_table[*name],
        arg => arg,
    };

    vm.stack.push(value.clone());
    info!("\t STACK: {:?}", vm.stack);
}

fn store(vm: &mut StackMachine<Data, Instruction>, arg: &Data) -> () {
    let name = match arg {
        Data::Puntero(name) => *name as usize,
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

#[test_log::test]
fn test() {
    let result = 0_usize;
    let code = vec![
        // result = 4 + 24
        Instruction::Load(Data::Int(24)),
        Instruction::Load(Data::Int(4)),
        Instruction::BinaryOps(BinaryOp::Add),
        Instruction::Store(Data::Puntero(result)),
        //print(result)
        Instruction::Print(Data::Puntero(result)),
        // result = result/3
        Instruction::Load(Data::Puntero(result)),
        Instruction::Load(Data::Int(3)),
        Instruction::BinaryOps(BinaryOp::Divide),
        Instruction::Store(Data::Puntero(result)),
        //print(result)
        Instruction::Print(Data::Puntero(result)),
        //exit()
        Instruction::HALT,
    ];

    info!("CODE: {:?}", &code);
    let mut vm = StackMachine::new(code);
    vm.variable_table.resize(8, Data::None);
    vm.const_table.resize(8, Data::None);
    vm.run();
}
