#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SspMnemonicModifier {
    Immediate,
    Reference,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SspMnemonic {
    Sub(SspMnemonicModifier),
    Cmp(SspMnemonicModifier),
    Add(SspMnemonicModifier),
    And(SspMnemonicModifier),
    Or(SspMnemonicModifier),
    Eor(SspMnemonicModifier),
    Ld(SspMnemonicModifier),
    Bra,
    Ret,
    Call,
    Mld,
    Mpya,
    Mpys,
    Mod,
}

impl SspMnemonic {
    pub fn new(mnemonic: &str) -> Option<SspMnemonic> {
        match mnemonic.to_lowercase().as_str() {
            "ld" => Some(SspMnemonic::Ld(SspMnemonicModifier::Reference)),
            "ldi" => Some(SspMnemonic::Ld(SspMnemonicModifier::Immediate)),
            "sub" => Some(SspMnemonic::Sub(SspMnemonicModifier::Reference)),
            "subi" => Some(SspMnemonic::Sub(SspMnemonicModifier::Immediate)),
            "cmp" => Some(SspMnemonic::Cmp(SspMnemonicModifier::Reference)),
            "cmpi" => Some(SspMnemonic::Cmp(SspMnemonicModifier::Immediate)),
            "add" => Some(SspMnemonic::Add(SspMnemonicModifier::Reference)),
            "addi" => Some(SspMnemonic::Add(SspMnemonicModifier::Immediate)),
            "and" => Some(SspMnemonic::And(SspMnemonicModifier::Reference)),
            "andi" => Some(SspMnemonic::And(SspMnemonicModifier::Immediate)),
            "or" => Some(SspMnemonic::Or(SspMnemonicModifier::Reference)),
            "ori" => Some(SspMnemonic::Or(SspMnemonicModifier::Immediate)),
            "eor" => Some(SspMnemonic::Eor(SspMnemonicModifier::Reference)),
            "eori" => Some(SspMnemonic::Eor(SspMnemonicModifier::Immediate)),
            "bra" => Some(SspMnemonic::Bra),
            "ret" => Some(SspMnemonic::Ret),
            "call" => Some(SspMnemonic::Call),
            "mld" => Some(SspMnemonic::Mld),
            "mpya" => Some(SspMnemonic::Mpya),
            "mpys" => Some(SspMnemonic::Mpys),
            "mod" => Some(SspMnemonic::Mod),
            _ => None,
        }
    }

    pub fn is_immediate(mnemonic: SspMnemonic) -> bool {
        match mnemonic {
            SspMnemonic::Sub(SspMnemonicModifier::Immediate)    =>   true,
            SspMnemonic::Cmp(SspMnemonicModifier::Immediate)    =>   true,
            SspMnemonic::Add(SspMnemonicModifier::Immediate)    =>   true,
            SspMnemonic::And(SspMnemonicModifier::Immediate)    =>   true,
            SspMnemonic::Or(SspMnemonicModifier::Immediate)     =>   true,
            SspMnemonic::Eor(SspMnemonicModifier::Immediate)    =>   true,
            SspMnemonic::Ld(SspMnemonicModifier::Immediate)     =>   true,
            _                                                   =>   false,
        }

    }
}

#[cfg(test)]
mod mnemonic_tests {
    use super::*;

    #[test]
    fn check_valid_mnemonics() {
        assert_eq!(
            SspMnemonic::new("ld"),
            Some(SspMnemonic::Ld(SspMnemonicModifier::Reference))
        );
        assert_eq!(
            SspMnemonic::new("ldi"),
            Some(SspMnemonic::Ld(SspMnemonicModifier::Immediate))
        );
        assert_eq!(
            SspMnemonic::new("sub"),
            Some(SspMnemonic::Sub(SspMnemonicModifier::Reference))
        );
        assert_eq!(
            SspMnemonic::new("subi"),
            Some(SspMnemonic::Sub(SspMnemonicModifier::Immediate))
        );
        assert_eq!(
            SspMnemonic::new("cmp"),
            Some(SspMnemonic::Cmp(SspMnemonicModifier::Reference))
        );
        assert_eq!(
            SspMnemonic::new("cmpi"),
            Some(SspMnemonic::Cmp(SspMnemonicModifier::Immediate))
        );
        assert_eq!(
            SspMnemonic::new("add"),
            Some(SspMnemonic::Add(SspMnemonicModifier::Reference))
        );
        assert_eq!(
            SspMnemonic::new("addi"),
            Some(SspMnemonic::Add(SspMnemonicModifier::Immediate))
        );
        assert_eq!(
            SspMnemonic::new("and"),
            Some(SspMnemonic::And(SspMnemonicModifier::Reference))
        );
        assert_eq!(
            SspMnemonic::new("andi"),
            Some(SspMnemonic::And(SspMnemonicModifier::Immediate))
        );
        assert_eq!(
            SspMnemonic::new("or"),
            Some(SspMnemonic::Or(SspMnemonicModifier::Reference))
        );
        assert_eq!(
            SspMnemonic::new("ori"),
            Some(SspMnemonic::Or(SspMnemonicModifier::Immediate))
        );
        assert_eq!(
            SspMnemonic::new("eor"),
            Some(SspMnemonic::Eor(SspMnemonicModifier::Reference))
        );
        assert_eq!(
            SspMnemonic::new("eori"),
            Some(SspMnemonic::Eor(SspMnemonicModifier::Immediate))
        );
        assert_eq!(SspMnemonic::new("bra"), Some(SspMnemonic::Bra));
        assert_eq!(SspMnemonic::new("ret"), Some(SspMnemonic::Ret));
        assert_eq!(SspMnemonic::new("call"), Some(SspMnemonic::Call));
        assert_eq!(SspMnemonic::new("mld"), Some(SspMnemonic::Mld));
        assert_eq!(SspMnemonic::new("mpya"), Some(SspMnemonic::Mpya));
        assert_eq!(SspMnemonic::new("mpys"), Some(SspMnemonic::Mpys));
        assert_eq!(SspMnemonic::new("mod"), Some(SspMnemonic::Mod));
    }

    #[test]
    fn check_invalid_mnemonics() {
        assert_eq!(SspMnemonic::new("lol"), None);
        assert_eq!(SspMnemonic::new(""), None);
    }
}
