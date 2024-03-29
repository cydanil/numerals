/*!
Convert roman numerals to arabic, and vice-versa.

This module provides two functions to convert to and from roman numerals.
There are a few rules to observe in checking the validity of a roman number:
    - Having two subtraction in a row is illegal:
          IXC does not equal 91 (C - (X - I))
    - Having four similar numerals in a row is illegal:
        400 should be written as CD (D - C), rather than CCCC
    - But IIII is fine:
        This is typically used by watchmakers to make the reading of the number
        4 easy to read upside down.
    - Only I, X, C, and M are allowed to be represented several times in a row:
        LL should be C; DD should be M
    - If a certain sequence can be represented with another symbol, it is illegal:
        LC should be L;

Although unicode caracters exist, Apostrophus and Vinculum are not fully supported.
*/

use std::collections::{BTreeSet, HashMap, VecDeque};
use std::error::Error;

lazy_static! {
    static ref ROMAN_TO_ARABIC: HashMap<char, u64> = [
        ('I', 1),  // ascii
        ('Ⅰ', 1),  // unicode
        ('Ⅱ', 2),
        ('Ⅲ', 3),
        ('Ⅳ', 4),
        ('V', 5),  // ascii
        ('Ⅴ', 5),  // unicode
        ('Ⅵ', 6),
        ('ↅ', 6),
        ('Ⅶ', 7),
        ('Ⅷ', 8),
        ('Ⅸ', 9),
        ('X', 10),  // ascii
        ('Ⅹ', 10),  // unicode
        ('Ⅺ', 11),
        ('L', 50),  // ascii
        ('Ⅼ', 50),  // unicode
        ('ↆ', 50),
        ('C', 100),  // ascii
        ('Ⅽ', 100),  // unicode
        ('D', 500),  // ascii
        ('Ⅾ', 500),  // unicode
        ('M', 1000),  // ascii
        ('Ⅿ', 1000),  // unicode
        ('ↀ', 1000),
        ('ↁ', 5000),
        ('ↂ', 10000),
        ('ↇ', 50000),
        ('ↈ', 100000),
    ]
    .iter()
    .cloned()
    .collect();
    static ref NUMERALS: BTreeSet<char> = ROMAN_TO_ARABIC.keys().cloned().collect();
    static ref ARABIC_TO_ASCII: Vec<(u64, &'static str)> = vec![
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    static ref ARABIC_TO_UNICODE: Vec<(u64, &'static str)> = vec![
        (1000, "Ⅿ"),
        (900, "ⅭⅯ"),
        (500, "Ⅾ"),
        (400, "ⅭⅮ"),
        (100, "Ⅽ"),
        (90, "ⅩⅭ"),
        (50, "Ⅼ"),
        (40, "ⅩⅬ"),
        (10, "Ⅹ"),
        (9, "ⅠⅩ"),
        (5, "V"),
        (4, "ⅠV"),
        (1, "Ⅰ"),
    ];
}

pub fn to_roman(input: u64, use_unicode: bool) -> Result<String, Box<dyn Error>> {
    let mut input = input;
    if input < 1 || input > 3999 {
        return Err(format!(
            "The value should be between 1 and 3999 inclusive, not {}",
            input
        )
        .into());
    }

    let list = match use_unicode {
        true => ARABIC_TO_UNICODE.to_vec(),
        false => ARABIC_TO_ASCII.to_vec(),
    };
    let mut ret = String::new();
    for (arabic, roman) in list.iter() {
        while input % arabic < input {
            ret += roman;
            input -= arabic;
        }
    }
    Ok(ret)
}

pub fn to_arabic(roman: String) -> Result<u64, Box<dyn Error>> {
    let roman = roman.to_uppercase();
    if roman.is_empty() {
        return Err("Invalid empty string".into());
    }

    if roman == "IIII" || roman == "ⅠⅠⅠⅠ" {
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
            // Having four additions in a row is illegal
            return Err("Invalid sequence".into());
        } else if current == buffer[2] && (current == 50 || current == 500) {
            // Having two consecutive L or D is illegal
            return Err("Invalid sequence".into());
        } else if current < buffer[2] {
            if buffer[2] - current == current {
                // Having a subtraction that does nothing is illegal
                return Err("Invalid sequence".into());
            }
            value -= current;
        } else {
            value += current;
        }
    }
    Ok(value)
}

