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

    let sum = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| (c as u64) - ASCII_DIGIT_BASE)
        })
        .filter_map(|mut line| {
            let first = line.next()?;
            let last = line.next_back().unwrap_or(first);
            Some((first * 10) + last)
        })
        .sum::<u64>();

    println!("part a = {sum}");
    assert_eq!(sum, 56049, "regression from correct answer");
}

fn part_b() {
    // const INPUT_FILE_NAME: &str = concat!["./inputs/", env!("CARGO_BIN_NAME"), "-test-b.txt"];

    let input = fs::read_to_string(INPUT_FILE_NAME).unwrap();

    let mut sum = 0;

    for line in input.lines() {
        let mut line_rev = line.to_owned().as_bytes().to_owned();
        line_rev.reverse();
        let line_rev = String::from_utf8_lossy(&line_rev).into_owned();

        // println!("{line}");
        // println!("{line_rev}");

        let re = Regex::new("(zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let line = re.replace(line, |captures: &Captures<'_>| match &captures[1] {
            "zero" => "0",
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => unreachable!(),
        });

        let re = Regex::new("(orez|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
        let line_rev = re.replace(&line_rev, |captures: &Captures<'_>| match &captures[1] {
            "orez" => "0",
            "eno" => "1",
            "owt" => "2",
            "eerht" => "3",
            "ruof" => "4",
            "evif" => "5",
            "xis" => "6",
            "neves" => "7",
            "thgie" => "8",
            "enin" => "9",
            _ => unreachable!(),
        });

        let first = line
            .chars()
            .find(|c| c.is_ascii_digit())
            .map(|c| (c as u64) - ASCII_DIGIT_BASE)
            .unwrap();

        let last = line_rev
            .chars()
            .find(|c| c.is_ascii_digit())
            .map(|c| (c as u64) - ASCII_DIGIT_BASE)
            .unwrap();

        sum += (first * 10) + last
    }

    println!("part b = {sum}");
    assert_eq!(sum, 54530, "regression from correct answer");
}
