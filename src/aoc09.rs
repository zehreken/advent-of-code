use std::num::Wrapping;

use crate::utils::read_lines;

pub fn run() {
    part_two();
    // part_one();
}

fn part_two() {
    let mut knots = vec![Vec2::new(); 10];
    let mut tail_positions = vec![];
    tail_positions.push(Vec2::new());
    read_lines("input/aoc09.txt")
        .expect("Error reading aoc09.txt")
        .for_each(|line| {
            let line = line.unwrap();
            let (direction, step) = line.split_once(' ').unwrap();
            let step = step.parse::<u16>().unwrap();

            for _ in 0..step {
                // Head always moves
                knots[0] = Vec2::sum(knots[0], move_one_step(direction));
                for knot_i in 1..knots.len() {
                    let diff = Vec2::diff(knots[knot_i - 1], knots[knot_i]);
                    let magnitude = diff.magnitude();
                    if magnitude > 2.1 {
                        // Diagonal
                        knots[knot_i] = Vec2::sum(
                            knots[knot_i],
                            Vec2 {
                                x: diff.x.signum(),
                                y: diff.y.signum(),
                            },
                        )
                    } else if magnitude > 1.5 {
                        // Axial
                        knots[knot_i] = Vec2::sum(
                            knots[knot_i],
                            Vec2 {
                                x: diff.x / 2,
                                y: diff.y / 2,
                            },
                        );
                    }
                }
                if !tail_positions
                    .iter()
                    .any(|e| e.x == knots[9].x && e.y == knots[9].y)
                {
                    tail_positions.push(knots[9]);
                }
            }
        });

    println!("num pos: {:?}", tail_positions.len());
}

fn part_one() {
    let mut head_position = Vec2::new();
    let mut tail_position = Vec2::new();
    let mut tail_positions = vec![];
    tail_positions.push(tail_position);
    read_lines("input/aoc09.txt")
        .expect("Error reading aoc09.txt")
        .for_each(|line| {
            let line = line.unwrap();
            let (direction, step) = line.split_once(' ').unwrap();
            let step = step.parse::<u16>().unwrap();
            // println!("line {}", line);
            for _ in 0..step {
                head_position = Vec2::sum(head_position, move_one_step(direction));
                let diff = Vec2::diff(head_position, tail_position);
                // println!("diff: {}", diff.magnitude());
                let magnitude = diff.magnitude();
                if magnitude > 2.1 {
                    // Diagonal
                    tail_position = Vec2::sum(
                        tail_position,
                        Vec2 {
                            x: diff.x.signum(),
                            y: diff.y.signum(),
                        },
                    )
                } else if magnitude > 1.5 {
                    // Axial
                    tail_position = Vec2::sum(
                        tail_position,
                        Vec2 {
                            x: diff.x / 2,
                            y: diff.y / 2,
                        },
                    );
                }
                // let hash = tail_position.get_hash();
                // println!(
                //     "diff: {:?}, head: {:?}, tail: {:?}",
                //     diff, head_position, tail_position
                // );
                if !tail_positions
                    .iter()
                    .any(|e| e.x == tail_position.x && e.y == tail_position.y)
                {
                    tail_positions.push(tail_position);
                }
            }
        });

    println!("head pos: {:?}", head_position);
    println!("tail pos: {:?}", tail_position);
    println!("num pos: {:?}", tail_positions.len());
}

fn move_one_step(direction: &str) -> Vec2 {
    let mut step = Vec2::new();
    if direction == "R" {
        step.x += 1;
    } else if direction == "L" {
        step.x -= 1;
    } else if direction == "D" {
        step.y -= 1;
    } else if direction == "U" {
        step.y += 1;
    }

    step
}

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn sum(a: Vec2, b: Vec2) -> Vec2 {
        Self {
            x: a.x + b.x,
            y: a.y + b.y,
        }
    }

    fn diff(a: Vec2, b: Vec2) -> Vec2 {
        Self {
            x: a.x - b.x,
            y: a.y - b.y,
        }
    }

    fn magnitude(&self) -> f32 {
        let x_f = self.x as f32;
        let y_f = self.y as f32;
        (x_f.powf(2.0) + y_f.powf(2.0)).sqrt()
    }

    // Colliding hash, fix it
    fn get_hash(&self) -> Wrapping<i32> {
        Wrapping((self.y * 397) ^ self.x)
    }
}
