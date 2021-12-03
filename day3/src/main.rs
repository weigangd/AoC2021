use std::fs::File;
use std::io::{BufRead, BufReader};

const N: usize = 12;

fn get_gamma_and_epsilon(lines: &[String]) -> (String, String) {
    let mut sums = [0; N];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            sums[i] += c.to_digit(10).unwrap();
        }
    }

    let line_count = lines.len() as u32;
    let mut gamma = String::from("");
    let mut epsilon = String::from("");
    for sum in sums {
        let gamma_bit = (sum >= (line_count + 1) / 2) as u8;
        gamma = format!("{}{}", gamma, gamma_bit);
        epsilon = format!("{}{}", epsilon, 1 - gamma_bit);
    }
    (gamma, epsilon)
}

fn part_a() {
    println!("----- Part a -----");
    let file = File::open("input.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();
    let (gamma, epsilon) = get_gamma_and_epsilon(&lines);
    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon, 2).unwrap();
    println!("gamma: {}", gamma);
    println!("epsilon: {}", epsilon);
    println!("power consumption: {}", gamma * epsilon);
}

fn part_b() {
    println!("----- Part b -----");
    let file = File::open("input.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();
    let mut oxygen = lines.clone();
    for i in 1..N + 1 {
        if oxygen.len() == 1 {
            break;
        }
        let (gamma, _) = get_gamma_and_epsilon(&oxygen);
        oxygen.retain(|s| s[i - 1..i] == gamma[i - 1..i]);
    }

    let mut co2 = lines;
    for i in 1..N + 1 {
        if co2.len() == 1 {
            break;
        }
        let (_, epsilon) = get_gamma_and_epsilon(&co2);
        co2.retain(|s| s[i - 1..i] == epsilon[i - 1..i]);
    }
    let oxygen = u32::from_str_radix(&oxygen[0], 2).unwrap();
    let co2 = u32::from_str_radix(&co2[0], 2).unwrap();
    println!("Oxygen: {}", oxygen);
    println!("CO2: {}", co2);
    println!("Life Support: {}", oxygen * co2);
}

fn main() {
    part_a();
    part_b();
}
