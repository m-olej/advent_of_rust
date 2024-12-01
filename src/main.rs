use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufReader};

fn day1() -> io::Result<()> {
    // read input file
    let file_path: &str = "inputs/input_d1.txt";

    let file = File::open(file_path);

    let reader = BufReader::new(file.unwrap());

    let mut row_1: Vec<i32> = Vec::new();
    let mut row_2: Vec<i32> = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let line: Vec<&str> = line.split_whitespace().collect();

        row_1.push(line[0].parse::<i32>().unwrap());
        row_2.push(line[1].parse::<i32>().unwrap());
    }

    row_1.sort();
    row_2.sort();

    let mut sum: i32 = 0;

    for i in 0..row_1.len() {
        if row_1[i] > row_2[i] {
            sum += row_1[i] - row_2[i];
        } else {
            sum += row_2[i] - row_1[i];
        }
    }

    println!("answer: {}", sum);

    Ok(())
}

fn main() {
    let _ = day1();
}