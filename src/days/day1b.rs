use std::{collections::HashMap, fs};

pub fn main() {
    let file_string = fs::read_to_string("./data/input1.txt").unwrap();
    let lines = file_string.lines().collect::<Vec<_>>();

    let mut column1: Vec<usize> = Vec::new();
    let mut column2: Vec<usize> = Vec::new();
    
    for line in lines {
        let numbers = line
            .split_whitespace()
            .map(|entry| entry.parse().unwrap())
            .collect::<Vec<usize>>();
        column1.push(numbers[0]);
        column2.push(numbers[1]);
    }

    column2.sort();
    let mut occurance_map: HashMap<usize, usize> = HashMap::new();

    // This eats up the "column2" vector, because it is unborrowed by the for loop
    for x in column2 {
        if !occurance_map.contains_key(&x) {
            occurance_map.insert(x, 1);
        } else {
            *occurance_map.get_mut(&x).unwrap() += 1;
        }
    }

    // this can print the map, then still use it in the second for loop, because it is a reference and does not take ownership
    dbg!(&occurance_map);


    let mut checksum: usize = 0;

    // this takes ownership of "column1", 
    for x in column1 {
        checksum += x * occurance_map.get(&x).unwrap_or(&0);
    }

    println!("{}", checksum);
}