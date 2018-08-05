extern crate num;

mod data;

use std::collections::HashMap;

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

fn challenge_03() -> Challenge {
    let garbage = include_str!("./resources/ocr");
    let mut counts  = HashMap::<u8,(u16,usize)>::new();

    // count them
    for (i, &c) in garbage.as_bytes().iter().enumerate() {
        let (count, _) = counts.entry(c).or_insert((0, i));
        *count += 1;
    }

    // filter the ones that occur once
    let mut ones:Vec<(&u8,&usize)> = counts
        .iter()
        .filter(| (_, (v, _)) | *v == 1)
        .map(|(k, (_, pos))| (k, pos))
        .collect();

    // in place sorting of ones by their position
    ones.sort_by(|(_, k), (_, l)| k.cmp(l));

    let solution:String = String::from_utf8(
        ones.iter()
            .map(|(&c, _)| c.clone())
            .collect()
      )
      .expect("could not decode utf8 string");

    Challenge {
        num: 3,
        url: String::from("http://www.pythonchallenge.com/pc/def/ocr.html"),
        hints: vec![solution],
        solution_url: String::from("http://www.pythonchallenge.com/pc/def/equality.html"),
    }
}

fn main() {
    print!("{}", challenge_01().to_string());
    print!("{}", challenge_02().to_string());
    print!("{}", challenge_03().to_string());
}
