use std::collections::VecDeque;

use crate::utils::read_lines;

pub fn run() {
    part_two();
    part_one();
}

fn part_two() {}

fn part_one() {
    let mut root_dir = Directory {
        name: "/".to_owned(),
        parent: None,
        children: vec![],
        files: vec![],
    };
    let mut current_dir: &Directory = &root_dir;
    read_lines("input/aoc07.txt")
        .expect("Error reading aoc07.txt")
        .skip(1)
        .for_each(|line| {
            if let Ok(line_ok) = line {
                let mut split = line_ok.split(' ');
                let first = split.next();
                if let Some(first_ok) = first {
                    // This is a command
                    if first_ok == "$" {
                        let second = split.next();
                        if let Some(second_ok) = second {
                            if second_ok == "cd" {
                                // Navigate
                                println!("navigating...")
                            } else if second_ok == "ls" {
                                // Maybe I can skip this, 'ls' is only for humans anyways
                                println!("skipping ls...");
                            }
                        }
                    } else if first_ok == "dir" {
                        // Create Folder if does not exist
                        println!("creating new folder...");
                        let name = split.next().unwrap();
                        let dir = Directory {
                            name: name.to_owned(),
                            parent: Some(Box::new(&current_dir)),
                            children: vec![],
                            files: vec![],
                        };
                        current_dir.children.push(dir);
                    } else {
                        // Create File if does not exist
                    }
                }
            }
        });
}

#[derive(Debug)]
struct Directory<'a> {
    pub name: String,
    pub parent: Option<Box<&'a Directory<'a>>>,
    pub children: Vec<Directory<'a>>,
    pub files: Vec<File>,
}

#[derive(Debug)]
struct File {
    pub name: String,
    pub size: u32,
}
