pub fn is_label(mnemonic: &str) -> bool {
    mnemonic.ends_with(":")
}

pub fn is_label_ref(mnemonic: &str) -> bool {
    mnemonic.starts_with("@")
}

#[cfg(test)]
mod label_tests {
    use super::*;

    #[test]
    fn check_is_label_valid() {
        assert_eq!(is_label("lol:"), true);
        assert_eq!(is_label("lol():"), true);
    }

    #[test]
    fn check_is_label_invalid() {
        assert_eq!(is_label(":lol"), false);
        assert_eq!(is_label("ld"), false);
        assert_eq!(is_label(""), false);
    }

    #[test]
    fn check_is_label_ref_valid() {
        assert_eq!(is_label_ref("@lol"), true);
        assert_eq!(is_label_ref("@lol()"), true);
    }

    #[test]
    fn check_is_label_ref_invalid() {
        assert_eq!(is_label_ref("lol@"), false);
        assert_eq!(is_label_ref("ld@"), false);
        assert_eq!(is_label_ref(""), false);
    }
}
