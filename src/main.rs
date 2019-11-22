#[macro_use]
extern crate lazy_static;
use std::env;

mod roman;
use crate::roman::{to_arabic, to_roman};

fn main() {
    let mut iter = env::args().skip(1).take(1);
    let input: String;
    match iter.next() {
        Some(val) => input = val,
        None => return,
    };

    let is_arabic: bool = match input.parse::<u64>() {
        Ok(_) => true,
        Err(_) => false,
    };

    let ret: String;
    if is_arabic {
        ret = match to_roman(input.parse::<u64>().unwrap()) {
            Ok(val) => val,
            Err(e) => e.to_string(),
        };
    } else {
        ret = match to_arabic(input) {
            Ok(val) => val.to_string(),
            Err(e) => e.to_string(),
        };
    }
    println!("{}", ret);
}
