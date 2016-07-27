extern crate regex;
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
struct Route {
    stops: Vec<usize>,
    distance: usize,
}

fn permutate(route: Route, route_size: usize, jumps: &[Vec<(usize, usize)>]) -> (usize, usize) {
    let santa_loc = *route.stops.last().unwrap();
    let mut smallest = std::usize::MAX;
    let mut biggest = 0;
    for jump in &jumps[santa_loc] {
        let mut valid = true;
        for stop in &route.stops {
            if jump.0 == *stop {
                valid = false;
            }
        }
        if valid {
            let mut new_route = route.clone();
            new_route.stops.push(jump.0);
            new_route.distance += jump.1;
            if new_route.stops.len() < route_size {
                let (route_smallest, route_biggest) = permutate(new_route, route_size, jumps);
                if route_smallest < smallest {
                    smallest = route_smallest;
                }
                if route_biggest > biggest {
                    biggest = route_biggest;
                }
            } else {
                if new_route.distance < smallest {
                    smallest = new_route.distance;
                }
                if new_route.distance > biggest {
                    biggest = new_route.distance;
                }
            }
        }
    }
    (smallest, biggest)
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let input = BufReader::new(input);
    let mut index_map = HashMap::new();
    let mut jumps = Vec::new();
    let regex = Regex::new(r"^(\w+) to (\w+) = (\d+)").unwrap();
    for wraped in input.lines() {
        let line = wraped.unwrap();
        let caps = regex.captures(&line).unwrap();
        let len = index_map.len();
        let index1 = *index_map.entry(caps[1].to_string()).or_insert(len);
        if len != index_map.len() {
            jumps.push(Vec::new());
        }
        let len = index_map.len();
        let index2 = *index_map.entry(caps[2].to_string()).or_insert(len);
        if len != index_map.len() {
            jumps.push(Vec::new());
        }
        let distance: usize = caps[3].parse().unwrap();
        jumps[index1].push((index2, distance));
        jumps[index2].push((index1, distance));
    }
    let route_size = index_map.len();
    drop(index_map);
    let mut shortest_distance = std::usize::MAX;
    let mut longest_distance = 0;
    for x in 0..route_size {
        let mut route = Route {
            stops: Vec::with_capacity(1),
            distance: 0,
        };
        route.stops.push(x);
        let (route_smallest, route_biggest) = permutate(route, route_size, &jumps);
        if route_smallest < shortest_distance {
            shortest_distance = route_smallest;
        }
        if route_biggest > longest_distance {
            longest_distance = route_biggest;
        }
    }
    println!("The shortest distance that Santa can deliver all the pressents is {}",
             shortest_distance);
    println!("The longest distance that Santa can deliver all the pressents is {}",
             longest_distance);
}
