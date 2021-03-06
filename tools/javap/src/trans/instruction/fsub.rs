use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Fsub;

impl Instruction for Fsub {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::fsub,
            icp: 0,
            wide: false,
        };

        (info, pc + 1)
    }
}
