use std::io::{BufRead, BufReader};
use std::fs::File;

fn read_program(reader: BufReader<File>) -> Vec<i32> {
    let mut contents = String::new();
    reader.read_to_string(&mut contents);
    let numbers: Vec<i32> = contents.split(", ").map(|s| i32::from_str(s).unwrap().collect());
    numbers
}