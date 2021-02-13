#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SspMacro {
    Dw,
    Org,
    Equ,
    Equb,
}

impl SspMacro {
    pub fn new(macr: &str) -> Option<SspMacro> {
        match macr.to_lowercase().as_str() {
            "dw" => Some(SspMacro::Dw),
            "org" => Some(SspMacro::Org),
            "equ" => Some(SspMacro::Equ),
            "equb" => Some(SspMacro::Equb),
            _ => None,
        }
    }
}

#[cfg(test)]
mod macros_tests {
    use super::*;

    #[test]
    fn check_valid_macros() {
        assert_eq!(SspMacro::new("dw"), Some(SspMacro::Dw));
        assert_eq!(SspMacro::new("org"), Some(SspMacro::Org));
        assert_eq!(SspMacro::new("equ"), Some(SspMacro::Equ));
        assert_eq!(SspMacro::new("equb"), Some(SspMacro::Equb));
    }

    #[test]
    fn check_invalid_macros() {
        assert_eq!(SspMacro::new("lol"), None);
        assert_eq!(SspMacro::new(""), None);
    }
}
