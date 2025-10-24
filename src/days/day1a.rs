use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let reader = BufReader::new(
        File::open("./data/input1.txt").expect("Something went wrong reading the file"),
    );
    let input_array: Vec<Vec<usize>> = reader
        .lines()
        // für jede zeile
        .map(|line| {
            line
                .unwrap()
                .split_whitespace()
                //für jede nummer in der zeile
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();
    for (i, line) in input_array.iter().enumerate() {
        list1[i] = line[0];
        list2[i] = line[1];
    }
    list1.sort();
    list2.sort();
    let mut result = 0;
    for i in 0..list1.len() {
        result += (list1[i] as isize - list2[i] as isize).abs();
    }

    println!("{}", result);
}
