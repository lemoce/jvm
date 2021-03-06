use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Goto;

impl Instruction for Goto {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::goto,
            icp: 0,
            wide: false,
        };

        (info, pc + 3)
    }
}
