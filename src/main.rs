use clap::{Command, arg};

use crate::commons::Solution;

pub mod day01;
pub mod commons;
pub mod day02;

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
    println!("Result D1P1: {}", day01::Day01::part1(input_lines.clone()));
    println!("Result D1P2: {}", day01::Day01::part2(input_lines.clone()));
    println!("Result D2P1: {}", day02::Day02::part1(input_lines.clone()));
    println!("Result D2P2: {}", day02::Day02::part2(input_lines));
}