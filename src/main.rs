// used modules
extern crate num;
extern crate regex;

mod data;
mod challenges;

// local imports
use challenges::*;

fn main() {
    print!("{}", challenge_01().to_string());
    print!("{}", challenge_02().to_string());
    print!("{}", challenge_03().to_string());
    print!("{}", challenge_04().to_string());
}
