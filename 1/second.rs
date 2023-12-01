use std::fs::File;
use std::io::{self, BufRead};

const NUMBERS: [&str; 9] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        let mut nums = vec!();
        for (i, char) in line.chars().enumerate() {
            if let Some(digit) = char.to_digit(10) {
                nums.push(digit);
            } else {
                let substr = &line[i..];
                for (i, num_str) in NUMBERS.iter().enumerate() {
                    if substr.starts_with(num_str) {
                        nums.push((i + 1) as u32);
                    } 
                }
            }
        }
        sum += 10 * nums.first().unwrap() + nums.last().unwrap();
    }

    println!("Answer: {}", sum);

    Ok(())
}
