use crate::utils::read_lines;

const START: u8 = 35;
const END: u8 = 21;

pub fn run() {
    part_two();
    part_one();
}

fn part_two() {}

fn part_one() {
    let mut grid: Vec<Vec<u8>> = vec![];
    read_lines("input/aoc12.txt")
        .expect("Error reading aoc12.txt")
        .for_each(|l| {
            let mut row: Vec<u8> = vec![];
            l.unwrap().chars().for_each(|c| {
                let height = c as u8 - 48;
                row.push(height);
            });
            grid.push(row);
        });

    grid.iter().for_each(|row| {
        row.iter().for_each(|c| print!("{} ", c));
        println!("");
    });
}
