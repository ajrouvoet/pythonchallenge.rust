use std::collections::HashMap;

use data::challenge::{Challenge};

pub fn challenge_03() -> Challenge {
    let garbage = include_str!("../resources/ocr");
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
