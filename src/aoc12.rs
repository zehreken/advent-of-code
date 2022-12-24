use crate::utils::read_lines;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

const START: char = 'E';
const END: char = 'a';

pub fn run() {
    part_two();
    part_one();
}

fn part_two() {}

fn part_one() {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut row_count: i32 = 0;
    let mut column_count: i32 = 0;
    let mut start_pos = Vec2 { row: 0, column: 0 };

    read_lines("input/aoc12.txt")
        .expect("Error reading aoc12.txt")
        .for_each(|l| {
            let mut row: Vec<char> = vec![];
            column_count = 0;
            l.unwrap().chars().for_each(|c| {
                if c == START {
                    start_pos = Vec2 {
                        row: row_count,
                        column: column_count,
                    };
                }
                row.push(c);
                column_count += 1;
            });
            grid.push(row);
            row_count += 1;
        });

    let mut visited = vec![];
    let mut frontier = VecDeque::new();
    let root = Node {
        pos: Vec2 {
            row: start_pos.row,
            column: start_pos.column,
        },
        parent: None,
    };
    visited.push(root.pos);
    frontier.push_back(root.pos);
    let mut pos_to_node: HashMap<Vec2, Rc<Node>> = HashMap::new();
    pos_to_node.insert(start_pos, Rc::new(root));
    let mut target: Option<Rc<Node>> = None;

    while !frontier.is_empty() {
        let current_pos = frontier.pop_front();
        if let Some(current_pos) = current_pos {
            let current_height = grid[current_pos.row as usize][current_pos.column as usize];
            if current_height == END {
                println!("Reached the target!");
                // print_state(&grid, current_pos, &visited);
                target = Some(Rc::clone(&pos_to_node[&current_pos]));
                break;
            }
            let neighbours = get_neighbours(current_pos, row_count, column_count);
            for neighbour in neighbours {
                if !visited.iter().any(|n| *n == neighbour) {
                    let current_node = Rc::clone(&pos_to_node[&current_pos]);
                    let n_height = grid[neighbour.row as usize][neighbour.column as usize];
                    if current_height == START || current_height as i8 - n_height as i8 <= 1 {
                        let child = Node {
                            pos: Vec2 {
                                row: neighbour.row,
                                column: neighbour.column,
                            },
                            parent: Some(current_node),
                        };
                        visited.push(neighbour);
                        frontier.push_back(neighbour);
                        pos_to_node.insert(neighbour, Rc::new(child));
                    }
                    // print_state(&grid, current_pos, &visited);
                }
            }
        }
    }

    let mut path = vec![];
    let mut node_count = 0;
    let mut next = &target;
    'a: loop {
        if let Some(current) = next {
            path.push(current.pos);
            next = &current.parent;
            node_count += 1;
        } else {
            break 'a;
        }
    }
    print_path(&grid, &path);
    // Step count is 1 less than node count
    println!("{}", node_count);
}

fn print_path(grid: &Vec<Vec<char>>, path: &Vec<Vec2>) {
    grid.iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, height)| {
            if path
                .iter()
                .any(|n| n.row == r as i32 && n.column == c as i32)
            {
                print!("\x1b[93m{}\x1b[0m", height);
            } else {
                print!("{}", height);
            }
        });
        println!("");
    });
    println!("");
}

fn print_state(grid: &Vec<Vec<char>>, current_pos: Vec2, visited: &Vec<Vec2>) {
    grid.iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, height)| {
            if current_pos.row == r as i32 && current_pos.column == c as i32 {
                print!("_");
            } else if visited
                .iter()
                .any(|n| n.row == r as i32 && n.column == c as i32)
            {
                print!("*");
            } else {
                print!("{}", height);
            }
        });
        println!("");
    });
    println!("");
}

fn get_neighbours(pos: Vec2, row_count: i32, column_count: i32) -> Vec<Vec2> {
    // left, down, right, up
    let neighbour_coords: Vec<(i32, i32)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut neighbours = vec![];

    for (n_r, n_c) in neighbour_coords {
        if pos.row + n_r >= 0
            && pos.row + n_r < row_count
            && pos.column + n_c >= 0
            && pos.column + n_c < column_count
        {
            neighbours.push(Vec2 {
                row: pos.row + n_r,
                column: pos.column + n_c,
            });
        }
    }

    neighbours
}

#[derive(Debug)]
struct Node {
    pos: Vec2,
    parent: Option<Rc<Node>>,
}

#[derive(Debug, Clone, Copy, Eq, Hash)]
struct Vec2 {
    row: i32,
    column: i32,
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}
