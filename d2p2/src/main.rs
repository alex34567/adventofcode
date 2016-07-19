use std::io::prelude::*;
use std::fs::File;
use std::str::FromStr;

struct Present {
    l: u32,
    w: u32,
    h: u32
}

fn main() {
    let input_file;
    if let Ok(x) = File::open("input.txt") {
        input_file = x;
    } else {
        panic!("File not found");
    }
    let mut ribben = 0;
    let mut present = Present {l: 0, w: 0, h: 0};
    let mut l_w_or_h = 0; // l is 0, w is 1, h is 2
    let mut buffer = String::new();
    for c in input_file.bytes() {
        if let Ok(x) = c {
            if x == b'\r' {
                continue
            }
            if x == b'x' || x == b'\n'{
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
                    let mut smallest_dim = 0; // lw is 0, wh is 1, hl is 2
                    let mut smallest_dim_size = present.l * present.w;
                    if present.w * present.h < smallest_dim_size {
                        smallest_dim_size = present.w * present.h;
                        smallest_dim = 1;
                    }
                    if present.h * present.l < smallest_dim_size {
                        smallest_dim = 2;
                    }
                    match smallest_dim {
                        0 => ribben += (present.l + present.w) * 2,
                        1 => ribben += (present.w + present.h) * 2,
                        2 => ribben += (present.h + present.l) * 2,
                        _ => panic!("This should not happen"),
                    }
                    ribben += present.l * present.w * present.h;
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
    println!("{}", ribben);
}
