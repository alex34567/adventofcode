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

fn main() {
    let input_file;
    input_file = BufReader::new(File::open("input.txt").unwrap());
    let regex = Regex::new(r"(\d+)x(\d+)x(\d+)").unwrap();
    let mut paper = 0;
    let mut present = Present { l: 0, w: 0, h: 0 };
    for wraped in input_file.lines() {
        let line = wraped.unwrap();
        let captures = regex.captures(&line).unwrap();
        present.l = captures[1].parse().unwrap();
        present.w = captures[2].parse().unwrap();
        present.h = captures[3].parse().unwrap();
        let mut smallest_dim = present.l * present.w;
        if present.w * present.h < smallest_dim {
            smallest_dim = present.w * present.h;
        }
        if present.h * present.l < smallest_dim {
            smallest_dim = present.h * present.l;
        }
        paper += 2 * present.l * present.w;
        paper += 2 * present.w * present.h;
        paper += 2 * present.h * present.l;
        paper += smallest_dim;
    }
    println!("{}", paper);
}
