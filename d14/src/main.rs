extern crate regex;

use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

enum RainState {
    Flying(u32),
    Resting(u32),
}

struct RainDear {
    speed: u32,
    flytime: u32,
    resttime: u32,
    distance: u32,
    state: RainState,
    points: u32,
}

impl RainDear {
    fn tick(&mut self) {
        if let RainState::Flying(mut time) = self.state {
            time -= 1;
            if time == 0 {
                self.state = RainState::Resting(self.resttime);
                self.distance += self.speed;
                return;
            }
            self.state = RainState::Flying(time);
            self.distance += self.speed;
        }
        if let RainState::Resting(mut time) = self.state {
            time -= 1;
            if time == 0 {
                self.state = RainState::Flying(self.flytime);
                return;
            }
            self.state = RainState::Resting(time);
        }
    }
}

fn main() {
    let regex = Regex::new("^\\w+ can fly (\\d+) km/s for (\\d+) seconds, \
but then must rest for (\\d+) seconds\\.").unwrap();
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut rain_dears = Vec::new();
    for wraped in reader.lines() {
        let line = wraped.unwrap();
        let caps = regex.captures(&line).unwrap();
        rain_dears.push(
            RainDear {
                speed: caps[1].parse().unwrap(),
                flytime: caps[2].parse().unwrap(),
                resttime: caps[3].parse().unwrap(),
                distance: 0,
                state: RainState::Flying(caps[2].parse().unwrap()),
                points: 0,
            }
        )
    }
    for _ in 0..2503 {
        for dear in &mut rain_dears {
            dear.tick();
        }
        let mut biggest_distance = 0;
        let mut best_dears = Vec::new();
        for i in rain_dears.iter().enumerate() {
            let (index, dear) = i;
            if dear.distance == biggest_distance {
                best_dears.push(index);
                println!("{:?}", best_dears);
            }
            if dear.distance > biggest_distance {
                biggest_distance = dear.distance;
                best_dears = vec![index; 1];
            }
        }
        for good_dear in best_dears {
            rain_dears[good_dear].points += 1;
        }
    }
    let mut biggest_distance = 0;
    for dear in &rain_dears {
        if dear.distance > biggest_distance {
            biggest_distance = dear.distance;
        }
    }
    let mut best_score = 0;
    for dear in rain_dears {
        if dear.points > best_score {
            best_score = dear.points;
        }
    }
    println!("{}", biggest_distance);
    println!("{}", best_score);
}
