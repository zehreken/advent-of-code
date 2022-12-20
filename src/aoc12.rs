use crate::utils::read_lines;
use std::collections::VecDeque;
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

    while !frontier.is_empty() {
        let current_pos = frontier.pop_front();
        if let Some(current_pos) = current_pos {
            let current_height = grid[current_pos.row as usize][current_pos.column as usize];
            if current_height == END {
                //
                break;
            }
            let neighbours = get_neighbours(current_pos);
            for neighbour in neighbours {
                let n_height = grid[neighbour.row as usize][neighbour.column as usize];
                if !visited.contains(neighbour) {}
            }
        }
    }

    // bfs(&grid, &grid[0][0]);
}

/*
fn bfs(grid: &Vec<Vec<Node>>, start: &Node) {
    let mut visited_nodes = vec![];
    visited_nodes.push(start.pos);
    let mut queue = VecDeque::new();
    queue.push_back(start.pos);
    let root = Rc::new(start);
    while !queue.is_empty() {
        let current = queue.pop_front();
        if let Some(current) = current {
            let current_node = &grid[current.row as usize][current.column as usize];
            if current_node.height == END {
                // return Rc::new(current_node);
            }
            let neighbours = get_neighbours(grid, start);
            for neighbour in neighbours {
                let node = &grid[neighbour.row as usize][neighbour.column as usize];
                if !visited_nodes.contains(&neighbour) {
                    let child = Node {
                        pos: node.pos,
                        height: node.height,
                        parent: Some(Rc::new(current_node)),
                    };
                    visited_nodes.push(neighbour);
                    queue.push_back(neighbour);
                }
            }
        }
    }
    // root
}
*/

fn get_neighbours(pos: Vec2) -> Vec<Vec2> {
    // up, right, down, left
    let neighbour_coords: Vec<(i8, i8)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut neighbours = vec![];

    for (n_r, n_c) in neighbour_coords {
        if pos.row + n_r >= 0 || pos.row + n_r < 5 && pos.column + n_c >= 0 || pos.column + n_c < 8
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

fn calculate_h_score((r1, c1): (u8, u8), (r2, c2): (u8, u8)) -> u8 {
    r1.abs_diff(r2) + c1.abs_diff(c2)
}

#[derive(Debug)]
struct Node {
    pos: Vec2,
    height: u8,
    parent: Option<Rc<Node>>,
}

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    row: i8,
    column: i8,
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}
