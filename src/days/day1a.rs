use std::fs;

// Read input1.txt, parse to 2 integer lists, sort them, calculate absolute difference of each element and sum them up
pub fn main() {
    let string = fs::read_to_string("./data/input1.txt").unwrap();
    let lines = string.lines().collect::<Vec<&str>>();

    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();

    for line in lines {
        let numbers = line
            .split_whitespace()
            .map(|entry| entry.parse().unwrap())
            .collect::<Vec<usize>>();
        list1.push(numbers[0]);
        list2.push(numbers[1]);
    }

    list1.sort();
    list2.sort();

    let result: usize = list1
        .iter()
        .zip(list2)
        .map(|tuple| tuple.0.abs_diff(tuple.1))
        .sum();

    println!("{}", result);
}
