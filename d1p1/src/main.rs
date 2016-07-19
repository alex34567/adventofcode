use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut floor = 0;
    let input_file = File::open("input.txt").unwrap();
    for x in input_file.bytes() {
        let y = x.unwrap();
	if y == b'^' {
            floor += 1;
        } else if y == b'v' {
            floor -= 1;
        }
        if floor == -1 {
            break
        }
    }
    println!("{}", floor);
}
