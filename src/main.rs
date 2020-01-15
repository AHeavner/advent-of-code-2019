use std::fs::File;
use std::env;

mod day1;

fn main() {
    let file = read_file();
    day1::do_day1(file);
}

fn read_file() -> File {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    file
}

