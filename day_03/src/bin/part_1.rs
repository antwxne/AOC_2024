extern crate regex;

use regex::Regex;

use std::error::Error;
use std::fs::read_to_string;

fn parser(path: &'static str) -> Result<Vec<(i64, i64)>, Box<dyn Error>> {
    let regex = Regex::new(r"(?m)mul\((\d+),(\d+)\)")?;
    let content = read_to_string(path)?;
    let mut dest: Vec<(i64, i64)> = Vec::new();

    for (_, [n1, n2]) in regex.captures_iter(&content.as_str()).map(|c| c.extract()) {
        dest.push((n1.parse()?, n2.parse()?));
    }
    Ok(dest)
}

fn main() -> Result<(), Box<dyn Error>> {
    let result: i64 = parser("./input.txt")?.iter().map(|(n1, n2)| n1 * n2).sum();
    println!("result == {result}");
    Ok(())
}
