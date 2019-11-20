// Convert roman numerals to arabic.
//
// There are a few rules to observe in checking the validity of a roman number:
// - Having two subtraction in a row is illegal:
//      IXC does not equal 91 (C - (X - I))
// - Having four similar numerals in a rwo is illegal:
//      400 should be written as CD (D - C), rather than CCCC
// - But IIII is fine:
//      This is typically used by watchmakers to make the reading of the number
//      4 easy to read upside down.
// - Only I, X, C, and M are allowed to be represented several times in a row:
//      LL should be C; DD should be M
//
// The input is expected to be ASCII, although there exist unicode characters
// for roman numerals. Apostrophus and Vinculum are not supported.

#[macro_use]
extern crate lazy_static;

use std::collections::{BTreeSet, HashMap, VecDeque};
use std::env;
use std::error::Error;

lazy_static! {
    static ref ROMAN_TO_ARABIC: HashMap<char, u64> = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000)
    ]
    .iter()
    .cloned()
    .collect();
    static ref NUMERALS: BTreeSet<char> = ROMAN_TO_ARABIC.keys().cloned().collect();
}

fn convert(roman: String) -> Result<u64, Box<dyn Error>> {
    let roman = roman.to_ascii_uppercase();
    if roman.is_empty() {
        return Err("Invalid empty string".into());
    }

    if roman == "IIII" {
        // Having four additions in a row is illegal, short of the sequence IIII
        return Ok(4u64);
    }

    let mut characters = BTreeSet::new();
    for c in roman.chars() {
        characters.insert(c);
    }

    if !characters.is_subset(&NUMERALS) {
        return Err("Input contains invalid characters".into());
    }

    // This buffer is used to check that not 4 elements in a row are similar.
    let mut buffer: VecDeque<u64> = VecDeque::with_capacity(4);
    buffer.push_back(0); // oldest
    buffer.push_back(0); // preprevious
    buffer.push_back(0); // previous
    buffer.push_back(0); // current

    let mut value: u64 = 0;
    for c in roman.chars().rev() {
        let current: u64 = ROMAN_TO_ARABIC[&c];
        buffer.pop_front();
        buffer.push_back(current);

        if current < buffer[1] {
            // Having two subtraction in a row is illegal
            return Err("Invalid sequence".into());
        } else if buffer.iter().all(|&item| item == current) {
            return Err("Invalid sequence".into());
        } else if current == buffer[2] && (current == 50 || current == 500) {
            return Err("Invalid sequence".into());
        } else if current < buffer[2] {
            value -= current;
        } else {
            value += current;
        }
    }
    Ok(value)
}

fn main() {
    let mut iter = env::args().skip(1).take(1);
    let input: String;
    match iter.next() {
        Some(val) => input = val,
        None => return,
    };

    match convert(input) {
        Ok(val) => println!("{}", val),
        Err(e) => println!("{}", e),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_cases() {
        let x = convert("iv".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 4);

        let x = convert("LIX".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 59);

        let x = convert("CvL".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 145);
    }

    #[test]
    fn test_empty_input() {
        let x = convert(String::new());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid empty string\")");
    }

    #[test]
    fn test_invalid_characters() {
        let x = convert("LXS".to_string());
        assert!(x.is_err());
        assert_eq!(
            format!("{:?}", x),
            "Err(\"Input contains invalid characters\")"
        );
    }

    #[test]
    fn test_invalid_input_range() {
        let x = convert("XIL".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");

        let x = convert("VIL".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");

        let x = convert("IXC".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");

        let x = convert("XXC".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");
    }

    #[test]
    fn test_valid_input_range() {
        let x = convert("XCIX".to_string());
        assert_eq!(x.unwrap(), 99);

        let x = convert("MCMLXXXIV".to_string());
        assert_eq!(x.unwrap(), 1984);

        let x = convert("MMMCMXCIX".to_string());
        assert_eq!(x.unwrap(), 3999);

        let x = convert("LXXX".to_string());
        assert_eq!(x.unwrap(), 80);
    }

    #[test]
    fn test_four_same_symbols() {
        let x = convert("IIII".to_string());
        assert_eq!(x.unwrap(), 4);

        let x = convert("XXXX".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");

        let x = convert("VIIII".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");
    }

    #[test]
    fn test_double_symbols() {
        let x = convert("MM".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 2000);

        let x = convert("CC".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 200);

        let x = convert("XX".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 20);

        let x = convert("II".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 2);

        let x = convert("LL".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");

        let x = convert("DD".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");
    }
}
