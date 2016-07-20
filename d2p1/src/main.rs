use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

fn main() {
    let input_file;
    if let Ok(x) = File::open("input.txt") {
        input_file = x;
    } else {
        panic!("File not found");
    }
    let mut paper = 0;
    let mut present = Present { l: 0, w: 0, h: 0 };
    let mut l_w_or_h = 0; // l is 0, w is 1, h is 2
    let mut buffer = String::new();
    for c in input_file.bytes() {
        if let Ok(x) = c {
            if x == b'\r' {
                continue;
            }
            if x == b'x' || x == b'\n' {
                if let Ok(x) = u32::from_str(&buffer) {
                    match l_w_or_h {
                        0 => present.l = x,
                        1 => present.w = x,
                        2 => present.h = x,
                        _ => panic!("Bad input"),
                    }
                } else {
                    panic!("Bad input");
                }
                buffer.truncate(0);
                if x == b'\n' {
                    l_w_or_h = 0;
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
                } else {
                    l_w_or_h += 1;
                }
            } else {
                buffer.push(x as char);
            }
        } else {
            panic!("Read error");
        }
    }
    println!("{}", paper);
}
