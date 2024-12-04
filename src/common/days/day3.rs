use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MUL_REGEX: Regex = Regex::new(r"mul\((?<m1>-?\d+),(?<m2>-?\d+)\)").unwrap();
    static ref MUL_WITH_DO_REGEX: Regex =
        Regex::new(r"(mul\((?<m1>-?\d+),(?<m2>-?\d+)\)|do\(\)|don't\(\))").unwrap();
}

pub fn part1(input: &str) -> anyhow::Result<()> {
    let mut total: i64 = 0;
    for (_, [x, y]) in MUL_REGEX.captures_iter(input).map(|cap| cap.extract()) {
        let x = x.parse::<i64>().expect("Should be an int");
        let y = y.parse::<i64>().expect("Should be an int");
        total += x * y;
    }
    println!("TOTAL: {total}");
    Ok(())
}
pub fn part2(input: &str) -> anyhow::Result<()> {
    let mut total: i64 = 0;
    let mut include = true;

    for cap in MUL_WITH_DO_REGEX.captures_iter(input) {
        match cap.get(0).unwrap().as_str() {
            "do()" => {
                include = true;
            }
            "don't()" => {
                include = false;
            }
            _ => {
                let x = cap.name("m1").unwrap().as_str();
                let y = cap.name("m2").unwrap().as_str();
                let x = x.parse::<i64>().expect("Should be an int");
                let y = y.parse::<i64>().expect("Should be an int");
                if include {
                    total += x * y;
                }
            }
        }
    }
    println!("TOTAL: {total}");
    Ok(())
}
