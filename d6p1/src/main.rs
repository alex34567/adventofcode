extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let digit_rex = Regex::new("\\d+").unwrap();
    let input = BufReader::new(File::open("input.txt").unwrap());
    let mut lights = [[0u8; 1000]; 1000];
    let mut lights_on = 0;
    for wraped in input.lines() {
        let line = wraped.unwrap();
        let mut from_x = 0;
        let mut from_y = 0;
        let mut to_x = 0;
        let mut to_y = 0;
        for x in digit_rex.find_iter(&line).enumerate() {
            let (count, (start, end)) = x;
            let number = &line.as_bytes()[start..end];
            let number = std::str::from_utf8(number).unwrap();
            match count {
                0 => from_x = number.parse().unwrap(),
                1 => from_y = number.parse().unwrap(),
                2 => to_x = number.parse().unwrap(),
                3 => to_y = number.parse().unwrap(),
                _ => panic!("Bad input"),
            }
        }
        let from_x = from_x;
        let from_y = from_y;
        let to_x = to_x;
        let to_y = to_y;
        let line_ref = &line;
        if line_ref.contains("turn on ") {
            for y in from_y..to_y + 1 {
                for x in from_x..to_x + 1 {
                    if lights[y][x] == 0 {
                        lights[y][x] = 1;
                        lights_on += 1;
                    }
                }
            }
        }
        if line_ref.contains("turn off ") {
            for y in from_y..to_y + 1 {
                for x in from_x..to_x + 1 {
                    if lights[y][x] == 1 {
                        lights[y][x] = 0;
                        lights_on -= 1;
                    }
                }
            }
        }
        if line_ref.contains("toggle ") {
            for y in from_y..to_y + 1 {
                for x in from_x..to_x + 1 {
                    if lights[y][x] == 0 {
                        lights[y][x] = 1;
                        lights_on += 1;
                    } else if lights[y][x] == 1 {
                        lights[y][x] = 0;
                        lights_on -= 1;
                    }
                }
            }
        }
    }
    println!("{}", lights_on);
}
