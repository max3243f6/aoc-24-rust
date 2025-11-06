use std::fs;

pub fn main() {
    let string = fs::read_to_string("./data/input1.txt").unwrap();
    let lines = string.lines().collect::<Vec<&str>>();

    
}