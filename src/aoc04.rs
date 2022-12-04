use crate::utils::read_lines;

pub fn run() {
    part_two();
    part_one();
}

fn part_two() {
    let mut occurrence_count = 0;
    read_lines("input/aoc04.txt")
        .expect("Error reading aoc04.txt")
        .for_each(|line| {
            if let Ok(l) = line {
                let r = l.as_str();
                let mut split = r.split(',');
                let a = split.next();
                let b = split.next();
                if let Some(a_n) = a {
                    if let Some(b_n) = b {
                        if check_overlap(parse_range(a_n), parse_range(b_n)) {
                            occurrence_count += 1;
                        }
                    }
                }
            }
        });

    println!("occurrences: {occurrence_count}");
}

// Returns true if there is an overlap between the two ranges
fn check_overlap(a: (u32, u32), b: (u32, u32)) -> bool {
    if a.1 >= b.0 && b.1 >= a.0 {
        return true;
    }
    false
}

fn part_one() {
    let mut occurrence_count = 0;
    read_lines("input/aoc04.txt")
        .expect("Error reading aoc04.txt")
        .for_each(|line| {
            if let Ok(l) = line {
                let r = l.as_str();
                let mut split = r.split(',');
                let a = split.next();
                let b = split.next();
                if let Some(a_n) = a {
                    if let Some(b_n) = b {
                        if check_contains(parse_range(a_n), parse_range(b_n)) {
                            occurrence_count += 1;
                        }
                    }
                }
            }
        });

    println!("occurrences: {occurrence_count}");
}

fn parse_range(str: &str) -> (u32, u32) {
    let mut split = str.split('-');
    let low = split.next().unwrap();
    let high = split.next().unwrap();
    (low.parse::<u32>().unwrap(), high.parse::<u32>().unwrap())
}

// Returns true if a range is contained by the other
fn check_contains(a: (u32, u32), b: (u32, u32)) -> bool {
    if a.0 <= b.0 && a.1 >= b.1 {
        return true;
    } else if b.0 <= a.0 && b.1 >= a.1 {
        return true;
    }
    false
}
