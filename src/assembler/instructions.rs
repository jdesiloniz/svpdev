use super::errors;
use crate::asm::mnemonics;
use crate::asm::operators;
use crate::asm::registers;
use crate::tokenization::tokens;
use std::error::Error;

#[derive(Debug, Copy, Clone)]
pub struct Instruction<'a> {
    mnemonic: Option<mnemonics::SspMnemonic>,
    op1: Option<tokens::Token<'a>>,
    op2: Option<tokens::Token<'a>>,
}

impl<'a> Instruction<'a> {
    pub fn new(mnemonic: mnemonics::SspMnemonic) -> Instruction<'a> {
        Instruction {
            mnemonic: Some(mnemonic),
            op1: None,
            op2: None,
        }
    }

    // Validate if an op fits an existing but not complete instruction
    pub fn validate_op(&self, op: &tokens::Token) -> bool {
        match (self.mnemonic, self.op1, self.op2, op) {
            // An already full instruction shouldn't accept more ops:
            (Some(_), Some(_), Some(_), _) => false,
            (Some(mnemonics::SspMnemonic::Ret), _, _, _) => false,
            (
                Some(mnemonics::SspMnemonic::Sub(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Word(_))),
                _,
                _,
            ) => false,
            (
                Some(mnemonics::SspMnemonic::Cmp(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Word(_))),
                _,
                _,
            ) => false,
            (
                Some(mnemonics::SspMnemonic::Add(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Word(_))),
                _,
                _,
            ) => false,
            (
                Some(mnemonics::SspMnemonic::And(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Word(_))),
                _,
                _,
            ) => false,
            (
                Some(mnemonics::SspMnemonic::Or(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Word(_))),
                _,
                _,
            ) => false,
            (
                Some(mnemonics::SspMnemonic::Eor(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Word(_))),
                _,
                _,
            ) => false,
            (
                Some(mnemonics::SspMnemonic::Sub(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(_))),
                _,
                _,
            ) => false,
            (
                Some(mnemonics::SspMnemonic::Cmp(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(_))),
                _,
                _,
            ) => false,
            (
                Some(mnemonics::SspMnemonic::Add(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(_))),
                _,
                _,
            ) => false,
            (
                Some(mnemonics::SspMnemonic::And(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(_))),
                _,
                _,
            ) => false,
            (
                Some(mnemonics::SspMnemonic::Or(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(_))),
                _,
                _,
            ) => false,
            (
                Some(mnemonics::SspMnemonic::Eor(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(_))),
                _,
                _,
            ) => false,

            // Let's reject invalid values for operators:
            (Some(_), None, None, tokens::Token::Macro(_)) => false,
            (Some(_), None, None, tokens::Token::Mnemonic(_)) => false,
            (Some(_), None, None, tokens::Token::Invalid(_)) => false,
            (Some(_), Some(_), None, tokens::Token::Macro(_)) => false,
            (Some(_), Some(_), None, tokens::Token::Mnemonic(_)) => false,
            (Some(_), Some(_), None, tokens::Token::Invalid(_)) => false,

            _ => true,
        }
    }

    // This function performs a shallow validation on if an instruction should be considered complete,
    // i.e.: having all parameters in place. It won't check if the instruction is actually valid.
    pub fn is_complete(&self) -> bool {
        match (self.mnemonic, self.op1, self.op2) {
            // Ops with no operators
            (Some(mnemonics::SspMnemonic::Ret), None, None) => true,

            // Ops with immediate operators
            (
                Some(mnemonics::SspMnemonic::Sub(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Word(_))),
                None,
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Cmp(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Word(_))),
                None,
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Add(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Word(_))),
                None,
            ) => true,
            (
                Some(mnemonics::SspMnemonic::And(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Word(_))),
                None,
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Or(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Word(_))),
                None,
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Eor(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Word(_))),
                None,
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Sub(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(_))),
                None,
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Cmp(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(_))),
                None,
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Add(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(_))),
                None,
            ) => true,
            (
                Some(mnemonics::SspMnemonic::And(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(_))),
                None,
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Or(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(_))),
                None,
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Eor(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(_))),
                None,
            ) => true,

            // Ops that support two operators:
            (
                Some(mnemonics::SspMnemonic::Sub(_)),
                Some(tokens::Token::Operator(_)),
                Some(tokens::Token::Operator(_)),
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Cmp(_)),
                Some(tokens::Token::Operator(_)),
                Some(tokens::Token::Operator(_)),
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Add(_)),
                Some(tokens::Token::Operator(_)),
                Some(tokens::Token::Operator(_)),
            ) => true,
            (
                Some(mnemonics::SspMnemonic::And(_)),
                Some(tokens::Token::Operator(_)),
                Some(tokens::Token::Operator(_)),
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Or(_)),
                Some(tokens::Token::Operator(_)),
                Some(tokens::Token::Operator(_)),
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Eor(_)),
                Some(tokens::Token::Operator(_)),
                Some(tokens::Token::Operator(_)),
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Ld(_)),
                Some(tokens::Token::Operator(_)),
                Some(tokens::Token::Operator(_)),
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Mld),
                Some(tokens::Token::Operator(_)),
                Some(tokens::Token::Operator(_)),
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Mpya),
                Some(tokens::Token::Operator(_)),
                Some(tokens::Token::Operator(_)),
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Mpys),
                Some(tokens::Token::Operator(_)),
                Some(tokens::Token::Operator(_)),
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Mod),
                Some(tokens::Token::Operator(_)),
                Some(tokens::Token::Operator(_)),
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Bra),
                Some(tokens::Token::Operator(_)),
                Some(tokens::Token::Operator(_)),
            ) => true,
            (
                Some(mnemonics::SspMnemonic::Call),
                Some(tokens::Token::Operator(_)),
                Some(tokens::Token::Operator(_)),
            ) => true,

            _ => false,
        }
    }

    pub fn new_with_op(&self, op: &tokens::Token<'a>) -> Instruction<'a> {
        match (self.mnemonic, self.op1, self.op2) {
            (Some(_), Some(_), None) => Instruction {
                mnemonic: self.mnemonic,
                op1: self.op1,
                op2: Some(*op),
            },

            (Some(_), None, None) => Instruction {
                mnemonic: self.mnemonic,
                op1: Some(*op),
                op2: None,
            },

            _ => *self,
        }
    }

    // This function validates that the function we received is a valid SSP16xx one,
    // and if that's the case it'll build the appropiate opcodes:
    pub fn build(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        match (self.mnemonic, self.op1, self.op2) {
            // RET:
            (Some(mnemonics::SspMnemonic::Ret), None, None) => Ok(vec![0, 0x65]),

            // **** Arithmetic ops with one operator ****

            // OPi simm
            (
                Some(mnemonics::SspMnemonic::Sub(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(value))),
                None,
            ) => Ok(vec![0x38, value]),

            (
                Some(mnemonics::SspMnemonic::Cmp(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(value))),
                None,
            ) => Ok(vec![0x78, value]),

            (
                Some(mnemonics::SspMnemonic::Add(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(value))),
                None,
            ) => Ok(vec![0x98, value]),

            (
                Some(mnemonics::SspMnemonic::And(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(value))),
                None,
            ) => Ok(vec![0xB8, value]),

            (
                Some(mnemonics::SspMnemonic::Or(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(value))),
                None,
            ) => Ok(vec![0xD8, value]),

            (
                Some(mnemonics::SspMnemonic::Eor(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(value))),
                None,
            ) => Ok(vec![0xF8, value]),

            // **** Arithmetic ops with two operators ****

            // OPi A, imm:
            (
                Some(mnemonics::SspMnemonic::Sub(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(value))),
            ) => Ok(Instruction::opcodes_with_base_and_imm_value(
                vec![0x28, 0x0],
                value,
            )),

            (
                Some(mnemonics::SspMnemonic::Cmp(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(value))),
            ) => Ok(Instruction::opcodes_with_base_and_imm_value(
                vec![0x68, 0x0],
                value,
            )),

            (
                Some(mnemonics::SspMnemonic::Add(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(value))),
            ) => Ok(Instruction::opcodes_with_base_and_imm_value(
                vec![0x88, 0x0],
                value,
            )),

            (
                Some(mnemonics::SspMnemonic::And(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(value))),
            ) => Ok(Instruction::opcodes_with_base_and_imm_value(
                vec![0xA8, 0x0],
                value,
            )),

            (
                Some(mnemonics::SspMnemonic::Or(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(value))),
            ) => Ok(Instruction::opcodes_with_base_and_imm_value(
                vec![0xC8, 0x0],
                value,
            )),

            (
                Some(mnemonics::SspMnemonic::Eor(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(value))),
            ) => Ok(Instruction::opcodes_with_base_and_imm_value(
                vec![0xE8, 0x0],
                value,
            )),

            // OP A, s
            (
                Some(mnemonics::SspMnemonic::Sub(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(register))),
            ) => Ok(vec![0x20, register.value()]),

            (
                Some(mnemonics::SspMnemonic::Cmp(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(register))),
            ) => Ok(vec![0x60, register.value()]),

            (
                Some(mnemonics::SspMnemonic::Add(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(register))),
            ) => Ok(vec![0x80, register.value()]),

            (
                Some(mnemonics::SspMnemonic::And(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(register))),
            ) => Ok(vec![0xA0, register.value()]),

            (
                Some(mnemonics::SspMnemonic::Or(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(register))),
            ) => Ok(vec![0xC0, register.value()]),

            (
                Some(mnemonics::SspMnemonic::Eor(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(register))),
            ) => Ok(vec![0xE0, register.value()]),

            // OP A, ri
            (
                Some(mnemonics::SspMnemonic::Sub(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Ptr(register))),
            ) => Ok(vec![0x32 + register.ram_bank(), register.value()]),

            (
                Some(mnemonics::SspMnemonic::Cmp(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Ptr(register))),
            ) => Ok(vec![0x72 + register.ram_bank(), register.value()]),

            (
                Some(mnemonics::SspMnemonic::Add(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Ptr(register))),
            ) => Ok(vec![0x92 + register.ram_bank(), register.value()]),

            (
                Some(mnemonics::SspMnemonic::And(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Ptr(register))),
            ) => Ok(vec![0xB2 + register.ram_bank(), register.value()]),

            (
                Some(mnemonics::SspMnemonic::Or(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Ptr(register))),
            ) => Ok(vec![0xD2 + register.ram_bank(), register.value()]),

            (
                Some(mnemonics::SspMnemonic::Eor(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Ptr(register))),
            ) => Ok(vec![0xF2 + register.ram_bank(), register.value()]),

            // OP A, (ri)
            (
                Some(mnemonics::SspMnemonic::Sub(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(register))),
            ) => Ok(vec![
                0x22 + register.ram_bank(),
                (register.modifier_value() << 2) + register.value(),
            ]),

            (
                Some(mnemonics::SspMnemonic::Cmp(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(register))),
            ) => Ok(vec![
                0x62 + register.ram_bank(),
                (register.modifier_value() << 2) + register.value(),
            ]),

            (
                Some(mnemonics::SspMnemonic::Add(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(register))),
            ) => Ok(vec![
                0x82 + register.ram_bank(),
                (register.modifier_value() << 2) + register.value(),
            ]),

            (
                Some(mnemonics::SspMnemonic::And(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(register))),
            ) => Ok(vec![
                0xA2 + register.ram_bank(),
                (register.modifier_value() << 2) + register.value(),
            ]),

            (
                Some(mnemonics::SspMnemonic::Or(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(register))),
            ) => Ok(vec![
                0xC2 + register.ram_bank(),
                (register.modifier_value() << 2) + register.value(),
            ]),

            (
                Some(mnemonics::SspMnemonic::Eor(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(register))),
            ) => Ok(vec![
                0xE2 + register.ram_bank(),
                (register.modifier_value() << 2) + register.value(),
            ]),

            // OP A, ((ri))
            (
                Some(mnemonics::SspMnemonic::Sub(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrDoubleRef(register))),
            ) => Ok(vec![
                0x2A + register.ram_bank(),
                (register.modifier_value() << 2) + register.value(),
            ]),

            (
                Some(mnemonics::SspMnemonic::Cmp(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrDoubleRef(register))),
            ) => Ok(vec![
                0x6A + register.ram_bank(),
                (register.modifier_value() << 2) + register.value(),
            ]),

            (
                Some(mnemonics::SspMnemonic::Add(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrDoubleRef(register))),
            ) => Ok(vec![
                0x8A + register.ram_bank(),
                (register.modifier_value() << 2) + register.value(),
            ]),

            (
                Some(mnemonics::SspMnemonic::And(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrDoubleRef(register))),
            ) => Ok(vec![
                0xAA + register.ram_bank(),
                (register.modifier_value() << 2) + register.value(),
            ]),

            (
                Some(mnemonics::SspMnemonic::Or(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrDoubleRef(register))),
            ) => Ok(vec![
                0xCA + register.ram_bank(),
                (register.modifier_value() << 2) + register.value(),
            ]),

            (
                Some(mnemonics::SspMnemonic::Eor(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrDoubleRef(register))),
            ) => Ok(vec![
                0xEA + register.ram_bank(),
                (register.modifier_value() << 2) + register.value(),
            ]),

            // OP A, addr
            (
                Some(mnemonics::SspMnemonic::Sub(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(addr))),
            ) => Ok(vec![0x26, addr]),

            (
                Some(mnemonics::SspMnemonic::Cmp(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(addr))),
            ) => Ok(vec![0x66, addr]),

            (
                Some(mnemonics::SspMnemonic::Add(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(addr))),
            ) => Ok(vec![0x86, addr]),

            (
                Some(mnemonics::SspMnemonic::And(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(addr))),
            ) => Ok(vec![0xA6, addr]),

            (
                Some(mnemonics::SspMnemonic::Or(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(addr))),
            ) => Ok(vec![0xC6, addr]),

            (
                Some(mnemonics::SspMnemonic::Eor(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(addr))),
            ) => Ok(vec![0xE6, addr]),

            (
                Some(mnemonics::SspMnemonic::Sub(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(addr))),
            ) => Ok(vec![
                0x26 + (((addr & 0x100) >> 8) as u8),
                (addr & 0xFF) as u8,
            ]),

            (
                Some(mnemonics::SspMnemonic::Cmp(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(addr))),
            ) => Ok(vec![
                0x66 + (((addr & 0x100) >> 8) as u8),
                (addr & 0xFF) as u8,
            ]),

            (
                Some(mnemonics::SspMnemonic::Add(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(addr))),
            ) => Ok(vec![
                0x86 + (((addr & 0x100) >> 8) as u8),
                (addr & 0xFF) as u8,
            ]),

            (
                Some(mnemonics::SspMnemonic::And(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(addr))),
            ) => Ok(vec![
                0xA6 + (((addr & 0x100) >> 8) as u8),
                (addr & 0xFF) as u8,
            ]),

            (
                Some(mnemonics::SspMnemonic::Or(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(addr))),
            ) => Ok(vec![
                0xC6 + (((addr & 0x100) >> 8) as u8),
                (addr & 0xFF) as u8,
            ]),

            (
                Some(mnemonics::SspMnemonic::Eor(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(addr))),
            ) => Ok(vec![
                0xE6 + (((addr & 0x100) >> 8) as u8),
                (addr & 0xFF) as u8,
            ]),

            // MOD f, op
            (
                Some(mnemonics::SspMnemonic::Mod),
                Some(tokens::Token::Operator(operators::SspOperator::ConditionFlag(_))),
                Some(tokens::Token::Operator(operators::SspOperator::FlagOperation(op))),
            ) => Ok(vec![0x94, op.value()]),

            // **** Load instructions ****

            // LD d, s
            (
                Some(mnemonics::SspMnemonic::Ld(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(dst))),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(src))),
            ) => Ok(vec![0, (dst.value() << 4) + src.value()]),

            // LD d, (ri)
            (
                Some(mnemonics::SspMnemonic::Ld(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(dst))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(src))),
            ) => Ok(vec![
                0x2 + src.ram_bank(),
                (dst.value() << 4) + (src.modifier_value() << 2) + src.value(),
            ]),

            // LD (ri), s
            (
                Some(mnemonics::SspMnemonic::Ld(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(dst))),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(src))),
            ) => Ok(vec![
                0x4 + dst.ram_bank(),
                (src.value() << 4) + (dst.modifier_value() << 2) + dst.value(),
            ]),

            // LDI d, imm
            (
                Some(mnemonics::SspMnemonic::Ld(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(dst))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(value))),
            ) => Ok(Instruction::opcodes_with_base_and_imm_value(
                vec![0x8, dst.value() << 4],
                value,
            )),

            // LD d, ((ri))
            (
                Some(mnemonics::SspMnemonic::Ld(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(dst))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrDoubleRef(src))),
            ) => Ok(vec![
                0xA + src.ram_bank(),
                (dst.value() << 4) + (src.modifier_value() << 2) + src.value(),
            ]),

            // LDI (ri), imm
            (
                Some(mnemonics::SspMnemonic::Ld(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(dst))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(value))),
            ) => Ok(Instruction::opcodes_with_base_and_imm_value(
                vec![
                    0xC + dst.ram_bank(),
                    (dst.modifier_value() << 2) + dst.value(),
                ],
                value,
            )),

            // LD addr, a
            (
                Some(mnemonics::SspMnemonic::Ld(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Word(addr))),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
            ) => Ok(vec![
                0xE + (((addr & 0x100) >> 8) as u8),
                (addr & 0xFF) as u8,
            ]),

            (
                Some(mnemonics::SspMnemonic::Ld(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(addr))),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(
                    registers::SspGeneralRegister::A,
                ))),
            ) => Ok(vec![0xE, addr]),

            // LD d, ri
            (
                Some(mnemonics::SspMnemonic::Ld(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(dst))),
                Some(tokens::Token::Operator(operators::SspOperator::Ptr(src))),
            ) => Ok(vec![
                0x12 + src.ram_bank(),
                (dst.value() << 4) + src.value(),
            ]),

            // LD ri, s
            (
                Some(mnemonics::SspMnemonic::Ld(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Ptr(dst))),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(src))),
            ) => Ok(vec![
                0x14 + dst.ram_bank(),
                (src.value() << 4) + dst.value(),
            ]),

            // LDI ri, simm
            (
                Some(mnemonics::SspMnemonic::Ld(mnemonics::SspMnemonicModifier::Immediate)),
                Some(tokens::Token::Operator(operators::SspOperator::Ptr(dst))),
                Some(tokens::Token::Operator(operators::SspOperator::Byte(value))),
            ) => Ok(vec![0x18 + (dst.ram_bank() << 2) + dst.value(), value]),

            // LD d, (a)
            (
                Some(mnemonics::SspMnemonic::Ld(mnemonics::SspMnemonicModifier::Reference)),
                Some(tokens::Token::Operator(operators::SspOperator::Reg(dst))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrAccumulator)),
            ) => Ok(vec![0x4A, dst.value() << 4]),

            // **** Program control ****

            // CALL cond, addr
            (
                Some(mnemonics::SspMnemonic::Call),
                Some(tokens::Token::Operator(operators::SspOperator::Condition(cond))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(addr))),
            ) => Ok(Instruction::opcodes_with_base_and_imm_value(
                vec![0x48 + cond.flag_value(), (cond.value() << 4)],
                addr,
            )),

            // BRA cond, addr
            (
                Some(mnemonics::SspMnemonic::Bra),
                Some(tokens::Token::Operator(operators::SspOperator::Condition(cond))),
                Some(tokens::Token::Operator(operators::SspOperator::Word(addr))),
            ) => Ok(Instruction::opcodes_with_base_and_imm_value(
                vec![0x4C + cond.flag_value(), (cond.value() << 4)],
                addr,
            )),

            // MOD cond, op
            (
                Some(mnemonics::SspMnemonic::Mod),
                Some(tokens::Token::Operator(operators::SspOperator::Condition(cond))),
                Some(tokens::Token::Operator(operators::SspOperator::AccOperation(op))),
            ) => Ok(vec![
                0x90 + cond.flag_value(),
                (cond.value() << 4) + op.value(),
            ]),

            // **** Multiply/accumulate ****

            // MLD (rj), (ri)
            (
                Some(mnemonics::SspMnemonic::Mld),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(dst))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(src))),
            ) if src.ram_bank() == 0 && dst.ram_bank() == 1 => Ok(vec![
                0xB7,
                (dst.modifier_value() << 6)
                    + (dst.value() << 4)
                    + (src.modifier_value() << 2)
                    + src.value(),
            ]),

            // MPYA (rj), (ri)
            (
                Some(mnemonics::SspMnemonic::Mpya),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(dst))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(src))),
            ) if src.ram_bank() == 0 && dst.ram_bank() == 1 => Ok(vec![
                0x97,
                (dst.modifier_value() << 6)
                    + (dst.value() << 4)
                    + (src.modifier_value() << 2)
                    + src.value(),
            ]),

            // MPYS (rj), (ri)
            (
                Some(mnemonics::SspMnemonic::Mpys),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(dst))),
                Some(tokens::Token::Operator(operators::SspOperator::PtrRef(src))),
            ) if src.ram_bank() == 0 && dst.ram_bank() == 1 => Ok(vec![
                0x37,
                (dst.modifier_value() << 6)
                    + (dst.value() << 4)
                    + (src.modifier_value() << 2)
                    + src.value(),
            ]),

            _ => Err(Box::new(errors::AssemblyError(format!(
                "Invalid instruction: {:?}",
                self
            )))),
        }
    }

    pub fn word_to_bytes(word: u16) -> Vec<u8> {
        vec![((word & 0xFF00) >> 8) as u8, (word & 0xFF) as u8]
    }

    fn opcodes_with_base_and_imm_value(mut base: Vec<u8>, value: u16) -> Vec<u8> {
        base.append(&mut Instruction::word_to_bytes(value));
        base
    }
}