#[cfg(test)]
mod test_to_roman {
    use crate::roman::to_roman;

    #[test]
    fn test_invalid_inputs() {
        let x = to_roman(0u64, false);
        assert!(x.is_err());

        let x = to_roman(1u64, true);
        assert!(x.is_ok());

        let x = to_roman(3999u64, false);
        assert!(x.is_ok());

        let x = to_roman(4000u64, true);
        assert!(x.is_err());
    }

    #[test]
    fn test_valid_inputs() {
        let x = to_roman(1999u64, false);
        assert_eq!(x.unwrap(), "MCMXCIX".to_string());

        let x = to_roman(99u64, false);
        assert_eq!(x.unwrap(), "XCIX".to_string());

        let x = to_roman(1999, true);
        assert_eq!(x.unwrap(), "ⅯⅭⅯⅩⅭⅠⅩ");
    }
}

#[cfg(test)]
mod test_to_arabic {
    use crate::roman::to_arabic;

    #[test]
    fn test_string_cases() {
        let x = to_arabic("iv".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 4);

        let x = to_arabic("LIX".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 59);

        let x = to_arabic("CvL".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 145);
    }
    #[test]
    fn test_unicode_cases() {
        let x = to_arabic("ⅳ".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 4);

        let x = to_arabic("ⅬⅨ".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 59);

        let x = to_arabic("ⅭⅴⅬ".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 145);
    }

    #[test]
    fn test_empty_input() {
        let x = to_arabic(String::new());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid empty string\")");
    }

    #[test]
    fn test_invalid_characters() {
        let x = to_arabic("LXS".to_string());
        assert!(x.is_err());
        assert_eq!(
            format!("{:?}", x),
            "Err(\"Input contains invalid characters\")"
        );
    }

    #[test]
    fn test_invalid_inputs() {
        let x = to_arabic("XIL".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");

        let x = to_arabic("VIL".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");

        let x = to_arabic("IXC".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");

        let x = to_arabic("XXC".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");

        let x = to_arabic("LC".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");

        let x = to_arabic("LDVX".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");
    }

    #[test]
    fn test_valid_inputs() {
        let x = to_arabic("XCIX".to_string());
        assert_eq!(x.unwrap(), 99);

        let x = to_arabic("MCMLXXXIV".to_string());
        assert_eq!(x.unwrap(), 1984);

        let x = to_arabic("MMMCMXCIX".to_string());
        assert_eq!(x.unwrap(), 3999);

        let x = to_arabic("LXXX".to_string());
        assert_eq!(x.unwrap(), 80);
    }

    #[test]
    fn test_four_same_symbols() {
        let x = to_arabic("IIII".to_string());
        assert_eq!(x.unwrap(), 4);

        let x = to_arabic("XXXX".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");

        let x = to_arabic("VIIII".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");
    }

    #[test]
    fn test_double_symbols() {
        let x = to_arabic("MM".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 2000);

        let x = to_arabic("CC".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 200);

        let x = to_arabic("XX".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 20);

        let x = to_arabic("II".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 2);

        let x = to_arabic("LL".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");

        let x = to_arabic("DD".to_string());
        assert!(x.is_err());
        assert_eq!(format!("{:?}", x), "Err(\"Invalid sequence\")");
    }

    #[test]
    fn test_apostrohpus() {
        let x = to_arabic("ↀ".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 1000);

        let x = to_arabic("ↀXↀIX".to_string());
        assert!(x.is_ok());
        assert_eq!(x.unwrap(), 1999);

        let x = to_arabic("ↈIXC".to_string());
        assert!(x.is_err());

        let x = to_arabic("ↈⅠV".to_string());
        assert_eq!(x.unwrap(), 100004);
    }
}
