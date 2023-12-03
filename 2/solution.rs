use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

const NUM_REDS: u32 = 12;
const NUM_GREENS: u32 = 13;
const NUM_BLUES: u32 = 14;

struct ParseError;

impl FromStr for Draw {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(",").collect::<Vec<&str>>();
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for elem in split {
            let mut words = elem.split_whitespace();
            if let (Some(num), Some(color)) = (
                words.next().map(|x| x.parse::<u32>().unwrap()),
                words.next(),
            ) {
                match color {
                    "red" => r += num,
                    "green" => g += num,
                    "blue" => b += num,
                    _ => todo!(),
                };
            }
        }

        Ok(Draw {
            red: r,
            green: g,
            blue: b,
        })
    }
}

struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl FromStr for Game {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(":").collect::<Vec<&str>>();
        let id = split
            .first()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let draws = split
            .last()
            .unwrap()
            .split(";")
            .filter_map(|x| x.parse::<Draw>().ok())
            .collect();

        Ok(Game {
            id: id,
            draws: draws,
        })
    }
}

struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;

    let reader = io::BufReader::new(file);

    let games: Vec<Game> = reader
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| Game::from_str(&x).ok())
        .collect();

    let solution_1 = games
        .iter()
        .filter(|x| {
            for draw in &x.draws {
                if draw.red > NUM_REDS || draw.green > NUM_GREENS || draw.blue > NUM_BLUES {
                    return false;
                }
            }
            return true;
        })
        .fold(0, |acc, x| acc + x.id);

    println!("Answer 1: {}", solution_1);

    let solution_2: u32 = games
        .iter()
        .map(|x| {
            x.draws
                .iter()
                .map(|x| (x.red, x.green, x.blue))
                .reduce(|acc, e| (max(acc.0, e.0), max(acc.1, e.1), max(acc.2, e.2)))
                .unwrap()
        })
        .map(|(r, g, b)| r * g * b)
        .sum();

    println!("Answer 2: {}", solution_2);

    Ok(())
}
