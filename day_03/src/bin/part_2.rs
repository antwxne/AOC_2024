extern crate regex;

use regex::Regex;

use std::error::Error;
use std::fs::read_to_string;

fn parser(path: &'static str) -> Result<Vec<(i64, i64)>, Box<dyn Error>> {
    let content = read_to_string(path)?;
    let clean_regex = Regex::new(r"(?s)don't\(\).*?(?:do\(\)|$)")?;
    let cleaned_content = clean_regex.replace_all(&content, "");
    let regex: Regex = Regex::new(r"(?m)mul\((\d+),(\d+)\)")?;
    let mut dest: Vec<(i64, i64)> = Vec::new();

    for (_, [n1, n2]) in regex.captures_iter(&cleaned_content).map(|c| c.extract()) {
        dest.push((n1.parse()?, n2.parse()?));
    }
    Ok(dest)
}

fn main() -> Result<(), Box<dyn Error>> {
    let result: i64 = parser("./input.txt")?.iter().map(|(n1, n2)| n1 * n2).sum();
    println!("result == {result}");
    Ok(())
}
