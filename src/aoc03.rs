use crate::utils::read_lines;

pub fn run() {
    part_two();
    // part_one();
}

fn part_two() {
    let mut lines = read_lines("input/aoc03.txt").expect("Error reading aoc03.txt");

    let mut sum = 0;
    'l: loop {
        for _ in 0..3 {
            let a = lines.next();
            let b = lines.next();
            let c = lines.next();
            if let None = a {
                break 'l;
            }
            let a_u = a.unwrap().unwrap();
            let a_two = a_u.as_str();
            let b_u = b.unwrap().unwrap();
            let b_two = b_u.as_str();
            let c_u = c.unwrap().unwrap();
            let c_two = c_u.as_str();
            for c in a_two.chars() {
                if b_two.contains(c) && c_two.contains(c) {
                    if c.is_lowercase() {
                        sum += c as u32 - 96;
                    } else {
                        sum += c as u32 - 38;
                    }
                    break;
                }
            }
        }
    }
    println!("sum: {}", sum);
}

fn part_one() {
    let mut sum = 0;
    read_lines("input/aoc03.txt")
        .expect("Error reading aoc03.txt")
        .for_each(|line| {
            if let Ok(r) = line {
                let contents = r.as_str();
                let half_len = contents.len() / 2;
                let first_half = &contents[..half_len];
                let second_half = &contents[half_len..];

                for c in first_half.chars() {
                    if second_half.contains(c) {
                        if c.is_lowercase() {
                            sum += c as u32 - 96;
                        } else {
                            sum += c as u32 - 38;
                        }
                        break;
                    }
                }
            }
        });

    println!("sum {}", sum);
}
