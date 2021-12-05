use std::fs::File;
use std::io::{BufRead, BufReader};

struct BingoGame {
    boards: Vec<Board>,
    draws: Vec<u8>,
}

impl BingoGame {
    fn new(boards: Vec<Board>) -> Self {
        Self {
            boards,
            draws: vec![],
        }
    }

    fn handle_draw(&mut self, draw: u8) -> Vec<Board> {
        self.draws.push(draw);

        //HINT: use drain_filter when it's stable
        let mut i = 0;
        let mut winners = Vec::with_capacity(1);
        while i < self.boards.len() {
            if self.boards[i].mark_number(draw) {
                winners.push(self.boards.swap_remove(i));
            } else {
                i += 1;
            }
        }
        winners
    }
}

#[derive(Debug, Clone)]
struct Board {
    numbers: [u8; Self::N],
    marks: [bool; Self::N],
}

#[allow(non_upper_case_globals)]
impl Board {
    const n: usize = 5;
    const N: usize = Self::n * Self::n;

    fn new(numbers: [u8; Self::N]) -> Self {
        Self {
            numbers,
            marks: [false; Self::N],
        }
    }

    fn mark_number(&mut self, draw: u8) -> bool {
        self.numbers
            .iter()
            .position(|v| *v == draw)
            .map(|i| {
                self.marks[i] = true;
                let row = i / Self::n;
                let col = i % Self::n;
                self.check_row(row) || self.check_col(col)
            })
            .unwrap_or(false)
    }

    fn check_row(&self, row: usize) -> bool {
        let start = row * Self::n;
        let end = start + Self::n;
        self.marks[start..end].iter().all(|x| *x)
    }

    fn check_col(&self, col: usize) -> bool {
        self.marks[col..].iter().step_by(Self::n).all(|x| *x)
    }

    fn get_unmarked_numbers(&self) -> impl std::iter::Iterator<Item = u8> + '_ {
        self.numbers.iter().enumerate().filter_map(
            |(i, v)| {
                if !self.marks[i] {
                    Some(*v)
                } else {
                    None
                }
            },
        )
    }
}

fn parse_bingo_game() -> (Vec<u8>, BingoGame) {
    let file = File::open("input.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().flatten().collect();
    let draws = lines[0].split(',').map(|x| x.parse::<u8>().unwrap());
    let mut boards = vec![];
    let mut numbers = [0; Board::N];
    let mut numbers_it = lines[1..]
        .iter()
        .filter(|x| !x.is_empty())
        .flat_map(|line| line.split_whitespace().map(|x| x.parse::<u8>().unwrap()))
        .peekable();
    while numbers_it.peek().is_some() {
        for v in numbers.iter_mut() {
            *v = numbers_it.next().unwrap();
        }
        boards.push(Board::new(numbers));
    }
    (draws.collect(), BingoGame::new(boards))
}

fn part_a() {
    println!("----- Part a -----");
    let (draws, mut game) = parse_bingo_game();
    for draw in draws {
        if let Some(winning_board) = game.handle_draw(draw).get(0) {
            println!("draw: {}", draw);
            println!(
                "result: {}",
                winning_board
                    .get_unmarked_numbers()
                    .into_iter()
                    .map(|v| v as u32)
                    .sum::<u32>()
                    * draw as u32
            );
            break;
        }
    }
}

fn part_b() {
    println!("----- Part b -----");
    let (draws, mut game) = parse_bingo_game();
    let mut last_winning_board = None;
    let mut last_winning_draw = 0;
    for draw in draws {
        if let Some(winning_board) = game.handle_draw(draw).pop() {
            last_winning_board.replace(winning_board);
            last_winning_draw = draw;
        }
    }
    let last_winning_board = last_winning_board.expect("No board won the game");
    println!("draw: {}", last_winning_draw);
    println!(
        "result: {}",
        last_winning_board
            .get_unmarked_numbers()
            .into_iter()
            .map(|v| v as u32)
            .sum::<u32>()
            * last_winning_draw as u32
    );
}

fn main() {
    part_a();
    part_b();
}
