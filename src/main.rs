mod checkdigit;
mod error;
mod gtin;
mod util;

pub use crate::checkdigit::algo::*;
pub use crate::error::*;
pub use crate::gtin::*;
pub use crate::util::*;

fn main() {}

#[test]
fn test_succeed() {
    let gtin13 = validate("4569951116179").unwrap_or(false);
    let gtin8 = validate("0000049968712").unwrap_or(false);
    let gtin14 = validate("14569951116176").unwrap_or(false);
    let upc = validate("012345678905").unwrap_or(false);
    let sscc = validate("045699511100000016").unwrap_or(false);
    assert!(gtin13);
    assert!(gtin8);
    assert!(gtin14);
    assert!(upc);
    assert!(sscc);

    let gen_gtin13 = generate("456995111617");
    assert_eq!(gen_gtin13.unwrap_or("error".to_string()), "4569951116179");

    let compute_gtin13 = compute("456995111617");
    assert_eq!(compute_gtin13.unwrap_or("error".to_string()), "9");
}
#[test]
fn test_fail() {
    let gtin13 = validate("4569951116178").unwrap_or_default();
    let gtin8 = validate("0000049968711").unwrap_or_default();
    let gtin14 = validate("14569951116175").unwrap_or_default();
    let upc = validate("012345678904").unwrap_or_default();
    let sscc = validate("045699511100000015").unwrap_or_default();
    assert!(!gtin13);
    assert!(!gtin8);
    assert!(!gtin14);
    assert!(!upc);
    assert!(!sscc);
}
