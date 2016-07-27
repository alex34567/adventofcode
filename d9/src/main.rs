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
    let mut routes = Vec::with_capacity(jumps.len());
    for x in 0..route_size {
        routes.push(Route { 
         stops: Vec::with_capacity(1),
         distance: 0,
         });
        routes[x].stops.push(x);
    }
    while routes[0].stops.len() != route_size {
        let mut new_routes = Vec::with_capacity(routes.len());
        for route in &routes {
            let santa_loc = *route.stops.last().unwrap();
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
                    new_routes.push(new_route);
                }
            }
        }
        routes = new_routes;
    }
    let mut shortest_distance = std::usize::MAX;
    for route in &routes {
        if route.distance < shortest_distance {
            shortest_distance = route.distance;
        }
    }
    println!("The shortest distance that Santa can deliver all the pressents is {}", shortest_distance);
    let mut longest_distance = 0;
    for route in &routes {
        if route.distance > longest_distance {
            longest_distance = route.distance;
        }
    }
    println!("The longest distance that Santa can deliver all the pressents is {}", longest_distance);
}
