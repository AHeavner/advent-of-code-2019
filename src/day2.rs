use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn do_day2() {
    run_program(read_program);
}

fn read_program(reader: BufReader<File>) -> Vec<i32> {
    let mut contents = String::new();
    reader.read_to_string(&mut contents);
    let numbers: Vec<i32> = contents.split(", ").map(|s| i32::from_str(s).unwrap().collect());
    numbers
}

fn run_program(data: Vec<i32>) {
    let mut pointer = 0;
    let mut opcode = data[pointer];
    data[1] = 12;
    data[2] = 2;
    while opcode != 99 && pointer < data.len() {
        match opcode {
            1 => {
                data[pointer + 3] = data[pointer + 1] + data[pointer + 2];
            },
            2 => {
                data[pointer + 3] = data[pointer + 1] * data[pointer + 2];
            },
            _ => {
                println!("Day 2 bad opcode");
                break;
            },
        }
        pointer += 4;
    }
    println!("Day 2 data position 0: {}", data[0]);
}