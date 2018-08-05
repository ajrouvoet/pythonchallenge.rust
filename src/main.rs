// used modules
extern crate num;

mod data;
mod challenges;

// local imports
use challenges::*;

fn main() {
    print!("{}", challenge_01().to_string());
    print!("{}", challenge_02().to_string());
    print!("{}", challenge_03().to_string());
}
