use super::labels;
use super::registers::SspGeneralRegister;
use super::registers::SspPointerRegister;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SspMnemonicCondition {
    Always,
    Gpi0(bool),
    Gpi1(bool),
    L(bool),
    Z(bool),
    Ov(bool),
    N(bool),
    Diof(bool),
    Gpi2(bool),
    Gpi3(bool),
}

impl SspMnemonicCondition {
    pub fn new(cond: &str) -> Option<SspMnemonicCondition> {
        match cond.to_lowercase().as_str() {
            "always" => Some(SspMnemonicCondition::Always),
            "gpi0" => Some(SspMnemonicCondition::Gpi0(false)),
            "gpi0=0" => Some(SspMnemonicCondition::Gpi0(false)),
            "gpi0=1" => Some(SspMnemonicCondition::Gpi0(true)),
            "gpi1" => Some(SspMnemonicCondition::Gpi1(false)),
            "gpi1=0" => Some(SspMnemonicCondition::Gpi1(false)),
            "gpi1=1" => Some(SspMnemonicCondition::Gpi1(true)),
            "l" => Some(SspMnemonicCondition::L(false)),
            "l=0" => Some(SspMnemonicCondition::L(false)),
            "l=1" => Some(SspMnemonicCondition::L(true)),
            "z" => Some(SspMnemonicCondition::Z(false)),
            "z=0" => Some(SspMnemonicCondition::Z(false)),
            "z=1" => Some(SspMnemonicCondition::Z(true)),
            "ov" => Some(SspMnemonicCondition::Ov(false)),
            "ov=0" => Some(SspMnemonicCondition::Ov(false)),
            "ov=1" => Some(SspMnemonicCondition::Ov(true)),
            "n" => Some(SspMnemonicCondition::N(false)),
            "n=0" => Some(SspMnemonicCondition::N(false)),
            "n=1" => Some(SspMnemonicCondition::N(true)),
            "diof" => Some(SspMnemonicCondition::Diof(false)),
            "diof=0" => Some(SspMnemonicCondition::Diof(false)),
            "diof=1" => Some(SspMnemonicCondition::Diof(true)),
            "gpi2" => Some(SspMnemonicCondition::Gpi2(false)),
            "gpi2=0" => Some(SspMnemonicCondition::Gpi2(false)),
            "gpi2=1" => Some(SspMnemonicCondition::Gpi2(true)),
            "gpi3" => Some(SspMnemonicCondition::Gpi3(false)),
            "gpi3=0" => Some(SspMnemonicCondition::Gpi3(false)),
            "gpi3=1" => Some(SspMnemonicCondition::Gpi3(true)),
            _ => None,
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            SspMnemonicCondition::Always => 0,
            SspMnemonicCondition::Gpi0(_) => 2,
            SspMnemonicCondition::Gpi1(_) => 3,
            SspMnemonicCondition::L(_) => 4,
            SspMnemonicCondition::Z(_) => 5,
            SspMnemonicCondition::Ov(_) => 6,
            SspMnemonicCondition::N(_) => 7,
            SspMnemonicCondition::Diof(_) => 8,
            SspMnemonicCondition::Gpi2(_) => 9,
            SspMnemonicCondition::Gpi3(_) => 10,
        }
    }

