use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().parse().unwrap())
        .collect();
    let count = (1..lines.len())
        .filter(|i| lines[*i] > lines[*i - 1])
        .count();
    println!("Count increasing {}", count);
    let count_sums = (1..lines.len() - 2)
        .filter(|i| lines[*i + 2] > lines[*i - 1])
        .count();
    println!("Count increasing sums {}", count_sums);
}
