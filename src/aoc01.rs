use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let mut elf_to_calorie_map: HashMap<u32, u32> = HashMap::new();
    let mut elf_index = 0;
    let mut calorie_acc = 0;
    let mut max_calories = 0;

    if let Ok(lines) = read_lines("input/aoc01.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    elf_index += 1;
                    if calorie_acc > max_calories {
                        max_calories = calorie_acc;
                    }
                    calorie_acc = 0;
                } else {
                    // println!("{:?}", ip.parse::<u32>());
                    // elf_to_calorie_map.insert(elf_index, 0);
                    calorie_acc += ip.parse::<u32>().unwrap();
                }
            }
        }
    }

    println!("Number of elves: {}", elf_index);
    println!("Max calorie: {}", max_calories);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
