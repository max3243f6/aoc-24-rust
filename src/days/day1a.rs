use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};


pub fn main() {
    let reader = BufReader::new(File::open("./data/input1.txt").expect("Something went wrong reading the file"));
    let mut num_array: Vec<Vec<usize>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    
}