use crate::utils::read_lines;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

const START: i16 = 48;
const END: i16 = 75;

pub fn run() {
    part_two();
    part_one();
}

fn part_two() {}

fn part_one() {
    let mut grid: Vec<Vec<i16>> = vec![];
    let mut row_count: i32 = 0;
    let mut column_count: i32 = 0;
    let mut start_pos = Vec2 { row: 0, column: 0 };

    read_lines("input/aoc12.txt")
        .expect("Error reading aoc12.txt")
        .for_each(|l| {
            let mut row: Vec<i16> = vec![];
            column_count = 0;
            l.unwrap().chars().for_each(|c| {
                if c == 'S' {
                    start_pos = Vec2 {
                        row: row_count,
                        column: column_count,
                    };
                }
                let height = if c == 'S' {
                    48
                } else if c == 'E' {
                    75
                } else {
                    c as i16 - 48
                };
                row.push(height);
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
        height: START,
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
                //
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
                    if n_height - current_height <= 1 {
                        let child = Node {
                            pos: Vec2 {
                                row: neighbour.row,
                                column: neighbour.column,
                            },
                            height: n_height,
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
    let mut cache = &target;
    'a: loop {
        if let Some(current) = cache {
            path.push(current.pos);
            cache = &current.parent;
            node_count += 1;
        } else {
            break 'a;
        }
    }
    print_path(&grid, &path);
    // Step count is 1 less than node count
    println!("{}", node_count);

    // if let Some(target) = target {
    //     println!("{:?}", target);
    // } else {
    //     println!("Couldn't reach target!");
    // }
}

fn print_path(grid: &Vec<Vec<i16>>, path: &Vec<Vec2>) {
    grid.iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, height)| {
            if path
                .iter()
                .any(|n| n.row == r as i32 && n.column == c as i32)
            {
                print!("** ");
            } else {
                print!("{:?} ", height);
            }
        });
        println!("");
    });
    println!("");
}

fn print_state(grid: &Vec<Vec<i16>>, current_pos: Vec2, visited: &Vec<Vec2>) {
    grid.iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, height)| {
            if current_pos.row == r as i32 && current_pos.column == c as i32 {
                print!("{:?}_", height);
            } else if visited
                .iter()
                .any(|n| n.row == r as i32 && n.column == c as i32)
            {
                print!("** ");
            } else {
                print!("{:?} ", height);
            }
        });
        println!("");
    });
    println!("");
}

fn get_neighbours(pos: Vec2, row_count: i32, column_count: i32) -> Vec<Vec2> {
    // up, right, down, left
    let neighbour_coords: Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
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

fn _get_neighbours<'a>(grid: &'a Vec<Vec<Node>>, cell: &Node) -> Vec<Vec2> {
    // up, right, down, left
    let neighbour_coords: Vec<(i8, i8)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut neighbours = vec![];
    let row = cell.pos.row;
    let column = cell.pos.column;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            for (n_r, n_c) in &neighbour_coords {
                if row as i8 + n_r == r as i8 && column as i8 + n_c == c as i8 {
                    neighbours.push(grid[r][c].pos);
                }
            }
        }
    }

    neighbours
}

#[derive(Debug)]
struct Node {
    pos: Vec2,
    height: i16,
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
