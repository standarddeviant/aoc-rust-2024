use std::collections::HashMap;

use clap::Parser;
use regex::Regex;
// NOTE: use clap for cli args
// const PERIPHERAL_NAME_MATCH_FILTER: &str = "Neuro";
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
}

fn parse_whitespace_lists(input: String) -> (Vec<i64>, Vec<i64>) {
    let contents = std::fs::read_to_string(input).expect("Should have been able to read the file");

    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];
    let re = Regex::new(r"(\d+)\W+(\d+).*").unwrap();
    for line in contents.lines() {
        if let Some(m) = re.captures(line) {
            let tmp_l: i64 = m[1].parse().unwrap();
            let tmp_r: i64 = m[2].parse().unwrap();
            left.push(tmp_l);
            right.push(tmp_r);
        }
    }

    (left, right)
}

fn main() {
    let args = Args::parse();
    println!("args = {args:#?}");
    let (mut left, mut right) = parse_whitespace_lists(args.input);
    left.sort();
    right.sort();

    let mut sum1: i64 = 0;
    for ix in 0..left.len() {
        sum1 += i64::abs(left[ix] - right[ix]);
    }
    println!("day01: part1: answer is {sum1}");

    let mut n_times: HashMap<i64, i64> = HashMap::default();
    for ix in 0..right.len() {
        let k = right[ix];
        if n_times.contains_key(&k) {
            n_times.insert(k, n_times[&k] + 1);
        } else {
            n_times.insert(k, 1);
        }
    }

    let mut sum2: i64 = 0;
    for ix in 0..left.len() {
        let k = left[ix];
        if n_times.contains_key(&k) {
            sum2 += k * n_times[&k];
        }
    }

    println!("day01: part2: answer is {sum2}");
}
