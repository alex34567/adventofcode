use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut floor = 0;
    let input_file;
    if let Ok(x) = File::open("input.txt") {
        input_file = x;
    } else {
        panic!("File not found")
    }
    for x in input_file.bytes().enumerate() {
        let (s, z) = x;
        if let Ok(y) = z {
            if y == b'(' {
                floor += 1;
            } else if y == b')' {
                floor -= 1;
            }
        } else {
            panic!("Read error");
        }
        if floor == -1 {
            println!("{}", s + 1);
            break
        }
    }
}
