use crate::utils::read_lines;

pub fn run() {
    part_two();
    // part_one();
}

fn part_two() {
    let mut grid: Vec<Vec<u8>> = vec![];
    read_lines("input/aoc08.txt")
        .expect("Error reading aoc08.txt")
        .for_each(|l| {
            let mut row: Vec<u8> = vec![];
            l.unwrap().chars().for_each(|c| {
                let height = c as u8 - 48;
                row.push(height);
            });
            grid.push(row);
        });

    let mut scenic_score = 0;
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            let sc = calculate_scenic_score(&grid, row, column);
            if sc > scenic_score {
                scenic_score = sc;
            }
        }
    }
    println!("top scenic score: {}", scenic_score);
}

fn calculate_scenic_score(grid: &Vec<Vec<u8>>, row: usize, column: usize) -> u32 {
    let current_height = grid[row][column];
    // From top to current row
    let mut scenic_score_top = 0;
    for r in 0..row {
        let r = (row - 1) - r;
        scenic_score_top += 1;
        if grid[r][column] >= current_height {
            break;
        }
    }
    // From current row to bottom
    let mut scenic_score_bottom = 0;
    for r in (row + 1)..grid.len() {
        scenic_score_bottom += 1;
        if grid[r][column] >= current_height {
            break;
        }
    }
    // From left to current column
    let mut scenic_score_left = 0;
    for c in 0..column {
        let c = (column - 1) - c;
        scenic_score_left += 1;
        if grid[row][c] >= current_height {
            break;
        }
    }
    // From current column to right
    let mut scenic_score_right = 0;
    for c in (column + 1)..grid[0].len() {
        scenic_score_right += 1;
        if grid[row][c] >= current_height {
            break;
        }
    }

    scenic_score_top * scenic_score_bottom * scenic_score_left * scenic_score_right
}

fn part_one() {
    let mut grid: Vec<Vec<u8>> = vec![];
    read_lines("input/aoc08.txt")
        .expect("Error reading aoc08.txt")
        .for_each(|l| {
            let mut row: Vec<u8> = vec![];
            l.unwrap().chars().for_each(|c| {
                let height = c as u8 - 48;
                row.push(height);
            });
            grid.push(row);
        });

    let mut visible_count = 0;
    for row in 0..grid.len() {
        for column in 0..grid[row].len() {
            if is_invisible(&grid, row, column) {
                print!("x");
            } else {
                visible_count += 1;
                print!("v");
            }
        }
        println!("");
    }

    println!("visible count: {}", visible_count);
}

fn is_invisible(grid: &Vec<Vec<u8>>, row: usize, column: usize) -> bool {
    if row == 0 || column == 0 || row == grid.len() - 1 || column == grid[0].len() - 1 {
        return false;
    }

    let current_height = grid[row][column];
    // From top to current row
    let mut invisible_from_top = false;
    for r in 0..row {
        if grid[r][column] >= current_height {
            invisible_from_top = true;
        }
    }
    // From current row to bottom
    let mut invisible_from_bottom = false;
    for r in (row + 1)..grid.len() {
        if grid[r][column] >= current_height {
            invisible_from_bottom = true;
        }
    }
    // From left to current column
    let mut invisible_from_left = false;
    for c in 0..column {
        if grid[row][c] >= current_height {
            invisible_from_left = true;
        }
    }
    // From current column to right
    let mut invisible_from_right = false;
    for c in (column + 1)..grid[0].len() {
        if grid[row][c] >= current_height {
            invisible_from_right = true;
        }
    }
    invisible_from_top && invisible_from_bottom && invisible_from_left && invisible_from_right
}
