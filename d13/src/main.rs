extern crate regex;
use regex::Regex;
use regex::Captures;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn happy_expand(happyness_table: &mut Vec<Vec<i32>>, len: usize) {
    let new_vec;
    if len != 0 {
        new_vec = vec![0; len];
    } else {
        new_vec = Vec::new();
    }
    happyness_table.push(new_vec);
    for vec in happyness_table {
            vec.push(0);
    }
}

fn happy_parse(happyness_table: &mut Vec<Vec<i32>>,
               map: &mut HashMap<String, usize>, caps: &Captures, down: bool) {
    let len = map.len();
    let index1 = *map.entry(caps[1].to_string()).or_insert(len);
    if len != map.len() {
        happy_expand(happyness_table, len);
    }
    let len = map.len();
    let index2 = *map.entry(caps[3].to_string()).or_insert(len);
    if len != map.len() {
        happy_expand(happyness_table, len);
    }
    let mut happyness = caps[2].parse().unwrap();
    if down {
        happyness *= -1;
    }
    happyness_table[index1][index2] = happyness;
}

fn permutate(round_table: &Vec<usize>, happy_table: &Vec<Vec<i32>>)  -> i32 {
    if round_table.len() != happy_table.len() {
        let mut opt_happyness = std::i32::MIN;
        let mut test_table = round_table.clone();
        test_table.push(0);
        for x in 0..happy_table.len() {
            let mut continue_out = false;
            for value in round_table {
                if x == *value {
                    continue_out = true;
                    break;
                }
            }
            if continue_out {
                continue;
            }
            *test_table.last_mut().unwrap() = x;
            let happyness = permutate(&test_table, &happy_table);
            if happyness > opt_happyness {
                opt_happyness = happyness;
            }
        }
        opt_happyness
    } else {
        let mut happy_sum = 0;
        for i in round_table.iter()
            .zip(round_table.iter().skip(1)).zip(round_table.iter().skip(2)) {
            let ((person1, person2), person3) = i;
            happy_sum += happy_table[*person2][*person1];
            happy_sum += happy_table[*person2][*person3];
        }
        let first = round_table[0];
        let second = round_table[1];
        let last = round_table[happy_table.len() - 1];
        let second_to_last = round_table[happy_table.len() - 2];
        happy_sum += happy_table[last][second_to_last];
        happy_sum += happy_table[last][first];
        happy_sum += happy_table[first][second];
        happy_sum += happy_table[first][last];
        happy_sum
    }
}

fn main() {
    let gain_regex =
        Regex::new(r"^(\w+) would gain (\d+) happiness units by sitting next to (\w+).").unwrap();
    let lose_regex =
        Regex::new(r"^(\w+) would lose (\d+) happiness units by sitting next to (\w+).").unwrap();
    let reader = io::BufReader::new(File::open("input.txt").unwrap());
    let mut map = HashMap::new();
    let mut happyness_table = Vec::new();
    for wraped in reader.lines() {
        let line = wraped.unwrap();
        if let Some(caps) = gain_regex.captures(&line) {
            happy_parse(&mut happyness_table ,&mut map, &caps, false);
        }
        if let Some(caps) = lose_regex.captures(&line) {
            happy_parse(&mut happyness_table, &mut map, &caps, true);
        }
    }
    println!("The happiest table possible is {}", permutate(&Vec::new(), &happyness_table));
    let len = happyness_table.len() + 1;
    happy_expand(&mut happyness_table, len);
    println!("The happiest table possible including myself is {}", permutate(&Vec::new(), &happyness_table));
}
