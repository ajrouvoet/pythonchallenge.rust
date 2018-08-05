extern crate num;

use num::{BigUint};
use num::pow::pow;

struct Challenge {
    num: u8,
    url: String,
    hints: Vec<String>,

    solution_url: String
}

impl ToString for Challenge {
  fn to_string(&self) -> String {
      let title = format!(" Challenge {} ", self.num);
      let bar = String::from(format!("{:-^1$}\n", title, 80));
      let hints: String = self.hints.iter().map(|h| format!("  - {}\n", h)).collect();
      return bar
          + &format!("from: {}\n", self.url)
          + &format!("to: {}\n", self.solution_url)
          + &format!("via: \n{}", hints);
  }
}

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
    let cipher:&[u8] =
        b"g fmnc wms bgblr rpylqjyrc gr zw fylb. \
         rfyrq ufyr amknsrcpq ypc dmp. bmgle gr gl zw fylb gq glcddgagclr \
         ylb rfyr'q ufw rfgq rcvr gq qm jmle. sqgle qrpgle.kyicrpylq() gq \
         pcamkkclbcb. lmu ynnjw ml rfc spj.";


    fn decipher(cipher: &[u8], offset: u8) -> String {
        let decode = | &c | (if c > 96 {(c - 97 + offset) % 26 + 97} else { c });
        return String
            ::from_utf8(
              cipher
                  .iter()
                  .map(decode)
                  .collect()
            )
            .expect("Could not decode plain text utf8");
    }

    Challenge {
        num: 2,
        url: String::from("http://www.pythonchallenge.com/pc/def/map.html"),
        hints: vec![decipher(cipher, 2)],
        solution_url: format!("http://www.pythonchallenge.com/pc/def/{}.html", decipher(b"map", 2))
    }
}

fn main() {
    print!("{}", challenge_01().to_string());
    print!("{}", challenge_02().to_string());
}
