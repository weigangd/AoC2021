use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let lines: Vec<(i16, i16, i16, i16)> = BufReader::new(file)
        .lines()
        .map(|x| {
            let x = x.unwrap().replace(" -> ", ",");
            let mut it = x.split(',').map(|v| v.parse::<i16>().unwrap());
            (
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
            )
        })
        .collect();
    let mut map = std::collections::HashMap::new();
    for &(x1, y1, x2, y2) in lines.iter() {
        if x1 == x2 {
            let y_start = y1.min(y2);
            let y_end = y1.max(y2) + 1;
            for y in y_start..y_end {
                let e = map.entry((x1, y)).or_insert(0);
                *e += 1;
            }
        } else if y1 == y2 {
            let x_start = x1.min(x2);
            let x_end = x1.max(x2) + 1;
            for x in x_start..x_end {
                let e = map.entry((x, y1)).or_insert(0);
                *e += 1;
            }
        }
    }
    let n = map.iter().filter(|(_, v)| **v > 1).count();
    println!("part a n: {}", n);
    let mut map = std::collections::HashMap::new();
    for &(x1, y1, x2, y2) in lines.iter() {
        let mut x = x1;
        let mut y = y1;
        while x != x2 || y != y2 {
            let x_step = (x2 - x).signum();
            let y_step = (y2 - y).signum();
            let e = map.entry((x, y)).or_insert(0);
            *e += 1;
            x += x_step;
            y += y_step;
        }
        let e = map.entry((x2, y2)).or_insert(0);
        *e += 1;
    }
    let n = map.iter().filter(|(_, v)| **v > 1).count();
    println!("part b n: {}", n);
}
