use crate::utils::read_lines;

pub fn run() {
    part_two();
    // part_one();
}

fn part_two() {
    let mut screen: [[char; 40]; 6] = [['.'; 40]; 6];
    let mut x: i32 = 1;
    let mut cycle_count = 0;
    read_lines("input/aoc10.txt")
        .expect("Error reading aoc10.txt")
        .for_each(|line| {
            if let Ok(line) = line {
                if line == "noop" {
                    cycle_count += 1;
                    draw(&mut screen, &x, cycle_count);
                } else {
                    for i in 0..2 {
                        cycle_count += 1;
                        draw(&mut screen, &x, cycle_count);
                        if i == 1 {
                            let (_, value) = line.split_once(' ').unwrap();
                            x += value.parse::<i32>().unwrap();
                        }
                    }
                }
            }
        });
}

fn draw(screen: &mut [[char; 40]; 6], x: &i32, cycle_count: i32) {
    let pixel_index = (cycle_count - 1) % 40;
    let row = (cycle_count - 1) / 40;
    screen[row as usize][pixel_index as usize] = if pixel_index >= x - 1 && pixel_index <= x + 1 {
        '#'
    } else {
        '.'
    };

    println!(
        "c: {}, x: {}, p: {}, r: {}",
        cycle_count, x, pixel_index, row
    );
    if cycle_count % 240 == 0 {
        // if cycle_count < 10 {
        for row in screen {
            for pixel in row {
                print!("{}", pixel);
            }
            println!("");
        }
    }
}

fn wrap(value: i32, around: i32) -> i32 {
    if value < 0 {
        return value + around;
    } else if value == around {
        return value - around;
    } else {
        return value;
    }
}

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
