extern crate num;

mod data;

use num::{BigUint};
use num::pow::pow;
use data::challenge::{Challenge};

fn challenge_01() -> Challenge {
    // calculate the asked for big number
    let i = pow(BigUint::from(2u32), 38);

    Challenge {
        num: 1,
        url: String::from("http://www.pythonchallenge.com/pc/def/0.html"),
        hints: Vec::new(),
        solution_url: format!("http://www.pythonchallenge.com/pc/def/{}.html", i)
    }
}

fn challenge_02() -> Challenge {
    let cipher:&str =
        "g fmnc wms bgblr rpylqjyrc gr zw fylb. \
         rfyrq ufyr amknsrcpq ypc dmp. bmgle gr gl zw fylb gq glcddgagclr \
         ylb rfyr'q ufw rfgq rcvr gq qm jmle. sqgle qrpgle.kyicrpylq() gq \
         pcamkkclbcb. lmu ynnjw ml rfc spj.";

    fn decipher(cipher: &str, offset: u8) -> String {
        let decode = | &c:&u8 | {
            if c >= b'a' {
                (c - b'a' + offset) % 26 + b'a'
            } else { c }
        };

        return String
            ::from_utf8(
              cipher
                  .as_bytes()
                  .iter()
                  .map(decode)
                  .collect()
            )
            .expect("Could not decode plain text utf8");
    }

    Challenge {
        num: 2,
        url: String::from("http://www.pythonchallenge.com/pc/def/map.html"),
        hints: vec![decipher(&cipher[..], 2)],
        solution_url: format!("http://www.pythonchallenge.com/pc/def/{}.html", decipher(&"map", 2))
    }
}

// fn challenge_03() -> Challenge {
// }

fn main() {
    print!("{}", challenge_01().to_string());
    print!("{}", challenge_02().to_string());
    // print!("{}", challenge_03().to_string());
}
