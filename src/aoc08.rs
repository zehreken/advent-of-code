use crate::utils::read_lines;

pub fn run() {
    part_two();
    part_one();
}

fn part_two() {}

fn part_one() {
    let mut grid: Vec<Vec<u8>> = vec![];
    read_lines("input/aoc08.txt")
        .expect("Error reading aoc08.txt")
        .for_each(|l| {
            let mut row: Vec<u8> = vec![];
            l.unwrap().chars().for_each(|c| {
                let height = c as u8 - 48;
                row.push(height);
            });
            grid.push(row);
        });

    for row in grid {
        println!("row");
        for v in row {
            print!("{} ", v);
        }
        println!("");
    }
}
