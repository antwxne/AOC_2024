use std::fs::read_to_string;
use std::{error::Error, iter::zip};

fn read_input(path: &'static str) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
    let mut list_1: Vec<Vec<u64>> = Vec::new();

    for line in read_to_string(path)?.lines() {
        let mut new_level: Vec<u64> = Vec::new();
        for n in line.split_ascii_whitespace() {
            new_level.push(n.parse()?);
        }
        list_1.push(new_level);
    }
    Ok(list_1)
}

fn is_line_safe(line: &Vec<u64>) -> bool {
    const MIN_DIFF: u64 = 1;
    const MAX_DIFF: u64 = 3;

    if !line.is_sorted() && !line.is_sorted_by(|a, b| a > b) {
        return false;
    }

    for i in 0..line.len() - 1 {
        let diff = line[i].abs_diff(line[i + 1]);
        if diff < MIN_DIFF || diff > MAX_DIFF {
            return false;
        }
    }
    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let list_1 = read_input("./input.txt")?;

    let result = list_1
        .iter()
        .map(is_line_safe)
        .filter(|a| *a == true)
        .count();

    println!("result == {result}");
    Ok(())
}
