use std::collections::HashMap;

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines();
    let left: Vec<i32>;
    let right: Vec<i32>;
    let iter = lines.map(|line| {
        if let Some((a, b)) = line.split_once("   ") {
            let l: i32 = a.parse().expect("Not a i32");
            let r: i32 = b.parse().expect("Not a u32");
            (l, r)
        } else {
            unreachable!();
        }
    });
    (left, right) = iter.unzip();

    (left, right)
}

pub fn part1(input: &str) -> anyhow::Result<()> {
    let (mut left, mut right) = parse(&input);
    left.sort();
    right.sort();
    let result = left
        .iter()
        .zip(right.iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs());
    println!("Result: {result}");
    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let (left, right) = parse(&input);
    let counts = right.iter().fold(HashMap::<i32, i32>::new(), |mut map, x| {
        if let Some(count) = map.get(x) {
            map.insert(*x, count + 1);
        } else {
            map.insert(*x, 1);
        }
        map
    });
    let result: i64 = left.iter().fold(0, |acc, x| {
        if let Some(c) = counts.get(&x) {
            acc + (x * c) as i64
        } else {
            acc
        }
    });
    println!("Result: {result}");
    Ok(())
}
