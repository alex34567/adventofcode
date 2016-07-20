extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cell::Cell;

#[derive(Copy, Clone)]
struct WireStatic {
    from_index: usize,
    number: u16,
    l_o_r_shift: u8,
}

#[derive(Copy, Clone)]
struct TwoWires {
    wire1: usize,
    wire2: usize,
    and_o_or: u8,
}

#[derive(Copy, Clone)]
enum Wire {
    Static(u16), // Number
    OneWire(usize), // Index of wire to get value
    WireStatic(WireStatic),
    StaticWire(usize), // Index of wire to get value
    WireToWire(usize), // Index of wire to get value
    TwoWires(TwoWires),
    None,
}

#[derive(Clone)]
struct WireCell {
    wire: Cell<Wire>,
}

impl WireCell {
    fn get_value(&self, ref wires: &Vec<WireCell>) -> u16 {
        match self.wire.get() {
            Wire::Static(number) => return number,
            Wire::OneWire(index) => {
                let output = wires[index].get_value(&wires) ^ 0xFFFF;
                self.wire.set(Wire::Static(output));
                return output;
            }
            Wire::WireStatic(wirestatic) => {
                let output;
                if wirestatic.l_o_r_shift == 0 {
                    output = wires[wirestatic.from_index].get_value(&wires) << wirestatic.number;
                } else if wirestatic.l_o_r_shift == 1 {
                    output = wires[wirestatic.from_index].get_value(&wires) >> wirestatic.number;
                } else {
                    panic!("This should not happen");
                }
                self.wire.set(Wire::Static(output));
                return output;
            }
            Wire::StaticWire(from_index) => {
                let output = 1 & wires[from_index].get_value(&wires);
                self.wire.set(Wire::Static(output));
                return output;
            }
            Wire::WireToWire(from_index) => {
                let output = wires[from_index].get_value(&wires);
                self.wire.set(Wire::Static(output));
                return output;
            }
            Wire::TwoWires(two_wire) => {
                let wire1 = wires[two_wire.wire1].get_value(&wires);
                let wire2 = wires[two_wire.wire2].get_value(&wires);
                let output;
                if two_wire.and_o_or == 0 {
                    output = wire1 & wire2;
                } else if two_wire.and_o_or == 1 {
                    output = wire1 | wire2;
                } else {
                    panic!("This should not happen");
                }
                self.wire.set(Wire::Static(output));
                return output;
            }
            Wire::None => panic!("Wire not connected"),
        }
    }
}

fn ascii_to_index(ascii: &str) -> usize {
    let ascii = ascii.as_bytes();
    if ascii.len() > 2 || ascii.len() < 1 {
        panic!("Ascii is too big");
    }

    if ascii[0] < b'a' || ascii[0] > b'z' {
        panic!("Not lowercase letters");
    }

    let half_output = ascii[0] as usize - 97;

    if ascii.len() == 1 {
        return half_output;
    }

    if ascii[1] < b'a' || ascii[1] > b'z' {
        panic!("Not lowercase letters");
    }

    let other_half = (ascii[1] as usize - 96) * 26;
    (other_half) + half_output
}

fn main() {
    let static_rex = Regex::new(r"^(\d+) -> ([a-z]+)").unwrap();
    let one_wire_rex = Regex::new(r"^NOT+ ([a-z]+) -> ([a-z]+)").unwrap();
    let wire_static_rex = Regex::new(r"^([a-z]+) ([A-Z]+) (\d+) -> ([a-z]+)").unwrap();
    let static_wire_rex = Regex::new(r"^1 AND ([a-z]+) -> ([a-z]+)").unwrap();
    let wire_to_wire_rex = Regex::new(r"^([a-z]+) -> ([a-z]+)").unwrap();
    let two_wire_rex = Regex::new(r"^([a-z]+) ([A-Z]+) ([a-z]+) -> ([a-z]+)").unwrap();
    let input = BufReader::new(File::open("input.txt").unwrap());
    let mut wires = Vec::new();
    for _ in 0..702 {
        wires.push(WireCell { wire: Cell::new(Wire::None) });
    }
    for wraped in input.lines() {
        let line = wraped.unwrap();
        if static_rex.is_match(&line) {
            let caps = static_rex.captures(&line).unwrap();
            let number = caps[1].parse().unwrap();
            let index = ascii_to_index(&caps[2]);
            wires[index].wire.set(Wire::Static(number));
        } else if one_wire_rex.is_match(&line) {
            let caps = one_wire_rex.captures(&line).unwrap();
            let from_index = ascii_to_index(&caps[1]);
            let index = ascii_to_index(&caps[2]);
            wires[index].wire.set(Wire::OneWire(from_index));
        } else if wire_static_rex.is_match(&line) {
            let caps = wire_static_rex.captures(&line).unwrap();
            let from_index = ascii_to_index(&caps[1]);
            let number = caps[3].parse().unwrap();
            let index = ascii_to_index(&caps[4]);
            let l_o_r_shift;
            if &caps[2] == "LSHIFT" {
                l_o_r_shift = 0;
            } else if &caps[2] == "RSHIFT" {
                l_o_r_shift = 1;
            } else {
                panic!("Invalid input");
            }
            wires[index].wire.set(Wire::WireStatic(WireStatic {
                from_index: from_index,
                number: number,
                l_o_r_shift: l_o_r_shift,
            }));
        } else if static_wire_rex.is_match(&line) {
            let caps = static_wire_rex.captures(&line).unwrap();
            let from_index = ascii_to_index(&caps[1]);
            let index = ascii_to_index(&caps[2]);
            wires[index].wire.set(Wire::StaticWire(from_index));
        } else if wire_to_wire_rex.is_match(&line) {
            let caps = wire_to_wire_rex.captures(&line).unwrap();
            let from_index = ascii_to_index(&caps[1]);
            let index = ascii_to_index(&caps[2]);
            wires[index].wire.set(Wire::WireToWire(from_index));
        } else if two_wire_rex.is_match(&line) {
            let caps = two_wire_rex.captures(&line).unwrap();
            let wire1 = ascii_to_index(&caps[1]);
            let and_o_or;
            if &caps[2] == "AND" {
                and_o_or = 0;
            } else if &caps[2] == "OR" {
                and_o_or = 1;
            } else {
                panic!("Invalid input");
            }
            let wire2 = ascii_to_index(&caps[3]);
            let index = ascii_to_index(&caps[4]);
            wires[index].wire.set(Wire::TwoWires(TwoWires {
                wire1: wire1,
                wire2: wire2,
                and_o_or: and_o_or,
            }));
        } else {
            panic!("Invalid input");
        }
    }
    let wires_back = wires.clone();
    let b_number = wires[0].get_value(&wires);
    wires_back[1].wire.set(Wire::Static(b_number));
    println!("{}", wires_back[0].get_value(&wires_back));
}
