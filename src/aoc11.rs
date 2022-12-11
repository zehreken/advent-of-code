use crate::utils::read_lines;

pub fn run() {
    part_two();
    part_one();
}

fn part_two() {}

fn part_one() {
    let mut lines = vec![];
    lines = read_lines("input/aoc11.txt")
        .expect("Error reading aoc11.txt")
        .filter(|line| line.as_ref().unwrap() != "")
        .map(|l| l.unwrap())
        .collect();

    let mut monkeys = vec![];
    let mut monkey_items = vec![];
    let mut monkey_inspections = vec![];
    for i in (0..lines.len()).step_by(6) {
        let (monkey, items) = parse_monkey(&lines[i..(i + 6)]);
        monkeys.push(monkey);
        monkey_items.push(items);
        monkey_inspections.push(0);
    }

    for _ in 0..20 {
        play_round(&monkeys, &mut monkey_items, &mut monkey_inspections);
    }
    monkeys.iter().for_each(|m| println!("{:?}", m));
    println!("{:?}", monkey_items);
    monkey_inspections.sort();
    monkey_inspections.reverse();
    println!("{:?}", monkey_inspections);
    println!("{}", monkey_inspections[0] * monkey_inspections[1]);
}

fn play_round(
    monkeys: &Vec<Monkey>,
    monkey_items: &mut Vec<Vec<i32>>,
    monkey_inspections: &mut Vec<i32>,
) {
    for (i, monkey) in monkeys.iter().enumerate() {
        monkey_items[i].reverse();
        while let Some(item) = monkey_items[i].pop() {
            let new_item = monkey.inspect(item);
            let next_monkey_id = monkey.pick_monkey(new_item);
            // println!("{}", next_monkey_id);
            monkey_items[next_monkey_id as usize].push(new_item);
            monkey_inspections[i] += 1;
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: u8,
    operation: Operation,
    operator: i32,
    divisor: i32,
    true_monkey: u8,
    false_monkey: u8,
}

impl Monkey {
    fn inspect(&self, item: i32) -> i32 {
        let inspected_item: i32 = match self.operation {
            Operation::Add => item + self.operator,
            Operation::Multiply => item * self.operator,
            Operation::Square => item * item,
        };

        inspected_item / 3
    }

    fn pick_monkey(&self, item: i32) -> u8 {
        if item % self.divisor == 0 {
            return self.true_monkey;
        } else {
            return self.false_monkey;
        }
    }
}

fn parse_monkey(slice: &[String]) -> (Monkey, Vec<i32>) {
    // slice.iter().for_each(|l| println!("{}", l));
    let mut slice_iter = slice.iter();
    let id = parse_id(slice_iter.next().unwrap());
    let items = parse_items(slice_iter.next().unwrap());
    let (operation, operator) = parse_operation(slice_iter.next().unwrap());
    let divisor = parse_divisor(slice_iter.next().unwrap());
    let true_monkey = parse_true_monkey(slice_iter.next().unwrap());
    let false_monkey = parse_false_monkey(slice_iter.next().unwrap());

    (
        Monkey {
            id,
            operation,
            operator,
            divisor,
            true_monkey,
            false_monkey,
        },
        items,
    )
}

fn parse_id(s: &String) -> u8 {
    let id = s.split(' ').skip(1).next().unwrap().chars().next().unwrap() as u8 - 48;

    id
}

fn parse_items(s: &String) -> Vec<i32> {
    let items = s
        .split_once(':')
        .unwrap()
        .1
        .split(',')
        // .for_each(|item| println!("{}", item.trim()));
        .map(|item| item.trim().parse::<i32>().unwrap())
        .collect();

    items
}

fn parse_operation(s: &String) -> (Operation, i32) {
    let operation = s
        .split(' ')
        .skip(6)
        .next()
        .map(|c| {
            // println!("{}", c);
            if c == "+" {
                Operation::Add
            } else {
                Operation::Multiply
            }
        })
        .unwrap();
    let operator = s.split(' ').skip(7).next().unwrap().trim().parse();
    if let Ok(operator) = operator {
        return (operation, operator);
    } else {
        return (Operation::Square, 1);
    }
}

fn parse_divisor(s: &String) -> i32 {
    let divisor: i32 = s.split(' ').last().unwrap().trim().parse().unwrap();

    divisor
}

fn parse_true_monkey(s: &String) -> u8 {
    let true_monkey: u8 = s.split(' ').last().unwrap().trim().parse().unwrap();

    true_monkey
}

fn parse_false_monkey(s: &String) -> u8 {
    let false_monkey: u8 = s.split(' ').last().unwrap().trim().parse().unwrap();

    false_monkey
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
    Square,
}
