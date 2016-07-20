extern crate regex;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

enum SmallestDim {
    LengthTimesWidth(u32),
    WidthTimesHeight(u32),
    HeightTimesLength(u32),
}

impl SmallestDim {
    fn unwrap(&self) -> u32 {
        match self {
            &SmallestDim::LengthTimesWidth(x) => return x,
            &SmallestDim::WidthTimesHeight(x) => return x,
            &SmallestDim::HeightTimesLength(x) => return x,
        }
    }
}

fn main() {
    let input_file = BufReader::new(File::open("input.txt").unwrap());
    let regex = Regex::new(r"^(\d+)x(\d+)x(\d+)").unwrap();
    let mut paper = 0;
    let mut ribben = 0;
    for wraped in input_file.lines() {
        let line = wraped.unwrap();
        let captures = regex.captures(&line).unwrap();
        let mut present = Present { l: 0, w: 0, h: 0 };
        present.l = captures[1].parse().unwrap();
        present.w = captures[2].parse().unwrap();
        present.h = captures[3].parse().unwrap();
        let mut smallest_dim = SmallestDim::LengthTimesWidth(present.l * present.w);
        if present.w * present.h < smallest_dim.unwrap() {
            smallest_dim = SmallestDim::WidthTimesHeight(present.w * present.h);
        }
        if present.h * present.l < smallest_dim.unwrap() {
            smallest_dim = SmallestDim::HeightTimesLength(present.h * present.l);
        }
        paper += 2 * present.l * present.w;
        paper += 2 * present.w * present.h;
        paper += 2 * present.h * present.l;
        paper += smallest_dim.unwrap();
        ribben += present.l * present.w * present.h;
        match smallest_dim {
            SmallestDim::LengthTimesWidth(_) => ribben += 2 * (present.l + present.w),
            SmallestDim::WidthTimesHeight(_) => ribben += 2 * (present.w + present.h),
            SmallestDim::HeightTimesLength(_) => ribben += 2 * (present.h + present.l),
        }
    }
    println!("The elfs need to order {} square feet of paper", paper);
    println!("The elfs need to order {} feet of ribben", ribben);
}
