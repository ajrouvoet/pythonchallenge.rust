use std::collections::HashMap;
use regex::{Regex};

use data::challenge::{Challenge};

pub fn challenge_04() -> Challenge {
    let garbage = include_str!("../resources/equality");

    let re = Regex
        ::new(r"[^A-Z][A-Z]{3}(?P<c>[a-z])[A-Z]{3}[^A-Z]")
        .expect("invalid regex!");

    let hint = re
        .captures_iter(garbage)
        .map(|cap| {
             cap.get(1).map_or("", |m| m.as_str())
        })
        .collect();

    Challenge {
        num: 4,
        url: String::from("http://www.pythonchallenge.com/pc/def/equality.html"),
        hints: vec![hint],
        solution_url: String::from(""),
    }
}
