pub mod algo {
    pub use crate::error::*;
    pub use crate::gtin::*;
    pub use crate::util::*;

    pub fn compute(val: &str) -> Result<String, error::Error> {
        let char_list: Vec<char> = val.chars().rev().collect();

        let calc_result = char_list
            .iter()
            .enumerate()
            .map(|(index, item)| {
                util::mod10_weight3_double(index, item.to_string().parse::<u32>().unwrap())
            })
            .sum::<u32>();

        let last_digit_calc_result = calc_result.to_string().chars().last().unwrap();

        let correct_last_digit = 10 - util::char_to_int(last_digit_calc_result);

        return Ok(correct_last_digit.to_string());
    }
    #[allow(dead_code)]
    pub fn generate(val: &str) -> Result<String, error::Error> {
        let computed_value = compute(val)?;
        let result = util::build_code(val, computed_value.as_str())?;
        return Ok(result);
    }
    #[allow(dead_code)]
    pub fn validate(val: &str) -> Result<bool, error::Error> {
        let is_gtin_valid = gtin::CodeType::validate(val);

        if !is_gtin_valid {
            return Ok(false);
        }

        let (unprotected, check_chars) = util::split_nth(val, 1)?;

        let correct_last_digit = compute(unprotected)?;

        if check_chars != correct_last_digit {
            return Ok(false);
        }

        return Ok(true);
    }
}
