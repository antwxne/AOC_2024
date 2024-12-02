use std::fs::read_to_string;
use std::{error::Error, iter::zip};

fn read_input(path: &'static str) -> Result<(Vec<u64>, Vec<u64>), Box<dyn Error>> {
    let mut list_1: Vec<u64> = Vec::new();
    let mut list_2: Vec<u64> = Vec::new();

    for mut line in read_to_string(path)?
        .lines()
        .map(|line| line.split_ascii_whitespace())
    {
        if let Some(elem) = line.next() {
            list_1.push(elem.parse()?);
        }
        if let Some(elem) = line.next() {
            list_2.push(elem.parse()?);
        }
    }
    Ok((list_1, list_2))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (mut list_1, mut list_2) = read_input("./input.txt")?;
    list_1.sort();
    list_2.sort();
    let result: u64 = zip(list_1, list_2)
        .map(|(a, b): (u64, u64)| a.abs_diff(b))
        .sum();
    println!("result == {result}");
    Ok(())
}