    pub fn flag_value(&self) -> u8 {
        match self {
            SspMnemonicCondition::Always => 0,
            SspMnemonicCondition::Gpi0(false) => 0,
            SspMnemonicCondition::Gpi0(true) => 1,
            SspMnemonicCondition::Gpi1(false) => 0,
            SspMnemonicCondition::Gpi1(true) => 1,
            SspMnemonicCondition::L(false) => 0,
            SspMnemonicCondition::L(true) => 1,
            SspMnemonicCondition::Z(false) => 0,
            SspMnemonicCondition::Z(true) => 1,
            SspMnemonicCondition::Ov(false) => 0,
            SspMnemonicCondition::Ov(true) => 1,
            SspMnemonicCondition::N(false) => 0,
            SspMnemonicCondition::N(true) => 1,
            SspMnemonicCondition::Diof(false) => 0,
            SspMnemonicCondition::Diof(true) => 1,
            SspMnemonicCondition::Gpi2(false) => 0,
            SspMnemonicCondition::Gpi2(true) => 1,
            SspMnemonicCondition::Gpi3(false) => 0,
            SspMnemonicCondition::Gpi3(true) => 1,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SspMnemonicConditionFlag {
    F,
}

impl SspMnemonicConditionFlag {
    pub fn new(cond: &str) -> Option<SspMnemonicConditionFlag> {
        match cond.to_lowercase().as_str() {
            "f" => Some(SspMnemonicConditionFlag::F),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SspMnemonicAccOperation {
    Ror,
    Rol,
    Shr,
    Shl,
    Inc,
    Dec,
    Neg,
    Abs,
}

impl SspMnemonicAccOperation {
    pub fn new(cond: &str) -> Option<SspMnemonicAccOperation> {
        match cond.to_lowercase().as_str() {
            "ror" => Some(SspMnemonicAccOperation::Ror),
            "rol" => Some(SspMnemonicAccOperation::Rol),
            "shr" => Some(SspMnemonicAccOperation::Shr),
            "shl" => Some(SspMnemonicAccOperation::Shl),
            "inc" => Some(SspMnemonicAccOperation::Inc),
            "dec" => Some(SspMnemonicAccOperation::Dec),
            "neg" => Some(SspMnemonicAccOperation::Neg),
            "abs" => Some(SspMnemonicAccOperation::Abs),
            _ => None,
        }
    }
}

// "ror", "rol", "shr", "shl", "inc", "dec", "neg", "abs"
impl SspMnemonicAccOperation {
    pub fn value(&self) -> u8 {
        match self {
            SspMnemonicAccOperation::Ror => 0,
            SspMnemonicAccOperation::Rol => 1,
            SspMnemonicAccOperation::Shr => 2,
            SspMnemonicAccOperation::Shl => 3,
            SspMnemonicAccOperation::Inc => 4,
            SspMnemonicAccOperation::Dec => 5,
            SspMnemonicAccOperation::Neg => 6,
            SspMnemonicAccOperation::Abs => 7,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SspMnemonicFlagOperation {
    Resl,
    Setl,
    Resie,
    Setie,
    Resop,
    Setop,
    Res,
    Set,
}

impl SspMnemonicFlagOperation {
    pub fn new(fop: &str) -> Option<SspMnemonicFlagOperation> {
        match fop.to_lowercase().as_str() {
            "resl" => Some(SspMnemonicFlagOperation::Resl),
            "setl" => Some(SspMnemonicFlagOperation::Setl),
            "resie" => Some(SspMnemonicFlagOperation::Resie),
            "setie" => Some(SspMnemonicFlagOperation::Setie),
            "resop" => Some(SspMnemonicFlagOperation::Resop),
            "setop" => Some(SspMnemonicFlagOperation::Setop),
            "res" => Some(SspMnemonicFlagOperation::Res),
            "set" => Some(SspMnemonicFlagOperation::Set),
            _ => None,
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            SspMnemonicFlagOperation::Resl => 2,
            SspMnemonicFlagOperation::Setl => 3,
            SspMnemonicFlagOperation::Resie => 4,
            SspMnemonicFlagOperation::Setie => 5,
            SspMnemonicFlagOperation::Resop => 8,
            SspMnemonicFlagOperation::Setop => 9,
            SspMnemonicFlagOperation::Res => 14,
            SspMnemonicFlagOperation::Set => 15,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum SspOperator<'a> {
    Reg(SspGeneralRegister),
    Ptr(SspPointerRegister),
    PtrRef(SspPointerRegister),
    PtrDoubleRef(SspPointerRegister),
    PtrAccumulator,
    Condition(SspMnemonicCondition),
    ConditionFlag(SspMnemonicConditionFlag),
    FlagOperation(SspMnemonicFlagOperation),
    AccOperation(SspMnemonicAccOperation),
    Word(u16),
    Byte(u8),
    LabelRef(&'a str),
}

impl<'a> SspOperator<'a> {
    pub fn new(op: &str) -> Option<SspOperator> {
        match op.to_lowercase().as_str() {
            // Double references to pointer registers
            _ if op.starts_with("((") => {
                match SspPointerRegister::new(op.trim_start_matches("(").trim_end_matches(")")) {
                    Some(preg) => Some(SspOperator::PtrDoubleRef(preg)),
                    None => None,
                }
            }

            // References to pointer registers
            _ if op.starts_with("(") => {
                match SspPointerRegister::new(op.trim_start_matches("(").trim_end_matches(")")) {
                    Some(preg) => Some(SspOperator::PtrRef(preg)),
                    None if op == "(a)" => Some(SspOperator::PtrAccumulator), // Special case
                    None => None,
                }
            }

            // Label references
            _ if labels::is_label_ref(op) => {
                Some(SspOperator::LabelRef(op.trim_start_matches("@")))
            }

            // Pointer registers
            _ if SspPointerRegister::new(op).is_some() => {
                Some(SspOperator::Ptr(SspPointerRegister::new(op).unwrap()))
            }

            // General registers
            _ if SspGeneralRegister::new(op).is_some() => {
                Some(SspOperator::Reg(SspGeneralRegister::new(op).unwrap()))
            }

            // Conditions
            _ if SspMnemonicCondition::new(op).is_some() => Some(SspOperator::Condition(
                SspMnemonicCondition::new(op).unwrap(),
            )),
            _ if SspMnemonicConditionFlag::new(op).is_some() => Some(SspOperator::ConditionFlag(
                SspMnemonicConditionFlag::new(op).unwrap(),
            )),

            // Operations on accumulator
            _ if SspMnemonicAccOperation::new(op).is_some() => Some(SspOperator::AccOperation(
                SspMnemonicAccOperation::new(op).unwrap(),
            )),

            // Flag operations
            _ if SspMnemonicFlagOperation::new(op).is_some() => Some(SspOperator::FlagOperation(
                SspMnemonicFlagOperation::new(op).unwrap(),
            )),

            // Immediate hexadecimal words/bytes
            _ if op.starts_with("0x") || op.ends_with("h") || (op.len() > 0 && op.len() <= 4) => {
                let clean_op = op.trim_start_matches("0x").trim_end_matches("h");

                if clean_op.len() > 0 && clean_op.len() <= 2 {
                    match u8::from_str_radix(clean_op, 16) {
                        Ok(num) => Some(SspOperator::Byte(num)),
                        Err(_) => None,
                    }
                } else if clean_op.len() <= 4 {
                    match u16::from_str_radix(clean_op, 16) {
                        Ok(num) => Some(SspOperator::Word(num)),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            }

            _ => None,
        }
    }
}

impl<'a> fmt::Debug for SspOperator<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SspOperator::Byte(value) => write!(f, "SspOperator(Byte({:02X}))", value),
            SspOperator::Word(value) => write!(f, "SspOperator(Byte({:02X}))", value),
            SspOperator::Reg(reg) => write!(f, "SspOperator(Reg({:?}))", reg),
            SspOperator::Ptr(reg) => write!(f, "SspOperator(Ptr({:?}))", reg),
            SspOperator::PtrRef(reg) => write!(f, "SspOperator(PtrRef({:?}))", reg),
            SspOperator::PtrDoubleRef(reg) => write!(f, "SspOperator(PtrDoubleRef({:?}))", reg),
            SspOperator::PtrAccumulator => write!(f, "SspOperator(PtrAccumulator)"),
            SspOperator::Condition(cond) => write!(f, "SspOperator(Condition({:?}))", cond),
            SspOperator::ConditionFlag(flag) => write!(f, "SspOperator(ConditionFlag({:?}))", flag),
            SspOperator::FlagOperation(op) => write!(f, "SspOperator(FlagOperation({:?}))", op),
            SspOperator::AccOperation(op) => write!(f, "SspOperator(AccOperation({:?}))", op),
            SspOperator::LabelRef(label) => write!(f, "SspOperator(LabelRef({:?}))", label),
        }
    }
}

#[cfg(test)]
mod invalid_tests {
    use super::*;

    #[test]
    fn check_invalid_operators() {
        assert_eq!(SspOperator::new("lol"), None);
        assert_eq!(SspOperator::new(""), None);
    }
}

#[cfg(test)]
mod general_registers_tests {
    use super::*;

    #[test]
    fn check_valid_general_registers() {
        assert_eq!(
            SspOperator::new("-"),
            Some(SspOperator::Reg(SspGeneralRegister::Dummy))
        );
        assert_eq!(
            SspOperator::new("x"),
            Some(SspOperator::Reg(SspGeneralRegister::X))
        );
        assert_eq!(
            SspOperator::new("y"),
            Some(SspOperator::Reg(SspGeneralRegister::Y))
        );
        assert_eq!(
            SspOperator::new("a"),
            Some(SspOperator::Reg(SspGeneralRegister::A))
        );
        assert_eq!(
            SspOperator::new("st"),
            Some(SspOperator::Reg(SspGeneralRegister::St))
        );
        assert_eq!(
            SspOperator::new("pc"),
            Some(SspOperator::Reg(SspGeneralRegister::Pc))
        );
        assert_eq!(
            SspOperator::new("p"),
            Some(SspOperator::Reg(SspGeneralRegister::P))
        );
        assert_eq!(
            SspOperator::new("ext0"),
            Some(SspOperator::Reg(SspGeneralRegister::Ext0))
        );
        assert_eq!(
            SspOperator::new("ext1"),
            Some(SspOperator::Reg(SspGeneralRegister::Ext1))
        );
        assert_eq!(
            SspOperator::new("ext2"),
            Some(SspOperator::Reg(SspGeneralRegister::Ext2))
        );
        assert_eq!(
            SspOperator::new("ext3"),
            Some(SspOperator::Reg(SspGeneralRegister::Ext3))
        );
        assert_eq!(
            SspOperator::new("ext4"),
            Some(SspOperator::Reg(SspGeneralRegister::Ext4))
        );
        assert_eq!(
            SspOperator::new("ext5"),
            Some(SspOperator::Reg(SspGeneralRegister::Ext5))
        );
        assert_eq!(
            SspOperator::new("ext6"),
            Some(SspOperator::Reg(SspGeneralRegister::Ext6))
        );
        assert_eq!(
            SspOperator::new("ext7"),
            Some(SspOperator::Reg(SspGeneralRegister::Ext7))
        );
    }
}

#[cfg(test)]
mod pointer_registers_tests {
    use super::super::registers::SspModifier;
    use super::*;

    #[test]
    fn check_valid_pointer_registers() {
        assert_eq!(
            SspOperator::new("r0"),
            Some(SspOperator::Ptr(SspPointerRegister::R0(SspModifier::Zero)))
        );
        assert_eq!(
            SspOperator::new("r1"),
            Some(SspOperator::Ptr(SspPointerRegister::R1(SspModifier::Zero)))
        );
        assert_eq!(
            SspOperator::new("r2"),
            Some(SspOperator::Ptr(SspPointerRegister::R2(SspModifier::Zero)))
        );
        assert_eq!(
            SspOperator::new("r3"),
            Some(SspOperator::Ptr(SspPointerRegister::R3(SspModifier::Zero)))
        );
        assert_eq!(
            SspOperator::new("r4"),
            Some(SspOperator::Ptr(SspPointerRegister::R4(SspModifier::Zero)))
        );
        assert_eq!(
            SspOperator::new("r5"),
            Some(SspOperator::Ptr(SspPointerRegister::R5(SspModifier::Zero)))
        );
        assert_eq!(
            SspOperator::new("r6"),
            Some(SspOperator::Ptr(SspPointerRegister::R6(SspModifier::Zero)))
        );
        assert_eq!(
            SspOperator::new("r7"),
            Some(SspOperator::Ptr(SspPointerRegister::R7(SspModifier::Zero)))
        );
        assert_eq!(
            SspOperator::new("r0+"),
            Some(SspOperator::Ptr(SspPointerRegister::R0(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("r1+"),
            Some(SspOperator::Ptr(SspPointerRegister::R1(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("r2+"),
            Some(SspOperator::Ptr(SspPointerRegister::R2(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("r4+"),
            Some(SspOperator::Ptr(SspPointerRegister::R4(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("r5+"),
            Some(SspOperator::Ptr(SspPointerRegister::R5(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("r6+"),
            Some(SspOperator::Ptr(SspPointerRegister::R6(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("r0-"),
            Some(SspOperator::Ptr(SspPointerRegister::R0(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("r1-"),
            Some(SspOperator::Ptr(SspPointerRegister::R1(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("r2-"),
            Some(SspOperator::Ptr(SspPointerRegister::R2(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("r4-"),
            Some(SspOperator::Ptr(SspPointerRegister::R4(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("r5-"),
            Some(SspOperator::Ptr(SspPointerRegister::R5(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("r6-"),
            Some(SspOperator::Ptr(SspPointerRegister::R6(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("r0+!"),
            Some(SspOperator::Ptr(SspPointerRegister::R0(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("r1+!"),
            Some(SspOperator::Ptr(SspPointerRegister::R1(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("r2+!"),
            Some(SspOperator::Ptr(SspPointerRegister::R2(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("r4+!"),
            Some(SspOperator::Ptr(SspPointerRegister::R4(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("r5+!"),
            Some(SspOperator::Ptr(SspPointerRegister::R5(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("r6+!"),
            Some(SspOperator::Ptr(SspPointerRegister::R6(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("r3|00"),
            Some(SspOperator::Ptr(SspPointerRegister::R3(SspModifier::Bank0)))
        );
        assert_eq!(
            SspOperator::new("r3|01"),
            Some(SspOperator::Ptr(SspPointerRegister::R3(SspModifier::Bank1)))
        );
        assert_eq!(
            SspOperator::new("r3|10"),
            Some(SspOperator::Ptr(SspPointerRegister::R3(SspModifier::Bank2)))
        );
        assert_eq!(
            SspOperator::new("r3|11"),
            Some(SspOperator::Ptr(SspPointerRegister::R3(SspModifier::Bank3)))
        );
        assert_eq!(
            SspOperator::new("r7|00"),
            Some(SspOperator::Ptr(SspPointerRegister::R7(SspModifier::Bank0)))
        );
        assert_eq!(
            SspOperator::new("r7|01"),
            Some(SspOperator::Ptr(SspPointerRegister::R7(SspModifier::Bank1)))
        );
        assert_eq!(
            SspOperator::new("r7|10"),
            Some(SspOperator::Ptr(SspPointerRegister::R7(SspModifier::Bank2)))
        );
        assert_eq!(
            SspOperator::new("r7|11"),
            Some(SspOperator::Ptr(SspPointerRegister::R7(SspModifier::Bank3)))
        );
    }

    #[test]
    fn check_valid_simple_references() {
        assert_eq!(
            SspOperator::new("(r0)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R0(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("(r1)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R1(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("(r2)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R2(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("(r3)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R3(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("(r4)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R4(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("(r5)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R5(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("(r6)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R6(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("(r7)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R7(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("(r0+)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R0(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("(r1+)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R1(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("(r2+)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R2(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("(r4+)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R4(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("(r5+)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R5(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("(r6+)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R6(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("(r0-)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R0(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("(r1-)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R1(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("(r2-)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R2(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("(r4-)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R4(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("(r5-)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R5(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("(r6-)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R6(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("(r0+!)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R0(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("(r1+!)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R1(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("(r2+!)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R2(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("(r4+!)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R4(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("(r5+!)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R5(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("(r6+!)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R6(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("(r3|00)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R3(
                SspModifier::Bank0
            )))
        );
        assert_eq!(
            SspOperator::new("(r3|01)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R3(
                SspModifier::Bank1
            )))
        );
        assert_eq!(
            SspOperator::new("(r3|10)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R3(
                SspModifier::Bank2
            )))
        );
        assert_eq!(
            SspOperator::new("(r3|11)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R3(
                SspModifier::Bank3
            )))
        );
        assert_eq!(
            SspOperator::new("(r7|00)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R7(
                SspModifier::Bank0
            )))
        );
        assert_eq!(
            SspOperator::new("(r7|01)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R7(
                SspModifier::Bank1
            )))
        );
        assert_eq!(
            SspOperator::new("(r7|10)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R7(
                SspModifier::Bank2
            )))
        );
        assert_eq!(
            SspOperator::new("(r7|11)"),
            Some(SspOperator::PtrRef(SspPointerRegister::R7(
                SspModifier::Bank3
            )))
        );
        assert_eq!(SspOperator::new("(a)"), Some(SspOperator::PtrAccumulator));
    }

    #[test]
    fn check_valid_double_references() {
        assert_eq!(
            SspOperator::new("((r0))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R0(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("((r1))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R1(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("((r2))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R2(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("((r3))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R3(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("((r4))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R4(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("((r5))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R5(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("((r6))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R6(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("((r7))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R7(
                SspModifier::Zero
            )))
        );
        assert_eq!(
            SspOperator::new("((r0+))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R0(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("((r1+))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R1(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("((r2+))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R2(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("((r4+))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R4(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("((r5+))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R5(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("((r6+))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R6(
                SspModifier::PostIncrement
            )))
        );
        assert_eq!(
            SspOperator::new("((r0-))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R0(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("((r1-))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R1(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("((r2-))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R2(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("((r4-))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R4(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("((r5-))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R5(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("((r6-))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R6(
                SspModifier::PostDecrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("((r0+!))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R0(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("((r1+!))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R1(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("((r2+!))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R2(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("((r4+!))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R4(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("((r5+!))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R5(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("((r6+!))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R6(
                SspModifier::PostIncrementModulo
            )))
        );
        assert_eq!(
            SspOperator::new("((r3|00))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R3(
                SspModifier::Bank0
            )))
        );
        assert_eq!(
            SspOperator::new("((r3|01))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R3(
                SspModifier::Bank1
            )))
        );
        assert_eq!(
            SspOperator::new("((r3|10))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R3(
                SspModifier::Bank2
            )))
        );
        assert_eq!(
            SspOperator::new("((r3|11))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R3(
                SspModifier::Bank3
            )))
        );
        assert_eq!(
            SspOperator::new("((r7|00))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R7(
                SspModifier::Bank0
            )))
        );
        assert_eq!(
            SspOperator::new("((r7|01))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R7(
                SspModifier::Bank1
            )))
        );
        assert_eq!(
            SspOperator::new("((r7|10))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R7(
                SspModifier::Bank2
            )))
        );
        assert_eq!(
            SspOperator::new("((r7|11))"),
            Some(SspOperator::PtrDoubleRef(SspPointerRegister::R7(
                SspModifier::Bank3
            )))
        );
    }
}

#[cfg(test)]
mod conditions_and_flags_tests {
    use super::*;

    #[test]
    fn check_conditions() {
        assert_eq!(
            SspOperator::new("always"),
            Some(SspOperator::Condition(SspMnemonicCondition::Always))
        );
        assert_eq!(
            SspOperator::new("gpi0"),
            Some(SspOperator::Condition(SspMnemonicCondition::Gpi0(false)))
        );
        assert_eq!(
            SspOperator::new("gpi0=0"),
            Some(SspOperator::Condition(SspMnemonicCondition::Gpi0(false)))
        );
        assert_eq!(
            SspOperator::new("gpi0=1"),
            Some(SspOperator::Condition(SspMnemonicCondition::Gpi0(true)))
        );
        assert_eq!(
            SspOperator::new("gpi1"),
            Some(SspOperator::Condition(SspMnemonicCondition::Gpi1(false)))
        );
        assert_eq!(
            SspOperator::new("gpi1=0"),
            Some(SspOperator::Condition(SspMnemonicCondition::Gpi1(false)))
        );
        assert_eq!(
            SspOperator::new("gpi1=1"),
            Some(SspOperator::Condition(SspMnemonicCondition::Gpi1(true)))
        );
        assert_eq!(
            SspOperator::new("gpi2"),
            Some(SspOperator::Condition(SspMnemonicCondition::Gpi2(false)))
        );
        assert_eq!(
            SspOperator::new("gpi2=0"),
            Some(SspOperator::Condition(SspMnemonicCondition::Gpi2(false)))
        );
        assert_eq!(
            SspOperator::new("gpi2=1"),
            Some(SspOperator::Condition(SspMnemonicCondition::Gpi2(true)))
        );
        assert_eq!(
            SspOperator::new("gpi3"),
            Some(SspOperator::Condition(SspMnemonicCondition::Gpi3(false)))
        );
        assert_eq!(
            SspOperator::new("gpi3=0"),
            Some(SspOperator::Condition(SspMnemonicCondition::Gpi3(false)))
        );
        assert_eq!(
            SspOperator::new("gpi3=1"),
            Some(SspOperator::Condition(SspMnemonicCondition::Gpi3(true)))
        );
        assert_eq!(
            SspOperator::new("l"),
            Some(SspOperator::Condition(SspMnemonicCondition::L(false)))
        );
        assert_eq!(
            SspOperator::new("l=0"),
            Some(SspOperator::Condition(SspMnemonicCondition::L(false)))
        );
        assert_eq!(
            SspOperator::new("l=1"),
            Some(SspOperator::Condition(SspMnemonicCondition::L(true)))
        );
        assert_eq!(
            SspOperator::new("z"),
            Some(SspOperator::Condition(SspMnemonicCondition::Z(false)))
        );
        assert_eq!(
            SspOperator::new("z=0"),
            Some(SspOperator::Condition(SspMnemonicCondition::Z(false)))
        );
        assert_eq!(
            SspOperator::new("z=1"),
            Some(SspOperator::Condition(SspMnemonicCondition::Z(true)))
        );
        assert_eq!(
            SspOperator::new("ov"),
            Some(SspOperator::Condition(SspMnemonicCondition::Ov(false)))
        );
        assert_eq!(
            SspOperator::new("ov=0"),
            Some(SspOperator::Condition(SspMnemonicCondition::Ov(false)))
        );
        assert_eq!(
            SspOperator::new("ov=1"),
            Some(SspOperator::Condition(SspMnemonicCondition::Ov(true)))
        );
        assert_eq!(
            SspOperator::new("n"),
            Some(SspOperator::Condition(SspMnemonicCondition::N(false)))
        );
        assert_eq!(
            SspOperator::new("n=0"),
            Some(SspOperator::Condition(SspMnemonicCondition::N(false)))
        );
        assert_eq!(
            SspOperator::new("n=1"),
            Some(SspOperator::Condition(SspMnemonicCondition::N(true)))
        );
        assert_eq!(
            SspOperator::new("diof"),
            Some(SspOperator::Condition(SspMnemonicCondition::Diof(false)))
        );
        assert_eq!(
            SspOperator::new("diof=0"),
            Some(SspOperator::Condition(SspMnemonicCondition::Diof(false)))
        );
        assert_eq!(
            SspOperator::new("diof=1"),
            Some(SspOperator::Condition(SspMnemonicCondition::Diof(true)))
        );
        assert_eq!(
            SspOperator::new("f"),
            Some(SspOperator::ConditionFlag(SspMnemonicConditionFlag::F))
        );
    }

    #[test]
    fn check_accumulator_operators() {
        assert_eq!(
            SspOperator::new("ror"),
            Some(SspOperator::AccOperation(SspMnemonicAccOperation::Ror))
        );
        assert_eq!(
            SspOperator::new("rol"),
            Some(SspOperator::AccOperation(SspMnemonicAccOperation::Rol))
        );
        assert_eq!(
            SspOperator::new("shr"),
            Some(SspOperator::AccOperation(SspMnemonicAccOperation::Shr))
        );
        assert_eq!(
            SspOperator::new("shl"),
            Some(SspOperator::AccOperation(SspMnemonicAccOperation::Shl))
        );
        assert_eq!(
            SspOperator::new("inc"),
            Some(SspOperator::AccOperation(SspMnemonicAccOperation::Inc))
        );
        assert_eq!(
            SspOperator::new("dec"),
            Some(SspOperator::AccOperation(SspMnemonicAccOperation::Dec))
        );
        assert_eq!(
            SspOperator::new("neg"),
            Some(SspOperator::AccOperation(SspMnemonicAccOperation::Neg))
        );
        assert_eq!(
            SspOperator::new("abs"),
            Some(SspOperator::AccOperation(SspMnemonicAccOperation::Abs))
        );
    }

    #[test]
    fn check_flag_operators() {
        assert_eq!(
            SspOperator::new("resl"),
            Some(SspOperator::FlagOperation(SspMnemonicFlagOperation::Resl))
        );
        assert_eq!(
            SspOperator::new("setl"),
            Some(SspOperator::FlagOperation(SspMnemonicFlagOperation::Setl))
        );
        assert_eq!(
            SspOperator::new("resie"),
            Some(SspOperator::FlagOperation(SspMnemonicFlagOperation::Resie))
        );
        assert_eq!(
            SspOperator::new("setie"),
            Some(SspOperator::FlagOperation(SspMnemonicFlagOperation::Setie))
        );
        assert_eq!(
            SspOperator::new("resop"),
            Some(SspOperator::FlagOperation(SspMnemonicFlagOperation::Resop))
        );
        assert_eq!(
            SspOperator::new("setop"),
            Some(SspOperator::FlagOperation(SspMnemonicFlagOperation::Setop))
        );
        assert_eq!(
            SspOperator::new("res"),
            Some(SspOperator::FlagOperation(SspMnemonicFlagOperation::Res))
        );
        assert_eq!(
            SspOperator::new("set"),
            Some(SspOperator::FlagOperation(SspMnemonicFlagOperation::Set))
        );
    }
}

#[cfg(test)]
mod numeric_value_tests {
    use super::*;

    #[test]
    fn check_bytes() {
        assert_eq!(SspOperator::new("0x0"), Some(SspOperator::Byte(0)));
        assert_eq!(SspOperator::new("0x42"), Some(SspOperator::Byte(0x42)));
        assert_eq!(SspOperator::new("0xff"), Some(SspOperator::Byte(0xff)));
        assert_eq!(SspOperator::new("0h"), Some(SspOperator::Byte(0)));
        assert_eq!(SspOperator::new("42h"), Some(SspOperator::Byte(0x42)));
        assert_eq!(SspOperator::new("ffh"), Some(SspOperator::Byte(0xff)));
        assert_eq!(SspOperator::new("0"), Some(SspOperator::Byte(0)));
        assert_eq!(SspOperator::new("42"), Some(SspOperator::Byte(0x42)));
        assert_eq!(SspOperator::new("ff"), Some(SspOperator::Byte(0xff)));
    }

    #[test]
    fn check_words() {
        assert_eq!(SspOperator::new("0x0000"), Some(SspOperator::Word(0)));
        assert_eq!(SspOperator::new("0x0042"), Some(SspOperator::Word(0x42)));
        assert_eq!(SspOperator::new("0xffff"), Some(SspOperator::Word(0xffff)));
        assert_eq!(SspOperator::new("0000h"), Some(SspOperator::Word(0)));
        assert_eq!(SspOperator::new("0042h"), Some(SspOperator::Word(0x42)));
        assert_eq!(SspOperator::new("ffffh"), Some(SspOperator::Word(0xffff)));
        assert_eq!(SspOperator::new("0000"), Some(SspOperator::Word(0)));
        assert_eq!(SspOperator::new("0042"), Some(SspOperator::Word(0x42)));
        assert_eq!(SspOperator::new("ffff"), Some(SspOperator::Word(0xffff)));
    }

    #[test]
    fn invalid_numeric_values() {
        assert_eq!(SspOperator::new("a0x0"), None);
        assert_eq!(SspOperator::new("0x0g"), None);
        assert_eq!(SspOperator::new("0x000g"), None);
        assert_eq!(SspOperator::new("h42"), None);
        assert_eq!(SspOperator::new("h4242"), None);
        assert_eq!(SspOperator::new("42gg"), None);
        assert_eq!(SspOperator::new("0xfffff"), None);
        assert_eq!(SspOperator::new("00000"), None);
    }
}
