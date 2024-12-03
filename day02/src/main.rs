use clap::Parser;
use regex::{Match, Regex};
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
}

fn parse_numbers(input: String) -> Vec<Vec<i64>> {
    let mut out: Vec<Vec<i64>> = vec![];
    let re = Regex::new(r"\d+").unwrap();

    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());
    for line in contents.lines() {
        let tmps: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let tmpi: Vec<i64> = tmps.iter().map(|s| s.parse::<i64>().unwrap()).collect();
        out.push(tmpi);
    }

    out
}

fn check_part1(a: &Vec<i64>) -> bool {
    let mut p_count = 0;
    let mut n_count = 0;
    for ix in 0..(a.len() - 1) {
        let d = a[ix + 1] - a[ix];
        let a = d.abs();
        if d > 0 {
            p_count += 1;
        } else {
            n_count += 1;
        }
        if a < 1 || 3 < a {
            return false;
        }
    }
    // check all same direction
    if p_count > 0 && n_count > 0 {
        return false;
    }
    return true;
}

fn check_part2(a: &Vec<i64>) -> bool {
    if check_part1(a) {
        return true;
    }
    for skip_ix in 0..a.len() {
        let mut tmpv = a.clone();
        tmpv.remove(skip_ix);
        if check_part1(&tmpv) {
            return true;
        }
    }
    return false;
}

fn main() {
    let args = Args::parse();
    let nums = parse_numbers(args.input);

    let mut sum1 = 0;
    for v in &nums {
        if check_part1(v) {
            sum1 += 1;
        }
    }
    println!("day02, part1 = {sum1}");

    let mut sum2 = 0;
    for v in &nums {
        if check_part2(v) {
            sum2 += 1;
        }
    }
    println!("day02, part2 = {sum2}");
}
