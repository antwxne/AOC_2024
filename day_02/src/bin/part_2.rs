use std::error::Error;
use std::fs::read_to_string;

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

fn is_line_safe_with_dampener(line: &Vec<u64>, index_to_remove: usize) -> bool {
    let mut new_vec: Vec<u64> = line.clone();
    new_vec.remove(index_to_remove);
    is_line_safe(&new_vec)
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

fn is_line_safe_part2(line: &Vec<u64>) -> bool {
    if is_line_safe(line) {
        return true;
    }

    line.iter()
        .enumerate()
        .map(|(index, _)| is_line_safe_with_dampener(line, index))
        .filter(|a| *a == true)
        .count()
        > 0
}

fn main() -> Result<(), Box<dyn Error>> {
    let list_1 = read_input("./input.txt")?;

    let result = list_1
        .iter()
        .map(is_line_safe_part2)
        .filter(|a| *a == true)
        .count();

    println!("result == {result}");
    Ok(())
}
