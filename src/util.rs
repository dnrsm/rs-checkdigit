pub mod util {
    pub use crate::error::*;

    pub fn char_to_int(val: char) -> u32 {
        return val.to_digit(10).unwrap();
    }

    fn is_even(n: usize) -> bool {
        n % 2 == 0
    }

    pub fn mod10_weight3_double(index: usize, n: u32) -> u32 {
        n * (if is_even(index) { 3 } else { 1 })
    }

    pub fn split_nth(val: &str, n: usize) -> Result<(&str, &str), error::Error> {
        if let Some((mid, _)) = val.char_indices().nth_back(n - 1) {
            Ok(val.split_at(mid))
        } else {
            Err(error::Error::InvalidString(String::from(val)))
        }
    }

    pub fn build_code(val: &str, check_chars: &str) -> Result<String, error::Error> {
        let mut protected = String::with_capacity(val.len() + check_chars.len());
        protected.push_str(val);
        protected.push_str(check_chars);
        Ok(protected)
    }

    #[cfg(test)]
    mod tests {
        #[allow(unused_imports)]
        use super::*;
        #[test]
        fn is_even_succeed() {
            assert!(is_even(2));
            assert!(is_even(4));
            assert!(!is_even(5));
        }
    }
}
