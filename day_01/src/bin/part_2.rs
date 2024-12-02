use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;
use std::hash::Hash;

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

fn freq_in_list<T: Hash + Eq>(list: Vec<T>) -> HashMap<T, usize> {
    let mut dest: HashMap<T, usize> = HashMap::new();
    for elem in list {
        *dest.entry(elem).or_default() += 1;
    }
    dest
}

fn main() -> Result<(), Box<dyn Error>> {
    let (list_1, list_2) = read_input("./input.txt")?;
    let map_1 = freq_in_list(list_1);
    let map_2 = freq_in_list(list_2);

    let result: u64 = map_1
        .iter()
        .map(|(number, frequence)| {
            number * *frequence as u64 * *map_2.get(&number).unwrap_or(&0) as u64
        })
        .sum();

    println!("result == {result}");
    Ok(())
}
