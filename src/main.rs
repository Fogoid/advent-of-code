use clap::{Command, arg};

use crate::commons::Solution;

pub mod commons;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub const SOLUTIONS: &[&dyn Solution] = &[
    &day01::Day01,
    &day02::Day02,
    &day03::Day03,
    &day04::Day04,
];

fn main() {
    let cmd = Command::new("aoc23")
        .args(&[
            arg!(<PUZZLE> "What is the puzzle to run. Valid values: 1 to 25").index(1),
            arg!(<PART> "Part of the puzzle to execute. Valid values: 1, 2").index(2),
            arg!(<FILEPATH> "Path to the file which contains the puzzle input").index(3),
        ]);
    let matches = cmd.get_matches();

    let file_path = matches.get_one::<String>("FILEPATH").unwrap();
    let input_lines = commons::read_file_to_lines(file_path);
    
    let puzzle_id: i32 = matches.get_one::<String>("PUZZLE").unwrap()
        .parse().unwrap();
    let puzzle = match puzzle_id {
        n @ 1..=25 => SOLUTIONS.get((n-1) as usize).unwrap(),
        _ => panic!("There are only 25 puzzles. Please choose one of them")
    };

    let part: i32 = matches.get_one::<String>("PART").unwrap()
        .parse().unwrap();
    let result = match part {
       1 => puzzle.part1(&input_lines),
       2 => puzzle.part2(&input_lines),
       _ => panic!("There are only two parts for a puzzle. Please choose one of them")
    };

    println!("Result for puzzle {} part {}: {}", puzzle_id, part, result);
}