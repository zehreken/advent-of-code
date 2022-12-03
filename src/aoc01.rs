use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut calorie_acc = 0;
    let mut calories: Vec<u32> = Vec::new();

    if let Ok(lines) = read_lines("input/aoc01.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    calories.push(calorie_acc);
                    calorie_acc = 0;
                } else {
                    calorie_acc += ip.parse::<u32>().unwrap();
                }
            }
        }
    }

    calories.sort();
    calories.reverse();
    println!("{}, {}, {}", calories[0], calories[1], calories[2]);
    println!("sum: {}", calories[0] + calories[1] + calories[2]);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
