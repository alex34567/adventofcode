extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::Read;

fn main() {
    let regex = Regex::new(r"(-?\d+)").unwrap();
    let mut file = File::open("input.txt").unwrap();
    let mut string = String::new();
    let mut sum: i32 = 0;
    file.read_to_string(&mut string).unwrap();
    drop(file);
    for numbers in regex.captures_iter(&string) {
        sum += numbers[1].parse().unwrap()
    }
    println!("The sum of all the numbers is {}", sum);
}
