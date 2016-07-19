use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let input = File::open("input.txt").unwrap();
    let input = BufReader::new(input);
    let mut escaped_chars = 0;
    let mut total_chars = 0;
    for wraped in input.lines() {
        let line = wraped.unwrap();
        total_chars += line.len();
        escaped_chars += line.len();
        escaped_chars += 4;
        let mut iter = line.chars();
        while let Some(c) = iter.next() {
            if c == '\\' {
                escaped_chars += 1;
                if let Some(c) = iter.next() {
                    if c == '"' || c == '\\' {
                        escaped_chars += 1;
                    }
                }
            }
        }
    }
    println!("{}", escaped_chars - total_chars);
}
