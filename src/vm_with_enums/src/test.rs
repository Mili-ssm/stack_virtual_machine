use log::info;

use vm_lib::StackMachine;

use crate::{
    data_types::Data,
    instructions::{BinaryOp, Instruction},
};

#[test_log::test]
fn test() {
    let result = 0_usize;
    let code = vec![
        // result = 4 + 24
        Instruction::Load(Data::Int(24)),
        Instruction::Load(Data::Int(4)),
        Instruction::BinaryOps(BinaryOp::Add),
        Instruction::Store(Data::Pointer(result)),
        //print(result)
        Instruction::Print(Data::Pointer(result)),
        // result = result/3
        Instruction::Load(Data::Pointer(result)),
        Instruction::Load(Data::Int(3)),
        Instruction::BinaryOps(BinaryOp::Divide),
        Instruction::Store(Data::Pointer(result)),
        //print(result)
        Instruction::Print(Data::Pointer(result)),
        //exit()
        Instruction::HALT,
    ];

    info!("CODE: {:?}", &code);
    let mut vm = StackMachine::new(code, 8);
    vm.run();
}
