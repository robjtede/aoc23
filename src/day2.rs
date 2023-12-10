use std::fs;

use regex::{Captures, Regex};

const INPUT_FILE_NAME: &str = concat!["./inputs/", env!("CARGO_BIN_NAME"), ".txt"];

const ASCII_DIGIT_BASE: u64 = 0x30;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    // const INPUT_FILE_NAME: &str = concat!["./inputs/", env!("CARGO_BIN_NAME"), "-test-a.txt"];

    let input = fs::read_to_string(INPUT_FILE_NAME).unwrap();

    // println!("part a = {sum}");
    // assert_eq!(sum, 56049, "regression from correct answer");
}

fn part_b() {
    // const INPUT_FILE_NAME: &str = concat!["./inputs/", env!("CARGO_BIN_NAME"), "-test-b.txt"];

    let input = fs::read_to_string(INPUT_FILE_NAME).unwrap();

    // println!("part b = {sum}");
    // assert_eq!(sum, 54530, "regression from correct answer");
}
