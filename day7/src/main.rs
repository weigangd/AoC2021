use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut line = String::from("");
    BufReader::new(file).read_line(&mut line).ok();
    let crab_positions: Vec<i32> = line.trim().split(',').map(|v| v.parse::<i32>().unwrap()).collect();
    let min = *crab_positions.iter().min().unwrap();
    let max = *crab_positions.iter().max().unwrap();

    let result_part_a = (min..=max).into_iter().map(|i| (i,crab_positions.iter().map(|v|(v-i).abs()).sum::<i32>())).min_by_key(|(_,v)| *v);
    println!("part_a n: {:?}", result_part_a);

    let result_part_b = (min..=max).into_iter().map(|i| (i,crab_positions.iter().map(|v|(1..=(v-i).abs()).sum::<i32>()).sum::<i32>())).min_by_key(|(_,v)| *v);
    println!("part_b n: {:?}", result_part_b);
}
