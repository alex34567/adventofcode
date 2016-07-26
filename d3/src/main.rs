use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::io::BufReader;
use std::mem;

fn d3(robo: bool) -> u16 {
    let mut map = HashMap::new();
    map.insert((0,0), ());
    let mut current_x = 0;
    let mut current_y = 0;
    let mut houses = 1;
    let mut back_x = 0;
    let mut back_y = 0;
    let input_file = File::open("input.txt").unwrap();
    for wraped in BufReader::new(input_file).lines() {
        let l = wraped.unwrap();
        for x in l.chars() {
            if x == '>' {
                current_x += 1;
                if !map.contains_key(&(current_x, current_y)) {
                    map.insert((current_x,current_y), ());
                    houses += 1;
                }
            } else if x == '<' {
                current_x -= 1;
                if !map.contains_key(&(current_x, current_y)) {
                    map.insert((current_x,current_y), ());
                    houses += 1;
                }
            } else if x == '^' {
                current_y += 1;
                if !map.contains_key(&(current_x, current_y)) {
                    map.insert((current_x,current_y), ());
                    houses += 1;
                }
            } else if x == 'v' {
                current_y -= 1;
                if !map.contains_key(&(current_x, current_y)) {
                    map.insert((current_x,current_y), ());
                    houses += 1;
                }
            }
            if robo {
                mem::swap(&mut current_x, &mut back_x);
                mem::swap(&mut current_y, &mut back_y);
            }
        }
    }
    houses
}

fn main() {
    println!("Santa delivered to {} houses.", d3(false));
    println!("Santa and Robo-Santa delivered to {} houses.", d3(true));
}
