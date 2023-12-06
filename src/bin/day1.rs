use ::phf::{phf_map, Map};
use aocd::prelude::*;
use lazy_regex::regex_replace_all;
use std::str::Lines;

const TEXT_TO_NUMBER: Map<&'static str, &'static str> = phf_map! {
    "one" => "o1e",
    "two" => "t2o",
    "three" => "t3e",
    "four" => "f4r",
    "five" => "f5e",
    "six" => "s6x",
    "seven" => "s7n",
    "eight" => "e8t",
    "nine" => "n9e",
};

fn get_replacement(textual_number: &str) -> String {
    TEXT_TO_NUMBER.get(textual_number).unwrap().to_string()
}

fn substitute_textual_numbers(str: &str) -> String {
    let mut previous_replaced_string = str.to_owned();

    loop {
        let replaced_string = regex_replace_all!(
            r"(one|two|three|four|five|six|seven|eight|nine)",
            &previous_replaced_string,
            |_, textual_numeric| { get_replacement(textual_numeric) }
        );

        if replaced_string == previous_replaced_string {
            break;
        }

        previous_replaced_string = replaced_string.to_string();
    }

    previous_replaced_string.to_string()
}

fn parse_numbers(input: &str, support_textual_numbers: bool) -> u32 {
    let processed_input = if support_textual_numbers {
        substitute_textual_numbers(input)
    } else {
        input.to_string()
    };

    let first_digit = processed_input
        .chars()
        .find(|ch: &char| ch.is_digit(10))
        .unwrap();
    let last_digit = processed_input
        .chars()
        .rev()
        .find(|ch: &char| ch.is_digit(10))
        .unwrap();

    format!("{}{}", first_digit, last_digit)
        .parse::<u32>()
        .unwrap()
}

fn run(input: Lines, textual_numbers: bool) -> u32 {
    input
        .map(|line| parse_numbers(line, textual_numbers))
        .sum::<u32>()
}

#[aocd(2023, 1)]
fn main() {
    let calibration_input = input!();

    let result_part1 = run(calibration_input.lines(), false);
    let result_part2 = run(calibration_input.lines(), true);

    submit!(1, result_part1);
    submit!(2, result_part2);
}
