use std::io::BufReader;
use std::fs::File;
use std::env;

mod day1;

fn main() {
    let reader = read_file();
    let ints = day1::read_integers(reader);
    
    println!("Day 1 sum: {}", day1::calc_1(&ints));
    println!("Day 1 part 2 sum: {}", day1::calc_2(&ints));
}

fn read_file() -> BufReader<File> {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    reader
}

