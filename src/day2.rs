use std::io::{BufReader, Read};
use std::fs::File;
use std::str::FromStr;

pub fn do_day2(filename: &String) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let data = read_program(reader);
    let output1 = run_program(data.clone(), 12, 2);
    println!("Part 1 data position 0: {}", output1[0]);

    'outer: for i in 0..100 {
        for j in 0..100 {
            let output = run_program(data.clone(), i, j);
            if output[0] == 19690720 {
                println!("Part 2 input 1: {} input 2: {} answer: {}", i, j, 100 * i + j);
                break 'outer;
            }
        }
    }
}

fn read_program(mut reader: BufReader<File>) -> Vec<i32> {
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let numbers: Vec<i32> = contents.split(",").map(|s| i32::from_str(s).unwrap()).collect(); 
    numbers
}

fn run_program(mut data: Vec<i32>, input1: i32, input2: i32) -> Vec<i32> {
    let mut pointer = 0;
    let mut opcode = data[pointer];
    data[1] = input1;
    data[2] = input2;
    while pointer < data.len() {
        match opcode {
            1 => {
                let p1 = data[pointer + 1];
                let p2 = data[pointer + 2];
                let source = data[p1 as usize] + data[p2 as usize];
                let dest = data[pointer + 3];
                data[dest as usize] = source;
            },
            2 => {
                let p1 = data[pointer + 1];
                let p2 = data[pointer + 2];
                let source = data[p1 as usize] * data[p2 as usize];
                let dest = data[pointer + 3];
                data[dest as usize] = source;
            },
            99 => {
                break;
            }
            _ => {
                println!("Bad opcode");
                break;
            },
        }
        pointer += 4;
        opcode = data[pointer];
    }
    data
}

#[test]
fn test1() {
    let input = vec![1,0,0,0,99];
    let output = vec![2,0,0,0,99];
    assert_eq!(run_program(input, 0 , 0), output);
}

#[test]
fn test2() {
    let input = vec![2,3,0,3,99];
    let output = vec![2,3,0,6,99];
    assert_eq!(run_program(input, 0, 0), output);
}

#[test]
fn test3() {
    let input = vec![2,4,4,5,99,0];
    let output = vec![2,4,4,5,99,9801];
    assert_eq!(run_program(input, 0, 0), output);
}

#[test]
fn test4() {
    let input = vec![1,1,1,4,99,5,6,0,99];
    let output = vec![30,1,1,4,2,5,6,0,99];
    assert_eq!(run_program(input, 0, 0), output);
}