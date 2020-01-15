use std::env;

mod day1;
mod day2;

fn main() {
    select_day();
}

fn select_day() {
    let args: Vec<String> = env::args().collect();
    match &args[1].parse::<i32>().unwrap() {
        1 => day1::do_day1(&args[2]),
        2 => day2::do_day2(&args[2]),
        _ => (),
    }
}