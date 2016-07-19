use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let input = File::open("input.txt").unwrap();
    let input = BufReader::new(input);
    let mut nice = 0;
    for wraped in input.lines() {
        let line = wraped.unwrap();
        let line = line.into_bytes();
        let mut pair = 0;
        let mut repeat = 0;
        let mut iter2 = line.iter();
        iter2.next();
        let mut iter3 = line.iter();
        iter3.nth(1);
        for x in line.iter().enumerate().zip(iter2) {
            let ((s,c), c2) = x;
            let mut iter = line.iter();
            iter.nth(1 + s);
            let mut iter2 = line.iter();
            iter2.nth(2 + s);
            for x in iter.zip(iter2) {
                let (c3, c4) = x;
                if c == c3 && c2 == c4 {
                    pair = 1;
                }
            }
            if let Some(c3) = iter3.next() {
                if c == c3 {
                    repeat = 1;
                 }
            }
        }
        if repeat == 1 && pair == 1 {
            nice += 1;
        }
    }
    println!("{}", nice);
}
