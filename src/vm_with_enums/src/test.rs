use std::time::Instant;

use log::info;

use vm_lib::{ProgramCode, StackMachine};

use crate::{
    data_types::{Arg, Data},
    instructions::{BinaryOp, Instruction},
};

#[test_log::test]
fn test() {
    {
        let timer = Instant::now();
        let mut result = 24;
        result += 4;
        println!("Result: {}", result);
        result /= 3;
        println!("Result: {}", result);
        let elapsed = timer.elapsed();
        println!("Execution time: {:?}", elapsed);
    }
    println!("");
    println!("");
    println!("-------------------------------------------");
    println!("");
    println!("");
    {
        let code = vec![
            // result = 4 + 24
            Instruction::BinaryOp(
                BinaryOp::Add,
                Arg::Const(Data::Int(24)),
                Arg::Const(Data::Int(4)),
            ),
            //print(result)
            Instruction::Print(Arg::Acc),
            // result = result/3
            Instruction::BinaryOp(BinaryOp::Divide, Arg::Acc, Arg::Const(Data::Int(3))),
            //print(result)
            Instruction::Print(Arg::Acc),
            //exit()
            Instruction::HALT,
        ];

        let program = ProgramCode::new(code.clone(), vec![]);

        info!("CODE: {:?}", &code);
        let mut vm = StackMachine::new();

        vm.add_process(program);
        vm.run();
    }
}

#[test_log::test]
fn test2() {
    const INCREMENT: f64 = 1.000001;
    const MAX: f64 = 1_000_000_000_000.0;

    {
        let timer = Instant::now();
        let mut counter = 0;
        let mut i = 1.0f64;
        while i < MAX {
            i *= INCREMENT;
            counter += 1;
        }
        println!("Result: {} | Iteraciones: {}", i, counter);
        println!("Terminado jejeje");
        let elapsed = timer.elapsed();
        println!("Execution time: {:?}", elapsed);
    }
    println!("");
    println!("");
    println!("-------------------------------------------");
    println!("");
    println!("");
    {
        let code = vec![
            // i = 1.0
            Instruction::Store(Arg::Const(Data::Float(1.0))),
            //while i < MAX || (Condition is inverted to don`t jump)
            Instruction::BinaryOp(BinaryOp::LT, Arg::Const(Data::Float(MAX)), Arg::Ref(0)),
            Instruction::JumpIf(Arg::Acc, Arg::Const(Data::Int(3))),
            //i *= INCREMENT
            Instruction::BinaryOp(
                BinaryOp::Multiply,
                Arg::Ref(0),
                Arg::Const(Data::Float(INCREMENT)),
            ),
            Instruction::Copy(Arg::Acc, Arg::Ref(0)),
            // End of while
            Instruction::Jump(Arg::Const(Data::Int(-5))),
            //print(i)
            Instruction::Print(Arg::Ref(0)),
            //Finishing
            Instruction::Print(Arg::Const(Data::String(Box::new(
                "TERMINADO JEJEJE".to_string(),
            )))),
            //exit()
            Instruction::HALT,
        ];

        let program = ProgramCode::new(code.clone(), vec![]);
        info!("CODE: {:?}", &code);
        let mut vm = StackMachine::new();
        vm.add_process(program);
        vm.run();
    }
}
