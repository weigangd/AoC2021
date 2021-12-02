use std::fs::File;
use std::io::{BufRead, BufReader};

use std::str::FromStr;

struct Submarine {
    aim: i32,
    depth: i32,
    horizontal_position: i32,
}

impl Submarine {
    fn new() -> Self {
        Self {
            aim: 0,
            depth: 0,
            horizontal_position: 0,
        }
    }

    fn make_move(&mut self, movement: Movement) {
        match movement {
            Movement::Up(v) => self.depth -= v,
            Movement::Down(v) => self.depth += v,
            Movement::Forward(v) => self.horizontal_position += v,
        }
    }

    fn make_aiming_move(&mut self, movement: Movement) {
        match movement {
            Movement::Up(v) => self.aim -= v,
            Movement::Down(v) => self.aim += v,
            Movement::Forward(v) => {
                self.horizontal_position += v;
                self.depth += self.aim * v
            }
        }
    }
}

#[derive(Debug)]
enum Movement {
    Up(i32),
    Down(i32),
    Forward(i32),
}

impl FromStr for Movement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.split(' ');

        match (line.next(), line.next().map(|v| v.parse().ok()).flatten()) {
            (Some("up"), Some(v)) => Ok(Movement::Up(v)),
            (Some("down"), Some(v)) => Ok(Movement::Down(v)),
            (Some("forward"), Some(v)) => Ok(Movement::Forward(v)),
            _ => Err(String::from("Could not parse Movement")),
        }
    }
}

fn part_a() {
    println!("----- Part a -----");
    let mut sub = Submarine::new();
    let file = File::open("input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        sub.make_move(Movement::from_str(&line.unwrap()).unwrap());
    }
    println!(
        "Product of depth and horizontal position: {}",
        sub.depth * sub.horizontal_position
    );
}

fn part_b() {
    println!("----- Part b -----");
    let mut sub = Submarine::new();
    let file = File::open("input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        sub.make_aiming_move(Movement::from_str(&line.unwrap()).unwrap());
    }
    println!(
        "Product of depth and horizontal position: {}",
        sub.depth * sub.horizontal_position
    );
}

fn main() {
    part_a();
    part_b();
}
