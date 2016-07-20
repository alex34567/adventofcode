use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut floor = 0;
    let mut basement_index = Option::None;
    let input_file = BufReader::new(File::open("input.txt").unwrap());
    for x in input_file.lines() {
        for x in x.unwrap().chars().enumerate() {
            let (index, c) = x;
            if c == '(' {
                floor += 1;
            } else if c == ')' {
                floor -= 1;
            }
            if let None = basement_index {
                if floor == -1 {
                    basement_index = Option::Some(index + 1);
                }
            }
        }
    }
    println!("Santa is on floor {}", floor);
    if let Some(basement_index) = basement_index {
        println!("Santa entered the basement at position {}", basement_index);
    }
}
