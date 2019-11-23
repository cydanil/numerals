#[macro_use]
extern crate lazy_static;
use std::env;

mod roman;
use crate::roman::{to_arabic, to_roman};

fn main() {
    let mut input = String::new();
    let mut use_unicode = false;
    for arg in env::args().skip(1).take(2) {
        match arg.as_ref() {
            "-u" => use_unicode = true,
            "--unicode" => use_unicode = true,
            _ => input = arg,
        };
    }

    if input.is_empty() {
        return;
    }

    let is_arabic: bool = match input.parse::<u64>() {
        Ok(_) => true,
        Err(_) => false,
    };

    let ret: String;
    if is_arabic {
        ret = match to_roman(input.parse::<u64>().unwrap(), use_unicode) {
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
