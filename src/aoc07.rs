use std::collections::HashMap;

use crate::utils::read_lines;

pub fn run() {
    part_two();
    part_one();
}

fn part_two() {}

fn part_one() {
    let mut parent_to_children_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut child_to_parent_map: HashMap<String, String> = HashMap::new();
    let mut directory_to_files_map: HashMap<String, Vec<File>> = HashMap::new();

    // Adding root dir
    parent_to_children_map.insert("/".to_owned(), vec![]);
    // Setting current dir to root
    let mut current_dir: String = "/".to_owned();

    read_lines("input/aoc07.txt")
        .expect("Error reading aoc07.txt")
        .for_each(|line| {
            if let Ok(line) = line {
                // println!("{}", line);
                process_line(
                    line,
                    &mut current_dir,
                    &mut parent_to_children_map,
                    &mut child_to_parent_map,
                    &mut directory_to_files_map,
                );
            }
        });

    println!("========================");
    for (parent, children) in parent_to_children_map {
        let mut dir_size: u32 = 0;
        for child in children {
            for file in &directory_to_files_map[&child] {
                dir_size += file.size;
            }
        }
        println!("{}, {}", parent, dir_size);
    }
}

fn process_line(
    line: String,
    current_dir: &mut String,
    parent_to_children_map: &mut HashMap<String, Vec<String>>,
    child_to_parent_map: &mut HashMap<String, String>,
    directory_to_files_map: &mut HashMap<String, Vec<File>>,
) {
    let mut split = line.split(' ');
    let first = split.next();
    if let Some(first) = first {
        if first == "$" {
            let second = split.next();
            if let Some(second) = second {
                if second == "cd" {
                    let third = split.next();
                    if let Some(third) = third {
                        if third == ".." {
                            println!("Go one level up");
                            *current_dir = child_to_parent_map[current_dir].to_owned();
                        } else {
                            println!("Going in {}", third);
                            *current_dir = third.to_owned();
                        }
                    }
                } else if second == "ls" {
                    println!("Skipping ls, this is only for humans anyways");
                }
            }
        } else if first == "dir" {
            let second = split.next();
            if let Some(second) = second {
                println!("Create dir");
                if !parent_to_children_map.contains_key(current_dir) {
                    parent_to_children_map.insert(current_dir.to_owned(), vec![]);
                }
                parent_to_children_map
                    .get_mut(current_dir)
                    .unwrap()
                    .push(second.to_owned());
                if !child_to_parent_map.contains_key(second) {
                    child_to_parent_map.insert(second.to_owned(), current_dir.to_owned());
                }
            }
        } else {
            let second = split.next();
            if let Some(second) = second {
                if !directory_to_files_map.contains_key(current_dir) {
                    directory_to_files_map.insert(current_dir.to_owned(), vec![]);
                }
                directory_to_files_map
                    .get_mut(current_dir)
                    .unwrap()
                    .push(File {
                        size: first.parse::<u32>().unwrap(),
                        name: second.to_owned(),
                    })
            }
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}
