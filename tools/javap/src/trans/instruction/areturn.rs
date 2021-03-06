use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Areturn;

impl Instruction for Areturn {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::areturn,
            icp: 0,
            wide: false,
        };

        (info, pc + 1)
    }
}
