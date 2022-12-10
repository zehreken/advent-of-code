use crate::utils::read_lines;
use std::{cell::RefCell, rc::Rc, vec};

pub fn run() {
    part_two();
    part_one();
}

fn part_two() {
    let root = part_one();
    const DISK_SPACE: u32 = 70_000_000;
    const EMPTY_SPACE_NEEDED: u32 = 30_000_000;
    let current_used_space = calculate_size(&root, &mut 0);
    let necessary_space = current_used_space - (DISK_SPACE - EMPTY_SPACE_NEEDED);
    println!("{}", necessary_space);
    let mut current_max = 0;
    calculate_size_for_necessary_space(&root, necessary_space, &mut current_max);
}

fn calculate_size_for_necessary_space(
    dir: &Rc<RefCell<Directory>>,
    necessary_space: u32,
    current_max: &mut u32,
) -> u32 {
    let mut dir_size = dir.borrow().get_size();
    let children = &dir.borrow().children;
    for d in children {
        dir_size += calculate_size_for_necessary_space(d, necessary_space, current_max);
    }

    if dir_size <= necessary_space + 30000 && dir_size > *current_max {
        *current_max = dir_size;
        println!("dir size: {}", dir_size);
    }

    dir_size
}

fn part_one() -> Rc<RefCell<Directory>> {
    let root = Rc::new(RefCell::new(Directory {
        name: "/".to_owned(),
        parent: None,
        children: vec![],
        files: vec![],
    }));
    let mut current_dir = Rc::clone(&root);
    read_lines("input/aoc07.txt")
        .expect("Error reading aoc07.txt")
        .skip(1)
        .for_each(|line| {
            let line = line.unwrap();
            let mut split = line.split(' ');
            let first = split.next();
            if let Some(first) = first {
                if first == "$" {
                    let command = split.next();
                    if let Some(command) = command {
                        if command == "cd" {
                            let path = split.next();
                            if let Some(path) = path {
                                if path == ".." {
                                    let current_clone = Rc::clone(&current_dir);
                                    current_dir =
                                        Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                                } else {
                                    let new_dir = Rc::new(RefCell::new(Directory {
                                        name: path.to_owned(),
                                        parent: Some(Rc::clone(&current_dir)),
                                        children: vec![],
                                        files: vec![],
                                    }));
                                    current_dir.borrow_mut().children.push(Rc::clone(&new_dir));
                                    current_dir = Rc::clone(&new_dir);
                                }
                            }
                        } else if command == "ls" {
                        }
                    }
                } else if first == "dir" {
                    let dir_name = split.next();
                    if let Some(dir_name) = dir_name {
                        // println!("Create dir");
                    }
                } else {
                    let file_name = split.next();
                    if let Some(file_name) = file_name {
                        let file = File {
                            name: file_name.to_owned(),
                            size: first.parse::<u32>().unwrap(),
                        };
                        current_dir.borrow_mut().files.push(file);
                    }
                }
            }
        });

    let mut sum_of_dirs_have_size_under_100k = 0;
    calculate_size(&root, &mut sum_of_dirs_have_size_under_100k);
    // let children: &Vec<Rc<RefCell<Directory>>> = &root.borrow().children;
    // for d in children.iter() {
    //     calculate_size(d);
    // }
    // println!("{}", sum_of_dirs_have_size_under_100k);
    root
}

fn calculate_size(dir: &Rc<RefCell<Directory>>, sum_of_dirs_have_size_under_100k: &mut u32) -> u32 {
    let mut dir_size = dir.borrow().get_size();
    let children = &dir.borrow().children;
    for d in children {
        dir_size += calculate_size(d, sum_of_dirs_have_size_under_100k);
    }

    let name = &dir.borrow().name;
    // println!("{} {}", name, dir_size);
    if dir_size < 100_000 {
        *sum_of_dirs_have_size_under_100k += dir_size;
    }

    dir_size
}

struct Directory {
    name: String,
    parent: Option<Rc<RefCell<Directory>>>,
    children: Vec<Rc<RefCell<Directory>>>,
    files: Vec<File>,
}

impl Directory {
    fn get_size(&self) -> u32 {
        self.files.iter().map(|f| f.size).sum()
    }
}

struct File {
    name: String,
    size: u32,
}
