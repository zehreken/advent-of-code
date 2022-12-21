use crate::utils::read_lines;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

const START: u8 = 35;
const END: u8 = 21;

pub fn run() {
    part_two();
    part_one();
}

fn part_two() {}

fn part_one() {
    let mut grid: Vec<Vec<u8>> = vec![];
    let mut row_index = 0;
    let mut column_count = 0;

    read_lines("input/aoc12.txt")
        .expect("Error reading aoc12.txt")
        .for_each(|l| {
            let mut row: Vec<u8> = vec![];
            l.unwrap().chars().for_each(|c| {
                let height = c as u8 - 48;
                row.push(height);
                column_count += 1;
            });
            grid.push(row);
            column_count = 0;
            row_index += 1;
        });

    grid.iter().for_each(|row| {
        row.iter().for_each(|node| {
            print!("{:?} ", node);
        });
        println!("");
    });

    let mut visited = vec![];
    let mut frontier = VecDeque::new();
    let root = Node {
        pos: Vec2 { row: 0, column: 0 },
        height: START,
        parent: None,
    };
    visited.push(root.pos);
    frontier.push_back(root.pos);
    let mut pos_to_node: HashMap<Vec2, Rc<Node>> = HashMap::new();
    pos_to_node.insert(Vec2 { row: 0, column: 0 }, Rc::new(root));
    let mut target: Option<Rc<Node>> = None;

    while !frontier.is_empty() {
        let current_pos = frontier.pop_front();
        if let Some(current_pos) = current_pos {
            let current_height = grid[current_pos.row as usize][current_pos.column as usize];
            if current_height == END {
                //
                println!("Reached the target!");
                target = Some(Rc::clone(&pos_to_node[&current_pos]));
                break;
            }
            let neighbours = get_neighbours(current_pos);
            for neighbour in neighbours {
                let current_node = Rc::clone(&pos_to_node[&current_pos]);
                let n_height = grid[neighbour.row as usize][neighbour.column as usize];
                if !visited.iter().any(|n| *n == neighbour) {
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
            }
        }
    }

    println!("{:?}", target.unwrap());
}

fn get_neighbours(pos: Vec2) -> Vec<Vec2> {
    // up, right, down, left
    let neighbour_coords: Vec<(i8, i8)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut neighbours = vec![];

    for (n_r, n_c) in neighbour_coords {
        if pos.row + n_r >= 0 && pos.row + n_r < 5 && pos.column + n_c >= 0 && pos.column + n_c < 8
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
    height: u8,
    parent: Option<Rc<Node>>,
}

#[derive(Debug, Clone, Copy, Eq, Hash)]
struct Vec2 {
    row: i8,
    column: i8,
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}
