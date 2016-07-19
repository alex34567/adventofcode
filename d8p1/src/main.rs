use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let input = File::open("input.txt").unwrap();
    let input = BufReader::new(input);
    let mut mem_chars = 0;
    let mut total_chars = 0;
    for wraped in input.lines() {
        let line = wraped.unwrap();
        total_chars += line.len();
        let mut iter = line.chars();
        while let Some(c) = iter.next() {
            if c == '\\' {
                if let Some(c) = iter.next() {
                    if c == 'x' {
                        iter.nth(1);
                    }
                }
            }
            mem_chars += 1;
        }
        mem_chars -= 2;
    }
    println!("{}", total_chars - mem_chars);
}
