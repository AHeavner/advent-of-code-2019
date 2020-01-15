use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn do_day1(file: File) {
    let reader = BufReader::new(file);
    let numbers = read_integers(reader);

    println!("Day 1 sum: {}", calc_1(&numbers));
    println!("Day 1 part 2 sum: {}", calc_2(&numbers));
}

fn calc_2(numbers: &Vec<i32>) -> i32 {
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

fn calc_1(numbers: &Vec<i32>) -> i32 {
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

fn read_integers(reader: BufReader<File>) -> Vec<i32> {
    let mut ints: Vec<i32> = Vec::new();
    for line in reader.lines() {
        ints.push(line.unwrap().parse::<i32>().unwrap());
    }
    ints
}