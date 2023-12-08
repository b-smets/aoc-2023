use ::phf::phf_map;
use aocd::prelude::*;
use std::cmp;
use std::collections::HashMap;
use std::str::Lines;

#[derive(Debug)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Default for Hand {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

struct Game {
    id: u32,
    hands: Vec<Hand>,
}

#[derive(PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

static COLORS: phf::Map<&'static str, Color> = phf_map! {
    "red" => Color::Red,
    "green" => Color::Green,
    "blue" => Color::Blue,
};

fn parse_game(game_string: &str) -> Game {
    let game_split = game_string.split(':').collect::<Vec<_>>();
    let game_id = parse_game_id(game_split[0]);

    let parsed_hands: Vec<Hand> = game_split[1]
        .split(';')
        .map(|single_hand_str| parse_hand(single_hand_str))
        .collect();

    Game {
        id: game_id,
        hands: parsed_hands,
    }
}

fn parse_game_id(game_id_string: &str) -> u32 {
    game_id_string[5..].parse::<u32>().unwrap()
}

fn parse_hand(hand_string: &str) -> Hand {
    let hand_colors = hand_string
        .split(',')
        .map(|hand| parse_hand_color(hand))
        .collect::<HashMap<&Color, u32>>();

    let result = Hand {
        red: *hand_colors.get(&Color::Red).unwrap_or(&0),
        green: *hand_colors.get(&Color::Green).unwrap_or(&0),
        blue: *hand_colors.get(&Color::Blue).unwrap_or(&0),
    };

    result
}

fn parse_hand_color(hand_color_string: &str) -> (&Color, u32) {
    let segments = hand_color_string.trim().split(' ').collect::<Vec<_>>();

    let color = COLORS.get(segments[1]).expect("Invalid color");
    let amount = segments[0].parse::<u32>().unwrap();

    (color, amount)
}

fn find_biggest_hands_per_color<'a>(accumulator: &'a mut Hand, current: &Hand) -> &'a mut Hand {
    accumulator.red = cmp::max(accumulator.red, current.red);
    accumulator.green = cmp::max(accumulator.green, current.green);
    accumulator.blue = cmp::max(accumulator.blue, current.blue);

    accumulator
}

fn is_game_possible(game: &Game, constraints: &Hand) -> bool {
    let mut maximum_drawn_per_color = Hand::default();
    game.hands
        .iter()
        .fold(&mut maximum_drawn_per_color, find_biggest_hands_per_color);

    maximum_drawn_per_color.red <= constraints.red
        && maximum_drawn_per_color.green <= constraints.green
        && maximum_drawn_per_color.blue <= constraints.blue
}

fn determine_game_power(game: &Game) -> u32 {
    let mut maximum_drawn_per_color = Hand::default();
    game.hands
        .iter()
        .fold(&mut maximum_drawn_per_color, find_biggest_hands_per_color);

    maximum_drawn_per_color.red * maximum_drawn_per_color.green * maximum_drawn_per_color.blue
}

fn run_part1(input: Lines, constraints: &Hand) -> u32 {
    input
        .map(|line| parse_game(line))
        .filter(|game| is_game_possible(game, constraints))
        .map(|game| game.id)
        .sum()
}

fn run_part2(input: Lines) -> u32 {
    input
        .map(|line| parse_game(line))
        .map(|game| determine_game_power(&game))
        .sum()
}

#[aocd(2023, 2)]
fn main() {
    let constraints = Hand {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games_input = input!();
    let result_part1 = run_part1(games_input.lines(), &constraints);
    let result_part2 = run_part2(games_input.lines());

    submit!(1, result_part1);
    submit!(2, result_part2);
}
