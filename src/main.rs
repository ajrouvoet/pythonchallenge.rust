extern crate num;

use num::{BigUint};
use num::pow::pow;

struct Challenge {
    num: u8,
    url: String,

    solution_url: String
}

impl ToString for Challenge {
  fn to_string(&self) -> String {
      let title = format!(" Challenge {} ", self.num);
      let bar = String::from(format!("{:-^1$}\n", title, 80));
      return bar + &format!("from: {}\n", self.url) + &format!("to: {}\n", self.solution_url);
  }
}

fn challenge_01() -> Challenge {
    // calculate the asked for big number
    let i = pow(BigUint::from(2u32), 38);

    Challenge {
        num: 1,
        url: String::from("http://www.pythonchallenge.com/pc/def/0.html"),
        solution_url: format!("http://www.pythonchallenge.com/pc/def/{}.html", i)
    }
}

fn challenge_02() -> Challenge {
    Challenge {
        num: 2,
        url: String::from("http://www.pythonchallenge.com/pc/def/map.html"),
        solution_url: format!("")
    }
}

fn main() {
    print!("{}", challenge_01().to_string());
    print!("{}", challenge_02().to_string());
}
