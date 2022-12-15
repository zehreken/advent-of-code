use crate::utils::read_lines;
use std::collections::VecDeque;

const START: u8 = 35;
const END: u8 = 21;

pub fn run() {
    part_two();
    part_one();
}

fn part_two() {}

fn part_one() {
    let mut grid: Vec<Vec<Cell>> = vec![];
    let mut row_count = 0;
    let mut column_count = 0;
    read_lines("input/aoc12.txt")
        .expect("Error reading aoc12.txt")
        .for_each(|l| {
            let mut row: Vec<Cell> = vec![];
            l.unwrap().chars().for_each(|c| {
                let height = c as u8 - 48;
                let cell = Cell {
                    r: row_count,
                    c: column_count,
                    height,
                    h_score: calculate_h_score((row_count, column_count), (3, 6)),
                    g_score: 0,
                };
                row.push(cell);
                column_count += 1;
            });
            grid.push(row);
            column_count = 0;
            row_count += 1;
        });

    grid.iter().for_each(|row| {
        row.iter().for_each(|cell| print!("{:?} ", cell.height));
        println!("");
    });

    println!("");

    grid.iter().for_each(|row| {
        row.iter().for_each(|cell| print!("{:?} ", cell.h_score));
        println!("");
    });

    let mut visited_cells: Vec<&Cell> = vec![];
    let mut frontier_cells: VecDeque<&Cell> = VecDeque::new();
    frontier_cells.push_front(&grid[0][0]);

    get_neighbours(&grid, &grid[1][2])
        .iter()
        .for_each(|c| println!("{:?}", c));

    check_frontier_cells(&grid, &mut frontier_cells, &mut visited_cells);
}

fn get_neighbours<'a, 'b>(grid: &'a Vec<Vec<Cell>>, cell: &'b Cell) -> Vec<&'a Cell> {
    // up, right, down, left
    let neighbour_coords: Vec<(i8, i8)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut neighbours = vec![];
    let row = cell.r;
    let column = cell.c;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            for (n_r, n_c) in &neighbour_coords {
                if row as i8 + n_r == r as i8 && column as i8 + n_c == c as i8 {
                    neighbours.push(&grid[r][c]);
                }
            }
        }
    }

    neighbours
}

fn check_frontier_cells<'a>(
    grid: &'a Vec<Vec<Cell>>,
    frontier_cells: &mut VecDeque<&'a Cell>,
    visited_cells: &mut Vec<&Cell>,
) {
    for cell in frontier_cells {
        let neighbours = get_neighbours(grid, cell);
        for neighbour in neighbours {
            if cell.height <= neighbour.height {
                // frontier_cells.push_front(neighbour);
            }
        }
        // frontier_cells.pop_back();
    }
    // frontier_cells.pop_back();
}

fn calculate_h_score((r1, c1): (u8, u8), (r2, c2): (u8, u8)) -> u8 {
    r1.abs_diff(r2) + c1.abs_diff(c2)
}

#[derive(Debug)]
struct Cell {
    r: u8,
    c: u8,
    height: u8,
    h_score: u8,
    g_score: u8,
}

impl Cell {
    fn get_f_score(&self) -> u8 {
        self.h_score + self.g_score
    }
}
