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

    let mut line_index = 0;
    read_lines("input/aoc07.txt")
        .expect("Error reading aoc07.txt")
        .for_each(|line| {
            if let Ok(line) = line {
                println!("{}: {}", line_index, line);
                process_line(
                    line,
                    &mut current_dir,
                    &mut parent_to_children_map,
                    &mut child_to_parent_map,
                    &mut directory_to_files_map,
                );
                line_index += 1;
            }
        });

    println!("========================");
    let mut directory_to_size_map:HashMap<String, u32> = HashMap::new();
    for (directory, _) in &directory_to_files_map {
        let size = calculate_size(&directory, &parent_to_children_map, &directory_to_files_map);
        if !directory_to_size_map.contains_key(directory) {
            directory_to_size_map.insert(directory.to_owned(), size);
        }
        println!("directory {} is {}", directory, size);
    }

}

fn calculate_size(
    directory: &String,
    parent_to_children_map: &HashMap<String, Vec<String>>,
    directory_to_files_map: &HashMap<String, Vec<File>>,
) -> u32 {
    let mut dir_size = 0;
    for file in &directory_to_files_map[directory] {
        dir_size += file.size;
    }
    if parent_to_children_map.contains_key(directory) {
        let children = parent_to_children_map.get(directory).unwrap();
        for child in children {
            dir_size += calculate_size(child, parent_to_children_map, directory_to_files_map)
        }
    }
    // println!("directory {} is {}", directory, dir_size);
    dir_size
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
                            println!("Go one level up {} current {}", child_to_parent_map[current_dir], current_dir);
                            *current_dir = child_to_parent_map[current_dir].to_owned();
                        } else {
                            println!("Going in {}, current {}", third, current_dir);
                            *current_dir = third.to_owned();
                        }
                    }
                } else if second == "ls" {
                    // println!("Skipping ls, this is only for humans anyways");
                }
            }
        } else if first == "dir" {
            let dir_name = split.next();
            if let Some(dir_name) = dir_name {
                println!("Create dir");
                if !parent_to_children_map.contains_key(current_dir) {
                    parent_to_children_map.insert(current_dir.to_owned(), vec![]);
                }
                parent_to_children_map
                    .get_mut(current_dir)
                    .unwrap()
                    .push(dir_name.to_owned());
                if !child_to_parent_map.contains_key(dir_name) {
                    child_to_parent_map.insert(dir_name.to_owned(), current_dir.to_owned());
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
