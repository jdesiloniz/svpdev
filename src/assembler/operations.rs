use super::instructions;
use crate::asm::macros;
use crate::asm::mnemonics;

#[derive(Debug)]
pub enum Operation<'a> {
    Macro(macros::SspMacro),
    Instruction(instructions::Instruction<'a>),
}

impl<'a> Operation<'a> {
    pub fn new_macro(m: &macros::SspMacro) -> Operation<'a> {
        Operation::Macro(*m)
    }

    pub fn new_instruction(m: &mnemonics::SspMnemonic) -> Operation<'a> {
        Operation::Instruction(instructions::Instruction::new(*m))
    }
}
