use itertools::*;

fn parse(input: &str) -> Vec<Vec<i32>> {
    let lines = input.lines();
    let iter = lines.map(|line| {
        let report: Vec<i32> = line
            .split(" ")
            .map(|x| x.parse::<i32>().expect("Not a number!"))
            .collect();
        report
    });

    iter.collect()
}

fn is_correct(report: &Vec<i32>) -> bool {
    let len = report.len();
    if len < 2 {
        return false;
    }
    let mut last = report[0];
    let mut cur = report[1];
    let sign = (cur - last) > 0;
    let mut index = 1;
    while index < len {
        cur = report[index];
        let sign2 = (cur - last) > 0;
        if sign2 != sign {
            return false;
        }
        let diff = (cur - last).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        last = cur;
        index += 1;
    }
    return true;
}

fn is_correct_or_recoverable(report: &Vec<i32>) -> bool {
    if is_correct(&report) {
        return true;
    }
    let len = report.len();
    for removed in 0..len {
        let mut cleaned = report.clone();
        let _ = cleaned.remove(removed);
        if is_correct(&cleaned) {
            return true;
        }
    }
    return false;
}

pub fn part1(input: &str) -> anyhow::Result<()> {
    let reports = parse(input);
    let count = reports.iter().fold(
        0,
        |acc, report| {
            if is_correct(report) {
                acc + 1
            } else {
                acc
            }
        },
    );
    println!("total: {count}");
    Ok(())
}

pub fn part2(input: &str) -> anyhow::Result<()> {
    let reports = parse(input);
    let count = reports.iter().fold(0, |acc, report| {
        if is_correct_or_recoverable(report) {
            acc + 1
        } else {
            acc
        }
    });
    println!("total: {count}");
    Ok(())
}
