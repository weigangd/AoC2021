use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut line = String::from("");
    BufReader::new(file).read_line(&mut line).ok();
    let mut population: VecDeque<u64> = VecDeque::from([0; 9]);

    for days_til_propagation in line.trim().split(',').map(|v| v.parse::<usize>().unwrap()) {
        population[days_til_propagation] += 1;
    }
    for _ in 0..80 {
        population.rotate_left(1);
        population[6] += population[8];
    }
    println!("part_a n: {}", population.iter().sum::<u64>());
    for _ in 80..256 {
        population.rotate_left(1);
        population[6] += population[8];
    }
    println!("part_b n: {}", population.iter().sum::<u64>());
}
