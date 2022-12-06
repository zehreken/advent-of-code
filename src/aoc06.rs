use std::collections::VecDeque;

use crate::utils::read_lines;

pub fn run() {
    part_two();
    // part_one();
}

fn part_two() {
    let mut lines = read_lines("input/aoc06.txt").expect("Error reading aoc06.txt");
    let message = lines.next().unwrap().unwrap();

    let bytes = message.as_bytes();
    // println!("message: {:?}", bytes);
    let mut marker: VecDeque<u8> = VecDeque::with_capacity(14);
    let mut marker_index: u8 = 0;
    for i in 0..bytes.len() {
        let byte = &bytes[i];
        if !marker.contains(byte) {
            marker.push_back(*byte);
            marker_index += 1;
        } else {
            loop {
                let garbage = marker.pop_front();
                marker_index -= 1;
                if garbage.unwrap() == *byte {
                    marker.push_back(*byte);
                    marker_index += 1;
                    break;
                }
            }
        }

        if marker.len() == 14 {
            println!("{}", i + 1);
            break;
        }
    }

    println!("marker {:?}", marker);
}

fn part_one() {
    let mut lines = read_lines("input/aoc06.txt").expect("Error reading aoc06.txt");
    let message = lines.next().unwrap().unwrap();

    let bytes = message.as_bytes();
    // println!("message: {:?}", bytes);
    let mut marker: VecDeque<u8> = VecDeque::with_capacity(4);
    let mut marker_index: u8 = 0;
    for i in 0..bytes.len() {
        let byte = &bytes[i];
        if !marker.contains(byte) {
            marker.push_back(*byte);
            marker_index += 1;
        } else {
            loop {
                let garbage = marker.pop_front();
                marker_index -= 1;
                if garbage.unwrap() == *byte {
                    marker.push_back(*byte);
                    marker_index += 1;
                    break;
                }
            }
        }

        if marker.len() == 4 {
            println!("{}", i + 1);
            break;
        }
    }

    println!("marker {:?}", marker);
}
