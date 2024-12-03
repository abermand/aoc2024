mod days;

use anyhow::anyhow;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub type Solver = fn(&str) -> anyhow::Result<()>;

lazy_static! {
    static ref CHALLENGES: HashMap<u8, (Solver, Solver)> = {
        let mut m: HashMap<u8, (Solver, Solver)> = HashMap::new();
        m.insert(1, (days::day1::part1, days::day1::part2));
        m.insert(2, (days::day2::part1, days::day2::part2));
        m
    };
}

pub fn get_input(name: &str) -> anyhow::Result<String> {
    let file = File::open(name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_solver(day: u8, part: u8) -> anyhow::Result<Solver> {
    if let Some((p1, p2)) = CHALLENGES.get(&day) {
        match part {
            1 => return anyhow::Ok(*p1),
            2 => return anyhow::Ok(*p2),
            _ => return Err(anyhow!("Invalid part number: {part}")),
        }
    }
    Err(anyhow!("No implemented challenge for day {day}"))
}
