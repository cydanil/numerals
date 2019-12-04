/*!
Convert japanese numerals to arabic, and vice-versa.

This module provides two functions to convert to and from japanese numerals.
*/
use std::collections::{BTreeSet, HashMap};

lazy_static! {
    static ref KANJI_TO_ARABIC: HashMap<char, u64> = [
        ('零', 0),
        ('〇', 0),
        ('一', 1),
        ('二', 2),
        ('三', 3),
        ('四', 4),
        ('五', 5),
        ('六', 6),
        ('七', 7),
        ('八', 8),
        ('九', 9),
        ('十', 10),
        ('百', 100),
        ('千', 1000),
        ('万', 10_000),
        ('億', 100_000_000),
        ('兆', 1000_000_000_000),
        ('京', 10_000_000_000_000_000)
    ]
    .iter()
    .cloned()
    .collect();
    static ref NUMERALS: BTreeSet<char> = KANJI_TO_ARABIC.keys().cloned().collect();
    static ref ARABIC_TO_KANJI: HashMap<u64, &'static str> = [
        (10_000_000_000_000_000, "京"),
        (1_000_000_000_000_000, "千兆"),
        (100_000_000_000_000, "百兆"),
        (100_000_000_000_00, "十兆"),
        (100_000_000_000_0, "兆"),
        (100_000_000_000, "千億"),
        (100_000_000_00, "百億"),
        (100_000_000_0, "十億"),
        (100_000_000, "億"),
        (100_000_00, "千万"),
        (100_000_0, "百万"),
        (100_000, "十万"),
        (10_000, "万"),
        (1000, "千"),
        (100, "百"),
        (10, "十"),
        (9, "九"),
        (8, "八"),
        (7, "七"),
        (6, "六"),
        (5, "五"),
        (4, "四"),
        (3, "三"),
        (2, "二"),
        (1, "一"),
        (0, "零"),
    ]
    .iter()
    .cloned()
    .collect();
}

pub fn to_japanese(input: u64) -> String {
    if input == 0 {
        return "零".into();
    }

    let input: Vec<u64> = input
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .rev()
        .collect();

    let mut ret = String::new();
    let mut power: u64 = 1;
    let mut pwr_symbol: String;
    let mut current: String;

    for digit in &input {
        pwr_symbol = ARABIC_TO_KANJI[&power].to_string();
        current = ARABIC_TO_KANJI[digit].to_string();

        if power != 1 {
            current = match digit {
                1 => pwr_symbol,
                _ => format!("{}{}", current, pwr_symbol),
            };
        }
        if *digit != 0 {
            ret = format!("{}{}", current, ret);
        }
        power = power * 10;
    }
    ret
}

#[cfg(test)]
mod test_to_japanese {
    use crate::japanese::to_japanese;

    #[test]
    fn test_valid_inputs() {
        assert_eq!("零", to_japanese(0));
        assert_eq!("一", to_japanese(1));
        assert_eq!("千九百九十四", to_japanese(1994));
        assert_eq!("万千百十一", to_japanese(11111));
        assert_eq!("十万", to_japanese(100000));
        assert_eq!("千兆六十五万", to_japanese(1000000000650000));
        assert_eq!(
            "千八百四十四万六千七百四十四兆七百三十七億九百五十五万千六百十六",
            to_japanese(std::u64::MAX)
        );
    }
}
