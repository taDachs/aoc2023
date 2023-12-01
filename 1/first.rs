use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;

    let reader = io::BufReader::new(file);

    let solution: u32 = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect()
        })
        .map(|nums: Vec<u32>| 10 * nums.first().unwrap() + nums.last().unwrap())
        .sum();

    println!("Answer: {}", solution);

    Ok(())
}
