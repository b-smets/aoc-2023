use ::phf::{phf_map, Map};
use aocd::prelude::*;
use lazy_regex::regex_replace_all;
use std::str::Lines;

const TEXT_TO_NUMBER: Map<&'static str, char> = phf_map! {
    "one" => '1',
    "two" => '2',
    "three" => '3',
    "four" => '4',
    "five" => '5',
    "six" => '6',
    "seven" => '7',
    "eight" => '8',
    "nine" => '9',
};

fn get_replacement(textual_number: &str) -> String {
    TEXT_TO_NUMBER.get(textual_number).unwrap().to_string()
}

fn substitute_textual_numbers(str: &str) -> String {
    regex_replace_all!(
        r"(one|two|three|four|five|six|seven|eight|nine)",
        str,
        |_, textual_numeric| get_replacement(textual_numeric)
    )
    .to_string()
}

fn is_numeric(ch: char) -> bool {
    return ch >= '0' && ch <= '9';
}

fn parse_numbers(input: &str, support_textual_numbers: bool) -> u32 {
    let processed_input = if support_textual_numbers {
        substitute_textual_numbers(input)
    } else {
        input.to_string()
    };

    let first_digit_index = processed_input.find(|c| is_numeric(c)).unwrap();
    let last_digit_index = processed_input.rfind(|c| is_numeric(c)).unwrap();

    let first_digit = processed_input.chars().nth(first_digit_index).unwrap();
    let last_digit = processed_input.chars().nth(last_digit_index).unwrap();

    let result = format!("{}{}", first_digit, last_digit)
        .parse::<u32>()
        .unwrap();

    println!("{} - {}{} - {}", input, first_digit, last_digit, result);
    result
}

fn run(input: Lines, textual_numbers: bool) -> u32 {
    input
        .map(|line| parse_numbers(line, textual_numbers))
        .reduce(|acc, current| {
            println!("Adding {} {} => {}", acc, current, acc + current);

            acc + current
        })
        .unwrap()
}

#[aocd(2023, 1)]
fn main() {
    let calibration_input = input!();

    // let result_part1 = run(calibration_input.lines(), false);
    let result_part2 = run(calibration_input.lines(), true);

    // submit!(1, result_part1);
    submit!(2, result_part2);
}
