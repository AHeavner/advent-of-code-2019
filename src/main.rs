use std::io::{BufRead, BufReader};
use std::fs::File;
use std::env;

fn main() {
    let reader = read_file();
    let ints = read_integers(reader);
    let day1_sum = calc_day1(ints);
    
    println!("Day 1 sum: {}", day1_sum);
}

fn calc_day2(numbers: Vec<i32>) -> i32 {
    let mut sum = 0;
    for number in numbers {
        
    }
}

fn calc_day1(numbers: Vec<i32>) -> i32 {
    let mut sum = 0;
    for number in numbers {
        let mut temp = number;
        temp /= 3;
        temp -= 2;
        sum += temp;
    }
    sum
}

fn read_file() -> BufReader<File> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    reader
}

fn read_integers(reader: BufReader<File>) -> Vec<i32> {
    let mut ints: Vec<i32> = Vec::new();
    for line in reader.lines() {
        ints.push(line.unwrap().parse::<i32>().unwrap());
    }
    ints
}