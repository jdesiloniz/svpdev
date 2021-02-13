#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SspGeneralRegister {
    Dummy,
    X,
    Y,
    A,
    St,
    Stack,
    Pc,
    P,
    Ext0,
    Ext1,
    Ext2,
    Ext3,
    Ext4,
    Ext5,
    Ext6,
    Ext7,
}

impl SspGeneralRegister {
    pub fn new(reg: &str) -> Option<SspGeneralRegister> {
        match reg.to_lowercase().as_str() {
            // Primary registers
            "-" => Some(SspGeneralRegister::Dummy),
            "x" => Some(SspGeneralRegister::X),
            "y" => Some(SspGeneralRegister::Y),
            "a" => Some(SspGeneralRegister::A),
            "st" => Some(SspGeneralRegister::St),
            "stack" => Some(SspGeneralRegister::Stack),
            "pc" => Some(SspGeneralRegister::Pc),
            "p" => Some(SspGeneralRegister::P),
            "ext0" => Some(SspGeneralRegister::Ext0),
            "ext1" => Some(SspGeneralRegister::Ext1),
            "ext2" => Some(SspGeneralRegister::Ext2),
            "ext3" => Some(SspGeneralRegister::Ext3),
            "ext4" => Some(SspGeneralRegister::Ext4),
            "ext5" => Some(SspGeneralRegister::Ext5),
            "ext6" => Some(SspGeneralRegister::Ext6),
            "ext7" => Some(SspGeneralRegister::Ext7),
            _ => None,
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            SspGeneralRegister::Dummy => 0,
            SspGeneralRegister::X => 0x1,
            SspGeneralRegister::Y => 0x2,
            SspGeneralRegister::A => 0x3,
            SspGeneralRegister::St => 0x4,
            SspGeneralRegister::Stack => 0x5,
            SspGeneralRegister::Pc => 0x6,
            SspGeneralRegister::P => 0x7,
            SspGeneralRegister::Ext0 => 0x8,
            SspGeneralRegister::Ext1 => 0x9,
            SspGeneralRegister::Ext2 => 0xA,
            SspGeneralRegister::Ext3 => 0xB,
            SspGeneralRegister::Ext4 => 0xC,
            SspGeneralRegister::Ext5 => 0xD,
            SspGeneralRegister::Ext6 => 0xE,
            SspGeneralRegister::Ext7 => 0xF,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SspModifier {
    Zero,
    PostIncrement,
    PostIncrementModulo,
    PostDecrementModulo,
    Bank0,
    Bank1,
    Bank2,
    Bank3,
}

impl SspModifier {
    pub fn value(&self) -> u8 {
        match self {
            SspModifier::Zero => 0,
            SspModifier::PostIncrement => 3,
            SspModifier::PostIncrementModulo => 1,
            SspModifier::PostDecrementModulo => 2,
            SspModifier::Bank0 => 0,
            SspModifier::Bank1 => 1,
            SspModifier::Bank2 => 2,
            SspModifier::Bank3 => 3,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SspPointerRegister {
    R0(SspModifier),
    R1(SspModifier),
    R2(SspModifier),
    R3(SspModifier),
    R4(SspModifier),
    R5(SspModifier),
    R6(SspModifier),
    R7(SspModifier),
}

impl SspPointerRegister {
    pub fn new(preg: &str) -> Option<SspPointerRegister> {
        match preg.to_lowercase().as_str() {
            // Direct access without iteration
            "r0" => Some(SspPointerRegister::R0(SspModifier::Zero)),
            "r1" => Some(SspPointerRegister::R1(SspModifier::Zero)),
            "r2" => Some(SspPointerRegister::R2(SspModifier::Zero)),
            "r3" => Some(SspPointerRegister::R3(SspModifier::Zero)),
            "r4" => Some(SspPointerRegister::R4(SspModifier::Zero)),
            "r5" => Some(SspPointerRegister::R5(SspModifier::Zero)),
            "r6" => Some(SspPointerRegister::R6(SspModifier::Zero)),
            "r7" => Some(SspPointerRegister::R7(SspModifier::Zero)),

            // Direct access with iteration
            "r0+" => Some(SspPointerRegister::R0(SspModifier::PostIncrementModulo)),
            "r1+" => Some(SspPointerRegister::R1(SspModifier::PostIncrementModulo)),
            "r2+" => Some(SspPointerRegister::R2(SspModifier::PostIncrementModulo)),
            "r4+" => Some(SspPointerRegister::R4(SspModifier::PostIncrementModulo)),
            "r5+" => Some(SspPointerRegister::R5(SspModifier::PostIncrementModulo)),
            "r6+" => Some(SspPointerRegister::R6(SspModifier::PostIncrementModulo)),
            "r0-" => Some(SspPointerRegister::R0(SspModifier::PostDecrementModulo)),
            "r1-" => Some(SspPointerRegister::R1(SspModifier::PostDecrementModulo)),
            "r2-" => Some(SspPointerRegister::R2(SspModifier::PostDecrementModulo)),
            "r4-" => Some(SspPointerRegister::R4(SspModifier::PostDecrementModulo)),
            "r5-" => Some(SspPointerRegister::R5(SspModifier::PostDecrementModulo)),
            "r6-" => Some(SspPointerRegister::R6(SspModifier::PostDecrementModulo)),
            "r0+!" => Some(SspPointerRegister::R0(SspModifier::PostIncrement)),
            "r1+!" => Some(SspPointerRegister::R1(SspModifier::PostIncrement)),
            "r2+!" => Some(SspPointerRegister::R2(SspModifier::PostIncrement)),
            "r4+!" => Some(SspPointerRegister::R4(SspModifier::PostIncrement)),
            "r5+!" => Some(SspPointerRegister::R5(SspModifier::PostIncrement)),
            "r6+!" => Some(SspPointerRegister::R6(SspModifier::PostIncrement)),

            // RAM Bank accesses
            "r3|00" => Some(SspPointerRegister::R3(SspModifier::Bank0)),
            "r3|01" => Some(SspPointerRegister::R3(SspModifier::Bank1)),
            "r3|10" => Some(SspPointerRegister::R3(SspModifier::Bank2)),
            "r3|11" => Some(SspPointerRegister::R3(SspModifier::Bank3)),
            "r7|00" => Some(SspPointerRegister::R7(SspModifier::Bank0)),
            "r7|01" => Some(SspPointerRegister::R7(SspModifier::Bank1)),
            "r7|10" => Some(SspPointerRegister::R7(SspModifier::Bank2)),
            "r7|11" => Some(SspPointerRegister::R7(SspModifier::Bank3)),

            _ => None,
        }
    }

    pub fn ram_bank(&self) -> u8 {
        match self {
            SspPointerRegister::R0(_) => 0,
            SspPointerRegister::R1(_) => 0,
            SspPointerRegister::R2(_) => 0,
            SspPointerRegister::R3(_) => 0,
            _ => 1,
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            SspPointerRegister::R0(_) => 0,
            SspPointerRegister::R1(_) => 1,
            SspPointerRegister::R2(_) => 2,
            SspPointerRegister::R3(_) => 3,
            SspPointerRegister::R4(_) => 0,
            SspPointerRegister::R5(_) => 1,
            SspPointerRegister::R6(_) => 2,
            SspPointerRegister::R7(_) => 3,
        }
    }

    pub fn modifier_value(&self) -> u8 {
        match self {
            SspPointerRegister::R0(modifier) => modifier.value(),
            SspPointerRegister::R1(modifier) => modifier.value(),
            SspPointerRegister::R2(modifier) => modifier.value(),
            SspPointerRegister::R3(modifier) => modifier.value(),
            SspPointerRegister::R4(modifier) => modifier.value(),
            SspPointerRegister::R5(modifier) => modifier.value(),
            SspPointerRegister::R6(modifier) => modifier.value(),
            SspPointerRegister::R7(modifier) => modifier.value(),
        }
    }
}

#[cfg(test)]
mod pointer_registers_tests {
    use super::*;

    #[test]
    fn check_valid_general_registers() {
        assert_eq!(
            SspGeneralRegister::new("-"),
            Some(SspGeneralRegister::Dummy)
        );
        assert_eq!(SspGeneralRegister::new("x"), Some(SspGeneralRegister::X));
        assert_eq!(SspGeneralRegister::new("y"), Some(SspGeneralRegister::Y));
        assert_eq!(SspGeneralRegister::new("a"), Some(SspGeneralRegister::A));
        assert_eq!(SspGeneralRegister::new("st"), Some(SspGeneralRegister::St));
        assert_eq!(SspGeneralRegister::new("pc"), Some(SspGeneralRegister::Pc));
        assert_eq!(SspGeneralRegister::new("p"), Some(SspGeneralRegister::P));
        assert_eq!(
            SspGeneralRegister::new("ext0"),
            Some(SspGeneralRegister::Ext0)
        );
        assert_eq!(
            SspGeneralRegister::new("ext1"),
            Some(SspGeneralRegister::Ext1)
        );
        assert_eq!(
            SspGeneralRegister::new("ext2"),
            Some(SspGeneralRegister::Ext2)
        );
        assert_eq!(
            SspGeneralRegister::new("ext3"),
            Some(SspGeneralRegister::Ext3)
        );
        assert_eq!(
            SspGeneralRegister::new("ext4"),
            Some(SspGeneralRegister::Ext4)
        );
        assert_eq!(
            SspGeneralRegister::new("ext5"),
            Some(SspGeneralRegister::Ext5)
        );
        assert_eq!(
            SspGeneralRegister::new("ext6"),
            Some(SspGeneralRegister::Ext6)
        );
        assert_eq!(
            SspGeneralRegister::new("ext7"),
            Some(SspGeneralRegister::Ext7)
        );
    }

    #[test]
    fn check_valid_pointer_registers() {
        assert_eq!(
            SspPointerRegister::new("r0"),
            Some(SspPointerRegister::R0(SspModifier::Zero))
        );
        assert_eq!(
            SspPointerRegister::new("r1"),
            Some(SspPointerRegister::R1(SspModifier::Zero))
        );
        assert_eq!(
            SspPointerRegister::new("r2"),
            Some(SspPointerRegister::R2(SspModifier::Zero))
        );
        assert_eq!(
            SspPointerRegister::new("r3"),
            Some(SspPointerRegister::R3(SspModifier::Zero))
        );
        assert_eq!(
            SspPointerRegister::new("r4"),
            Some(SspPointerRegister::R4(SspModifier::Zero))
        );
        assert_eq!(
            SspPointerRegister::new("r5"),
            Some(SspPointerRegister::R5(SspModifier::Zero))
        );
        assert_eq!(
            SspPointerRegister::new("r6"),
            Some(SspPointerRegister::R6(SspModifier::Zero))
        );
        assert_eq!(
            SspPointerRegister::new("r7"),
            Some(SspPointerRegister::R7(SspModifier::Zero))
        );
        assert_eq!(
            SspPointerRegister::new("r0+"),
            Some(SspPointerRegister::R0(SspModifier::PostIncrementModulo))
        );
        assert_eq!(
            SspPointerRegister::new("r1+"),
            Some(SspPointerRegister::R1(SspModifier::PostIncrementModulo))
        );
        assert_eq!(
            SspPointerRegister::new("r2+"),
            Some(SspPointerRegister::R2(SspModifier::PostIncrementModulo))
        );
        assert_eq!(
            SspPointerRegister::new("r4+"),
            Some(SspPointerRegister::R4(SspModifier::PostIncrementModulo))
        );
        assert_eq!(
            SspPointerRegister::new("r5+"),
            Some(SspPointerRegister::R5(SspModifier::PostIncrementModulo))
        );
        assert_eq!(
            SspPointerRegister::new("r6+"),
            Some(SspPointerRegister::R6(SspModifier::PostIncrementModulo))
        );
        assert_eq!(
            SspPointerRegister::new("r0-"),
            Some(SspPointerRegister::R0(SspModifier::PostDecrementModulo))
        );
        assert_eq!(
            SspPointerRegister::new("r1-"),
            Some(SspPointerRegister::R1(SspModifier::PostDecrementModulo))
        );
        assert_eq!(
            SspPointerRegister::new("r2-"),
            Some(SspPointerRegister::R2(SspModifier::PostDecrementModulo))
        );
        assert_eq!(
            SspPointerRegister::new("r4-"),
            Some(SspPointerRegister::R4(SspModifier::PostDecrementModulo))
        );
        assert_eq!(
            SspPointerRegister::new("r5-"),
            Some(SspPointerRegister::R5(SspModifier::PostDecrementModulo))
        );
        assert_eq!(
            SspPointerRegister::new("r6-"),
            Some(SspPointerRegister::R6(SspModifier::PostDecrementModulo))
        );
        assert_eq!(
            SspPointerRegister::new("r0+!"),
            Some(SspPointerRegister::R0(SspModifier::PostIncrement))
        );
        assert_eq!(
            SspPointerRegister::new("r1+!"),
            Some(SspPointerRegister::R1(SspModifier::PostIncrement))
        );
        assert_eq!(
            SspPointerRegister::new("r2+!"),
            Some(SspPointerRegister::R2(SspModifier::PostIncrement))
        );
        assert_eq!(
            SspPointerRegister::new("r4+!"),
            Some(SspPointerRegister::R4(SspModifier::PostIncrement))
        );
        assert_eq!(
            SspPointerRegister::new("r5+!"),
            Some(SspPointerRegister::R5(SspModifier::PostIncrement))
        );
        assert_eq!(
            SspPointerRegister::new("r6+!"),
            Some(SspPointerRegister::R6(SspModifier::PostIncrement))
        );
        assert_eq!(
            SspPointerRegister::new("r3|00"),
            Some(SspPointerRegister::R3(SspModifier::Bank0))
        );
        assert_eq!(
            SspPointerRegister::new("r3|01"),
            Some(SspPointerRegister::R3(SspModifier::Bank1))
        );
        assert_eq!(
            SspPointerRegister::new("r3|10"),
            Some(SspPointerRegister::R3(SspModifier::Bank2))
        );
        assert_eq!(
            SspPointerRegister::new("r3|11"),
            Some(SspPointerRegister::R3(SspModifier::Bank3))
        );
        assert_eq!(
            SspPointerRegister::new("r7|00"),
            Some(SspPointerRegister::R7(SspModifier::Bank0))
        );
        assert_eq!(
            SspPointerRegister::new("r7|01"),
            Some(SspPointerRegister::R7(SspModifier::Bank1))
        );
        assert_eq!(
            SspPointerRegister::new("r7|10"),
            Some(SspPointerRegister::R7(SspModifier::Bank2))
        );
        assert_eq!(
            SspPointerRegister::new("r7|11"),
            Some(SspPointerRegister::R7(SspModifier::Bank3))
        );
    }
}
