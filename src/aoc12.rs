use crate::utils::read_lines;

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
    let mut frontier_cells: Vec<&mut Cell> = vec![];
    frontier_cells.push(&mut grid[0][0]);

    grid.iter().for_each(|row| {
        row.iter().for_each(|cell| {
            println!("{:?}", get_neighbours(&grid, &cell));
            print!("{:?} ", cell.height);
        });
    });
}

fn get_neighbours<'a>(grid: &'a Vec<Vec<Cell>>, cell: &'a Cell) -> Vec<&'a Cell> {
    // up, right, down, left
    let neighbour_coords: Vec<(i8, i8)> = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut neighbours = vec![];
    let row = cell.r;
    let column = cell.c;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            for (n_r, n_c) in &neighbour_coords {
                if grid[i][j].r as i8 + n_r >= 0
                    && grid[i][j].r as i8 + n_r < grid.len() as i8
                    && grid[i][j].c as i8 + n_c >= 0
                    && grid[i][j].c as i8 + n_c < grid[i].len() as i8
                {
                    neighbours.push(&grid[i][j]);
                }
            }
        }
    }

    neighbours
}

fn check_frontier_cells(frontier_cells: &mut Vec<&mut Cell>, visited_cells: &mut Vec<&Cell>) {}

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
