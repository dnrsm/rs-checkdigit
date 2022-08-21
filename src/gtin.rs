pub mod gtin {
    #[derive(Debug, PartialEq)]
    pub enum CodeType {
        Gtin13,
        Gtin8,
        Gtin14,
        UPC,
        SSCC,
    }
    impl CodeType {
        #[allow(dead_code)]
        fn check(val: &str) -> Option<Self> {
            match val.len() {
                13 => Some(Self::Gtin13),
                8 => Some(Self::Gtin8),
                14 => Some(Self::Gtin14),
                12 => Some(Self::UPC),
                18 => Some(Self::SSCC),
                _ => None,
            }
        }
        #[allow(dead_code)]
        pub fn validate(val: &str) -> bool {
            let code_type = Self::check(val);
            if code_type.is_none() {
                return false;
            }
            true
        }
    }
    mod tests {
        #[allow(unused_imports)]
        use super::*;
        #[test]
        fn test_check_gtin13() {
            let check = CodeType::check("1234567890123");
            assert!(check.is_some());
            assert_eq!(check.unwrap(), CodeType::Gtin13);
        }
        #[test]
        fn test_check_gtin8() {
            let check = CodeType::check("12345678");
            assert!(check.is_some());
            assert_eq!(check.unwrap(), CodeType::Gtin8);
        }
        #[test]
        fn test_check_gtin14() {
            let check = CodeType::check("12345678901234");
            assert!(check.is_some());
            assert_eq!(check.unwrap(), CodeType::Gtin14);
        }
        #[test]
        fn test_check_upc() {
            let check = CodeType::check("123456789012");
            assert!(check.is_some());
            assert_eq!(check.unwrap(), CodeType::UPC);
        }
        #[test]
        fn test_check_sscc() {
            let check = CodeType::check("123456789012345678");
            assert!(check.is_some());
            assert_eq!(check.unwrap(), CodeType::SSCC);
        }
        #[test]
        fn test_check_empty() {
            let check = CodeType::check("");
            assert!(check.is_none());
            assert_eq!(check, None);
        }
        #[test]
        fn test_validate() {
            let check = CodeType::validate("12345678");
            assert!(check);
        }
        #[test]
        fn test_validate_err() {
            let check = CodeType::validate("123456789");
            assert!(!check);
        }
    }
}
