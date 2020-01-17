use std::io::{BufReader, BufRead, Read};
use std::fs::File;

fn do_day3(filename: &String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines = read_data(reader);

}

fn read_data(reader: BufReader<File>) {
    let mut paths: Vec<Line> = Vec::new();
    for line in reader.lines() {
        let mut vectors: Vec<String> = Vec::new();
        let line = line.unwrap();
        vectors = line.split(",").map(|s| String::from(s)).collect();
    }
}

fn parse_vector(vector: String) -> Vector {
    let direction = &vector[..1];
    let distance = &vector[1..];
    match direction {
        "R" => {
            return distance.parse::<i32>().unwrap();
        },
        "L" => {
            return -distance.parse::<i32>().unwrap();
        },
        "U" => {

        },
        "D" => {

        }
    }
}

struct Vector {
    x: i32,
    y: i32,
}

struct Point {
    x: i32,
    y: i32,
}

struct Line {
    p1: Point,
    p2: Point,
}