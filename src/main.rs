use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
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

    let mut sum_diff: i32 = 0;
    let mut sim_score: i32 = 0;
    let mut sim_map: HashMap<i32, i32> = HashMap::new();
    for i in 0..row_1.len() {
        // sum_diff
        if row_1[i] > row_2[i] {
            sum_diff += row_1[i] - row_2[i];
        } else {
            sum_diff += row_2[i] - row_1[i];
        }
        // Similarity_score
        if sim_map.contains_key(&row_1[i]) {
            sim_score += row_1[i] * sim_map.get(&row_1[i]).unwrap();
        } else {
            let key_count: i32 = row_2.iter().filter(|&n| *n == row_1[i]).count() as i32;
            sim_map.insert(row_1[i], key_count);
            sim_score += row_1[i] * key_count;
        }
        println!("sim: {sim_score}, element: {}", row_1[i]);
    }

    for (key, value) in &sim_map {
        println!("k: {key}, v: {value}");
    }

    println!("sum_diff: {}", sum_diff);
    println!("sim_score: {}", sim_score);

    Ok(())
}

fn main() {
    let _ = day1();
}

