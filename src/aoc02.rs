use std::collections::HashMap;

use crate::utils::read_lines;

pub fn run() {
    println!("AoC 02 =*=*=*=*=*=*");

    part_two();
    // part_one();
}

fn part_two() {
    let sum = read_lines("input/aoc02.txt")
        .expect("Error reading aoc02")
        .map(|line| check_part_two(line.unwrap()))
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();
    println!("sum {sum}");
}

fn check_part_two(line: String) -> u32 {
    // A rock 1 , B paper 2, C scissors 3
    // X lose, Y draw, Z win
    // lose 0, draw 3, win 6
    let possible_outcomes: HashMap<&str, u32> = HashMap::from([
        ("A X", 0 + 3),
        ("A Y", 3 + 1),
        ("A Z", 6 + 2),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 0 + 2),
        ("C Y", 3 + 3),
        ("C Z", 6 + 1),
    ]);
    // println!("{}", possible_outcomes[line.as_str()]);
    possible_outcomes[line.as_str()]
}

fn part_one() {
    let sum = read_lines("input/aoc02.txt")
        .expect("Error reading aoc02")
        .map(|line| calculate_score(line.unwrap()))
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>();
    println!("sum {sum}");
}

fn calculate_score(line: String) -> u32 {
    // A rock 1 , B paper 2, C scissors 3
    // X rock, Y paper, Z scissors
    // lose 0, draw 3, win 6
    let possible_outcomes: HashMap<&str, u32> = HashMap::from([
        ("A X", 3 + 1),
        ("A Y", 6 + 2),
        ("A Z", 0 + 3),
        ("B X", 0 + 1),
        ("B Y", 3 + 2),
        ("B Z", 6 + 3),
        ("C X", 6 + 1),
        ("C Y", 0 + 2),
        ("C Z", 3 + 3),
    ]);
    // println!("{}", possible_outcomes[line.as_str()]);
    possible_outcomes[line.as_str()]
}
