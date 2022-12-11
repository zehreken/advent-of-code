use crate::utils::read_lines;

pub fn run() {
    part_two();
    part_one();
}

fn part_two() {}

fn part_one() {
    let mut x: i32 = 1;
    let mut cycle_count = 0;
    let mut signals = vec![];
    read_lines("input/aoc10.txt")
        .expect("Error reading aoc10.txt")
        .for_each(|line| {
            if let Ok(line) = line {
                if line == "noop" {
                    cycle_count += 1;
                    let signal = get_signal_strength(x, cycle_count);
                    if let Some(signal) = signal {
                        signals.push(signal);
                    }
                } else {
                    for i in 0..2 {
                        cycle_count += 1;
                        let signal = get_signal_strength(x, cycle_count);
                        if let Some(signal) = signal {
                            signals.push(signal);
                        }
                        if i == 1 {
                            let (_, value) = line.split_once(' ').unwrap();
                            x += value.parse::<i32>().unwrap();
                        }
                    }
                }
            }
        });

    println!("signals sum: {}", signals.iter().sum::<i32>());
}

fn get_signal_strength(x: i32, cycle_count: u32) -> Option<i32> {
    if cycle_count < 20 {
        return None;
    }
    if (cycle_count - 20) % 40 == 0 {
        println!("{}, {}", x, cycle_count);
        return Some(x * cycle_count as i32);
    }
    None
}
