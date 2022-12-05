use crate::utils::read_lines;

pub fn run() {
    let mut stacks = Vec::new();
    stacks.push(vec!['R', 'P', 'C', 'D', 'B', 'G']);
    stacks.push(vec!['H', 'V', 'G']);
    stacks.push(vec!['N', 'S', 'Q', 'D', 'J', 'P', 'M']);
    stacks.push(vec!['P', 'S', 'L', 'G', 'D', 'C', 'N', 'M']);
    stacks.push(vec!['J', 'B', 'N', 'C', 'P', 'F', 'L', 'S']);
    stacks.push(vec!['Q', 'B', 'D', 'Z', 'V', 'G', 'T', 'S']);
    stacks.push(vec!['B', 'Z', 'M', 'H', 'F', 'T', 'Q']);
    stacks.push(vec!['C', 'M', 'D', 'B', 'F']);
    stacks.push(vec!['F', 'C', 'Q', 'G']);

    part_two();
    part_one(&mut stacks);
}

fn part_two() {}

fn part_one(stacks: &mut Vec<Vec<char>>) {
    read_lines("input/aoc05.txt")
        .expect("Error reading aoc05.txt")
        .for_each(|line| {
            if let Ok(line_ok) = line {
                let (count, from, to) = parse_command(line_ok);
                for _ in 0..count {
                    let temp = stacks[from - 1].pop();
                    stacks[to - 1].push(temp.unwrap());
                }
            }
        });

    let solution: Vec<_> = stacks.iter().map(|stack| stack.last()).collect();
    println!("");
    for c in solution {
        print!("{}", c.unwrap());
    }
    println!("");
}

fn parse_command(command: String) -> (usize, usize, usize) {
    let mut split = command.as_str().split(' ');
    split.next();
    let count = split.next().unwrap().parse::<usize>().unwrap();
    split.next();
    let from = split.next().unwrap().parse::<usize>().unwrap();
    split.next();
    let to = split.next().unwrap().parse::<usize>().unwrap();

    (count, from, to)
}
