use crate::utils::read_lines;

pub fn run() {
    println!("AoC 01 =*=*=*=*=*=*");
    try_two();
    // try_one();
}

pub fn try_two() {
    let mut calories = Vec::new();
    let mut calorie_acc = 0;
    read_lines("input/aoc01.txt")
        .expect("Error reading aoc01.txt")
        .for_each(|line| {
            if let Ok(ip) = line {
                if ip.is_empty() {
                    calories.push(calorie_acc);
                    calorie_acc = 0;
                } else {
                    calorie_acc += ip.parse::<u32>().unwrap();
                }
            }
        });

    calories.sort();
    calories.reverse();
    let max = calories.first().unwrap();
    println!("max {}", max);
    let sum_of_first_three: u32 = calories[0..3].iter().sum();
    println!("sum of first 3: {sum_of_first_three}")
}

pub fn try_one() {
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
