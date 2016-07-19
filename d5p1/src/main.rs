use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let input = File::open("input.txt").unwrap();
    let input = BufReader::new(input);
    let mut nice = 0;
    for line in input.lines() {
        let mut vowels = 0;
        let mut double_letter = 0;
        let mut bad_substring = 0;
        let mut substring: [u8; 2] = [0; 2];
        for c in line.unwrap().chars() {
            substring[0] = substring[1];
            substring[1] = c as u8;
            if c == 'a' || c == 'e' || 
                    c == 'i' || c == 'o' || c == 'u' {
                vowels += 1;
            }
            if substring[0] == substring[1] {
                double_letter = 1;
            }
            if substring[0] == b'a' && substring[1] == b'b' ||
                    substring[0] == b'c' && substring[1] == b'd' ||
                    substring[0] == b'p' && substring[1] == b'q' ||
                    substring[0] == b'x' && substring[1] == b'y' {
                bad_substring = 1;
                break;
            }
        }
        if vowels >= 3 && double_letter == 1 && bad_substring == 0 {
            nice += 1;
        }
    }
    println!("{}", nice);
}
