use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn calc_2(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for number in numbers {
        let mut req = fuel_requirement(*number);
        while req >= 0 {
            sum += req;
            req = fuel_requirement(req);
        }
    }
    sum
}

pub fn calc_1(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for number in numbers {
        let temp = fuel_requirement(*number);
        sum += temp;
    }
    sum
}

fn fuel_requirement(number: i32) -> i32 {
    let mut number = number;
    number /= 3;
    number -= 2;
    number
}

pub fn read_integers(reader: BufReader<File>) -> Vec<i32> {
    let mut ints: Vec<i32> = Vec::new();
    for line in reader.lines() {
        ints.push(line.unwrap().parse::<i32>().unwrap());
    }
    ints
}