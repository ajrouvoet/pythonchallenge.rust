use data::challenge::{Challenge};

use num::{BigUint};
use num::pow::pow;

pub fn challenge_01() -> Challenge {
    // calculate the asked for big number
    let i = pow(BigUint::from(2u32), 38);

    Challenge {
        num: 1,
        url: String::from("http://www.pythonchallenge.com/pc/def/0.html"),
        hints: Vec::new(),
        solution_url: format!("http://www.pythonchallenge.com/pc/def/{}.html", i)
    }
}