#[cfg(test)]
mod opcode_tests {
    use super::*;

    fn check_inst(inst: &Instruction, expected_result: &Vec<u8>) {
        let result = inst.build();
        assert!(result.is_ok(), format!("Should compile: {:?}", inst));

        let opcodes = result.unwrap();
        assert_eq!(opcodes, *expected_result, "Wrong opcodes for {:?}", inst);
    }

    #[test]
    fn check_ret() {
        let inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Ret),
            op1: None,
            op2: None,
        };
        check_inst(&inst, &vec![0, 0x65]);
    }

    #[test]
    fn check_opi() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Sub(
                mnemonics::SspMnemonicModifier::Immediate,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Byte(
                0x50 as u8,
            ))),
            op2: None,
        };
        check_inst(&inst, &vec![0x38, 0x50]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Cmp(
            mnemonics::SspMnemonicModifier::Immediate,
        ));
        check_inst(&inst, &vec![0x78, 0x50]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Add(
            mnemonics::SspMnemonicModifier::Immediate,
        ));
        check_inst(&inst, &vec![0x98, 0x50]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::And(
            mnemonics::SspMnemonicModifier::Immediate,
        ));
        check_inst(&inst, &vec![0xB8, 0x50]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Or(
            mnemonics::SspMnemonicModifier::Immediate,
        ));
        check_inst(&inst, &vec![0xD8, 0x50]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Eor(
            mnemonics::SspMnemonicModifier::Immediate,
        ));
        check_inst(&inst, &vec![0xF8, 0x50]);
    }

    #[test]
    fn check_op_imm() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Sub(
                mnemonics::SspMnemonicModifier::Immediate,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::A,
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::Word(
                0x5050,
            ))),
        };
        check_inst(&inst, &vec![0x28, 0x00, 0x50, 0x50]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Cmp(
            mnemonics::SspMnemonicModifier::Immediate,
        ));
        check_inst(&inst, &vec![0x68, 0x00, 0x50, 0x50]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Add(
            mnemonics::SspMnemonicModifier::Immediate,
        ));
        check_inst(&inst, &vec![0x88, 0x00, 0x50, 0x50]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::And(
            mnemonics::SspMnemonicModifier::Immediate,
        ));
        check_inst(&inst, &vec![0xA8, 0x00, 0x50, 0x50]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Or(
            mnemonics::SspMnemonicModifier::Immediate,
        ));
        check_inst(&inst, &vec![0xC8, 0x00, 0x50, 0x50]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Eor(
            mnemonics::SspMnemonicModifier::Immediate,
        ));
        check_inst(&inst, &vec![0xE8, 0x00, 0x50, 0x50]);
    }

    #[test]
    fn check_op_a_reg() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Sub(
                mnemonics::SspMnemonicModifier::Reference,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::A,
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::X,
            ))),
        };
        check_inst(&inst, &vec![0x20, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Cmp(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0x60, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Add(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0x80, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::And(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xA0, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Or(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xC0, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Eor(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xE0, 0x01]);
    }

    #[test]
    fn check_op_a_ptr() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Sub(
                mnemonics::SspMnemonicModifier::Reference,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::A,
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::Ptr(
                registers::SspPointerRegister::R1(registers::SspModifier::Zero),
            ))),
        };
        check_inst(&inst, &vec![0x32, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Cmp(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0x72, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Add(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0x92, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::And(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xB2, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Or(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xD2, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Eor(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xF2, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Sub(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::Ptr(
            registers::SspPointerRegister::R5(registers::SspModifier::Zero),
        )));
        check_inst(&inst, &vec![0x33, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Cmp(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0x73, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Add(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0x93, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::And(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xB3, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Or(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xD3, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Eor(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xF3, 0x01]);
    }

    #[test]
    fn check_op_a_ptr_ref() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Sub(
                mnemonics::SspMnemonicModifier::Reference,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::A,
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
                registers::SspPointerRegister::R1(registers::SspModifier::Zero),
            ))),
        };
        check_inst(&inst, &vec![0x22, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Cmp(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0x62, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Add(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0x82, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::And(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xA2, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Or(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xC2, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Eor(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xE2, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Sub(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R1(registers::SspModifier::PostIncrement),
        )));
        check_inst(&inst, &vec![0x22, 0x0D]);

        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R1(registers::SspModifier::PostIncrementModulo),
        )));
        check_inst(&inst, &vec![0x22, 0x05]);

        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R1(registers::SspModifier::PostDecrementModulo),
        )));
        check_inst(&inst, &vec![0x22, 0x09]);

        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R5(registers::SspModifier::PostIncrement),
        )));
        check_inst(&inst, &vec![0x23, 0x0D]);
    }

    #[test]
    fn check_op_a_dbl_ptr_ref() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Sub(
                mnemonics::SspMnemonicModifier::Reference,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::A,
            ))),
            op2: Some(tokens::Token::Operator(
                operators::SspOperator::PtrDoubleRef(registers::SspPointerRegister::R1(
                    registers::SspModifier::Zero,
                )),
            )),
        };
        check_inst(&inst, &vec![0x2A, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Cmp(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0x6A, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Add(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0x8A, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::And(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xAA, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Or(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xCA, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Eor(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xEA, 0x01]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Sub(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        inst.op2 = Some(tokens::Token::Operator(
            operators::SspOperator::PtrDoubleRef(registers::SspPointerRegister::R1(
                registers::SspModifier::PostIncrement,
            )),
        ));
        check_inst(&inst, &vec![0x2A, 0x0D]);

        inst.op2 = Some(tokens::Token::Operator(
            operators::SspOperator::PtrDoubleRef(registers::SspPointerRegister::R1(
                registers::SspModifier::PostIncrementModulo,
            )),
        ));
        check_inst(&inst, &vec![0x2A, 0x05]);

        inst.op2 = Some(tokens::Token::Operator(
            operators::SspOperator::PtrDoubleRef(registers::SspPointerRegister::R1(
                registers::SspModifier::PostDecrementModulo,
            )),
        ));
        check_inst(&inst, &vec![0x2A, 0x09]);

        inst.op2 = Some(tokens::Token::Operator(
            operators::SspOperator::PtrDoubleRef(registers::SspPointerRegister::R5(
                registers::SspModifier::PostIncrement,
            )),
        ));
        check_inst(&inst, &vec![0x2B, 0x0D]);
    }

    #[test]
    fn check_op_a_addr() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Sub(
                mnemonics::SspMnemonicModifier::Reference,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::A,
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::Word(0xFF))),
        };
        check_inst(&inst, &vec![0x26, 0xFF]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Cmp(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0x66, 0xFF]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Add(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0x86, 0xFF]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::And(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xA6, 0xFF]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Or(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xC6, 0xFF]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Eor(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        check_inst(&inst, &vec![0xE6, 0xFF]);

        inst.mnemonic = Some(mnemonics::SspMnemonic::Sub(
            mnemonics::SspMnemonicModifier::Reference,
        ));
        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::Word(0x1FF)));
        check_inst(&inst, &vec![0x27, 0xFF]);
    }

    #[test]
    fn check_mod_f() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Mod),
            op1: Some(tokens::Token::Operator(
                operators::SspOperator::ConditionFlag(operators::SspMnemonicConditionFlag::F),
            )),
            op2: Some(tokens::Token::Operator(
                operators::SspOperator::FlagOperation(operators::SspMnemonicFlagOperation::Resl),
            )),
        };
        check_inst(&inst, &vec![0x94, 0x02]);

        inst.op2 = Some(tokens::Token::Operator(
            operators::SspOperator::FlagOperation(operators::SspMnemonicFlagOperation::Setl),
        ));
        check_inst(&inst, &vec![0x94, 0x03]);

        inst.op2 = Some(tokens::Token::Operator(
            operators::SspOperator::FlagOperation(operators::SspMnemonicFlagOperation::Resie),
        ));
        check_inst(&inst, &vec![0x94, 0x04]);

        inst.op2 = Some(tokens::Token::Operator(
            operators::SspOperator::FlagOperation(operators::SspMnemonicFlagOperation::Setie),
        ));
        check_inst(&inst, &vec![0x94, 0x05]);

        inst.op2 = Some(tokens::Token::Operator(
            operators::SspOperator::FlagOperation(operators::SspMnemonicFlagOperation::Resop),
        ));
        check_inst(&inst, &vec![0x94, 0x08]);

        inst.op2 = Some(tokens::Token::Operator(
            operators::SspOperator::FlagOperation(operators::SspMnemonicFlagOperation::Setop),
        ));
        check_inst(&inst, &vec![0x94, 0x09]);

        inst.op2 = Some(tokens::Token::Operator(
            operators::SspOperator::FlagOperation(operators::SspMnemonicFlagOperation::Res),
        ));
        check_inst(&inst, &vec![0x94, 0x0E]);

        inst.op2 = Some(tokens::Token::Operator(
            operators::SspOperator::FlagOperation(operators::SspMnemonicFlagOperation::Set),
        ));
        check_inst(&inst, &vec![0x94, 0x0F]);
    }

    #[test]
    fn check_ld_r_r() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Ld(
                mnemonics::SspMnemonicModifier::Reference,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::Dummy,
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::Dummy,
            ))),
        };
        check_inst(&inst, &vec![0x00, 0x00]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Reg(
            registers::SspGeneralRegister::X,
        )));
        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::Reg(
            registers::SspGeneralRegister::A,
        )));
        check_inst(&inst, &vec![0x00, 0x13]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Reg(
            registers::SspGeneralRegister::A,
        )));
        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::Reg(
            registers::SspGeneralRegister::Ext1,
        )));
        check_inst(&inst, &vec![0x00, 0x39]);
    }

    #[test]
    fn check_ld_r_ri() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Ld(
                mnemonics::SspMnemonicModifier::Reference,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::A,
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::Ptr(
                registers::SspPointerRegister::R5(registers::SspModifier::Zero),
            ))),
        };
        check_inst(&inst, &vec![0x13, 0x31]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Reg(
            registers::SspGeneralRegister::Ext1,
        )));
        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::Ptr(
            registers::SspPointerRegister::R6(registers::SspModifier::Zero),
        )));
        check_inst(&inst, &vec![0x13, 0x92]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Reg(
            registers::SspGeneralRegister::A,
        )));
        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::Ptr(
            registers::SspPointerRegister::R2(registers::SspModifier::Zero),
        )));
        check_inst(&inst, &vec![0x12, 0x32]);
    }

    #[test]
    fn check_ld_ri_r() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Ld(
                mnemonics::SspMnemonicModifier::Reference,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Ptr(
                registers::SspPointerRegister::R2(registers::SspModifier::Zero),
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::A,
            ))),
        };
        check_inst(&inst, &vec![0x14, 0x32]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Ptr(
            registers::SspPointerRegister::R6(registers::SspModifier::Zero),
        )));
        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::Reg(
            registers::SspGeneralRegister::Ext1,
        )));
        check_inst(&inst, &vec![0x15, 0x92]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Ptr(
            registers::SspPointerRegister::R0(registers::SspModifier::Zero),
        )));
        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::Reg(
            registers::SspGeneralRegister::X,
        )));
        check_inst(&inst, &vec![0x14, 0x10]);
    }

    #[test]
    fn check_ldi_r() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Ld(
                mnemonics::SspMnemonicModifier::Immediate,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::X,
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::Word(
                0x5050,
            ))),
        };
        check_inst(&inst, &vec![0x08, 0x10, 0x50, 0x50]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Reg(
            registers::SspGeneralRegister::A,
        )));
        check_inst(&inst, &vec![0x08, 0x30, 0x50, 0x50]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Reg(
            registers::SspGeneralRegister::Ext6,
        )));
        check_inst(&inst, &vec![0x08, 0xE0, 0x50, 0x50]);
    }

    #[test]
    fn check_ld_r_double_ref() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Ld(
                mnemonics::SspMnemonicModifier::Reference,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::A,
            ))),
            op2: Some(tokens::Token::Operator(
                operators::SspOperator::PtrDoubleRef(registers::SspPointerRegister::R0(
                    registers::SspModifier::Zero,
                )),
            )),
        };
        check_inst(&inst, &vec![0x0A, 0x30]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Reg(
            registers::SspGeneralRegister::X,
        )));
        inst.op2 = Some(tokens::Token::Operator(
            operators::SspOperator::PtrDoubleRef(registers::SspPointerRegister::R3(
                registers::SspModifier::Zero,
            )),
        ));
        check_inst(&inst, &vec![0x0A, 0x13]);

        inst.op2 = Some(tokens::Token::Operator(
            operators::SspOperator::PtrDoubleRef(registers::SspPointerRegister::R7(
                registers::SspModifier::Zero,
            )),
        ));
        check_inst(&inst, &vec![0x0B, 0x13]);
    }

    #[test]
    fn check_ldi_pointer_ref() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Ld(
                mnemonics::SspMnemonicModifier::Immediate,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
                registers::SspPointerRegister::R0(registers::SspModifier::Zero),
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::Word(
                0x5050,
            ))),
        };
        check_inst(&inst, &vec![0x0C, 0x00, 0x50, 0x50]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R4(registers::SspModifier::Zero),
        )));
        check_inst(&inst, &vec![0x0D, 0x00, 0x50, 0x50]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R4(registers::SspModifier::PostIncrement),
        )));
        check_inst(&inst, &vec![0x0D, 0x0C, 0x50, 0x50]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R4(registers::SspModifier::PostIncrementModulo),
        )));
        check_inst(&inst, &vec![0x0D, 0x04, 0x50, 0x50]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R4(registers::SspModifier::PostDecrementModulo),
        )));
        check_inst(&inst, &vec![0x0D, 0x08, 0x50, 0x50]);
    }

    #[test]
    fn check_ld_addr_a() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Ld(
                mnemonics::SspMnemonicModifier::Reference,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Word(
                0x0050,
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::A,
            ))),
        };
        check_inst(&inst, &vec![0x0E, 0x50]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Word(
            0x0150,
        )));
        check_inst(&inst, &vec![0x0F, 0x50]);
    }

    #[test]
    fn check_ld_r_ri_ref() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Ld(
                mnemonics::SspMnemonicModifier::Reference,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::Y,
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
                registers::SspPointerRegister::R0(registers::SspModifier::PostIncrementModulo),
            ))),
        };
        check_inst(&inst, &vec![0x02, 0x24]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Reg(
            registers::SspGeneralRegister::A,
        )));
        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R3(registers::SspModifier::Bank3),
        )));
        check_inst(&inst, &vec![0x02, 0x3F]);

        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R7(registers::SspModifier::Bank0),
        )));
        check_inst(&inst, &vec![0x03, 0x33]);
    }

    #[test]
    fn check_ld_ri_ref_r() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Ld(
                mnemonics::SspMnemonicModifier::Reference,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
                registers::SspPointerRegister::R0(registers::SspModifier::PostIncrementModulo),
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::Y,
            ))),
        };
        check_inst(&inst, &vec![0x04, 0x24]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R3(registers::SspModifier::Bank3),
        )));
        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::Reg(
            registers::SspGeneralRegister::A,
        )));
        check_inst(&inst, &vec![0x04, 0x3F]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R7(registers::SspModifier::Bank0),
        )));
        check_inst(&inst, &vec![0x05, 0x33]);
    }

    #[test]
    fn check_ldi_ri() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Ld(
                mnemonics::SspMnemonicModifier::Immediate,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Ptr(
                registers::SspPointerRegister::R0(registers::SspModifier::PostIncrementModulo),
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::Byte(0x50))),
        };
        check_inst(&inst, &vec![0x18, 0x50]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Ptr(
            registers::SspPointerRegister::R6(registers::SspModifier::Zero),
        )));
        check_inst(&inst, &vec![0x1E, 0x50]);
    }

    #[test]
    fn check_ld_r_ref_a() {
        let inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Ld(
                mnemonics::SspMnemonicModifier::Reference,
            )),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Reg(
                registers::SspGeneralRegister::X,
            ))),
            op2: Some(tokens::Token::Operator(
                operators::SspOperator::PtrAccumulator,
            )),
        };
        check_inst(&inst, &vec![0x4A, 0x10]);
    }

    #[test]
    fn check_call() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Call),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Condition(
                operators::SspMnemonicCondition::Always,
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::Word(
                0x5050,
            ))),
        };
        check_inst(&inst, &vec![0x48, 0x00, 0x50, 0x50]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Condition(
            operators::SspMnemonicCondition::Z(true),
        )));
        check_inst(&inst, &vec![0x49, 0x50, 0x50, 0x50]);
    }

    #[test]
    fn check_bra() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Bra),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Condition(
                operators::SspMnemonicCondition::Always,
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::Word(
                0x5050,
            ))),
        };
        check_inst(&inst, &vec![0x4C, 0x00, 0x50, 0x50]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Condition(
            operators::SspMnemonicCondition::Z(true),
        )));
        check_inst(&inst, &vec![0x4D, 0x50, 0x50, 0x50]);
    }

    #[test]
    fn check_mod_cond() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Mod),
            op1: Some(tokens::Token::Operator(operators::SspOperator::Condition(
                operators::SspMnemonicCondition::Always,
            ))),
            op2: Some(tokens::Token::Operator(
                operators::SspOperator::AccOperation(operators::SspMnemonicAccOperation::Shl),
            )),
        };
        check_inst(&inst, &vec![0x90, 0x03]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::Condition(
            operators::SspMnemonicCondition::Z(false),
        )));
        inst.op2 = Some(tokens::Token::Operator(
            operators::SspOperator::AccOperation(operators::SspMnemonicAccOperation::Neg),
        ));
        check_inst(&inst, &vec![0x90, 0x56]);
    }

    #[test]
    fn check_mld() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Mld),
            op1: Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
                registers::SspPointerRegister::R4(registers::SspModifier::PostIncrementModulo),
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
                registers::SspPointerRegister::R0(registers::SspModifier::PostIncrementModulo),
            ))),
        };
        check_inst(&inst, &vec![0xB7, 0x44]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R7(registers::SspModifier::Bank2),
        )));
        check_inst(&inst, &vec![0xB7, 0xB4]);
    }

    #[test]
    fn check_mpya() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Mpya),
            op1: Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
                registers::SspPointerRegister::R4(registers::SspModifier::PostIncrementModulo),
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
                registers::SspPointerRegister::R0(registers::SspModifier::PostIncrementModulo),
            ))),
        };
        check_inst(&inst, &vec![0x97, 0x44]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R7(registers::SspModifier::Bank0),
        )));
        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R3(registers::SspModifier::Bank0),
        )));
        check_inst(&inst, &vec![0x97, 0x33]);
    }

    #[test]
    fn check_mpys() {
        let mut inst = Instruction {
            mnemonic: Some(mnemonics::SspMnemonic::Mpys),
            op1: Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
                registers::SspPointerRegister::R4(registers::SspModifier::PostIncrementModulo),
            ))),
            op2: Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
                registers::SspPointerRegister::R0(registers::SspModifier::PostIncrementModulo),
            ))),
        };
        check_inst(&inst, &vec![0x37, 0x44]);

        inst.op1 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R7(registers::SspModifier::Bank0),
        )));
        inst.op2 = Some(tokens::Token::Operator(operators::SspOperator::PtrRef(
            registers::SspPointerRegister::R3(registers::SspModifier::Bank0),
        )));
        check_inst(&inst, &vec![0x37, 0x33]);
    }
}
