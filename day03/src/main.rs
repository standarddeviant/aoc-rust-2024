use clap::Parser;
use regex::{Captures, Match, Regex, RegexSet};
use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
}

fn parse_muls(input: String) -> Vec<(i64, i64)> {
    let mut out: Vec<(i64, i64)> = vec![];
    let re_mul = Regex::new(r"mul\((?<int1>\d+),(?<int2>\d+)\)").unwrap();
    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());
    for line in contents.lines() {
        // let tmps: Vec<(&str, &str)> = re.captures_iter(line).map(|c| (c[0], c[1])).collect();
        let tmpv: Vec<(i64, i64)> = re_mul
            .captures_iter(line)
            .map(|c| {
                // println!("DBG: c = {c:?}");
                let int1 = c[1].parse::<i64>().unwrap();
                let int2 = c[2].parse::<i64>().unwrap();
                (int1, int2)
            })
            .collect();
        out.extend(tmpv);
    }

    out
}

fn parse_muls_part2(input: String) -> Vec<(i64, i64)> {
    let mut out: Vec<(i64, i64)> = vec![];

    // NOTE: set up (2) to different regex patterns
    let re_mul = Regex::new(r"mul\((?<int1>\d+),(?<int2>\d+)\)").unwrap();
    let re_dos_donts = Regex::new(r"(do|don\'t)\(\)").unwrap();

    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());

    // NOTE: get regex captures, combine vectors and sort vectors
    let mut cvec: Vec<Captures> = re_dos_donts.captures_iter(contents.as_str()).collect();
    let mul_vec: Vec<Captures> = re_mul.captures_iter(contents.as_str()).collect();
    cvec.extend(mul_vec);
    cvec.sort_by(|a, b| {
        if a.get(0).unwrap().start() < b.get(0).unwrap().start() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    // NOTE: iterate through sorted vector of regex captures
    let mut enabled: bool = true;
    let mut ix = 0;
    for c in cvec {
        let s = c.get(0).unwrap().as_str();
        // println!("DBG: @ ix = {ix} : enabled = {enabled}, s = {s:?}");
        // println!("{:?}, {}", c.get(0), c[0].eq("mul"));
        if enabled && s.starts_with("mul") {
            let i1: i64 = c[1].parse().unwrap();
            let i2: i64 = c[2].parse().unwrap();
            out.push((i1, i2));
        } else if s.starts_with("don't") {
            enabled = false;
        } else if s.starts_with("do") {
            enabled = true;
        }

        ix += 1;
    }

    out
}

fn main() {
    let args = Args::parse();
    let muls = parse_muls(args.input.clone());
    // println!("muls = {muls:?}");

    let mut sum1 = 0;
    for m in muls {
        sum1 += m.0 * m.1;
    }
    println!("day03, part1 = {sum1}");

    let mut sum2 = 0;
    let muls2 = parse_muls_part2(args.input);
    for m in muls2 {
        sum2 += m.0 * m.1;
    }
    println!("day03, part2 = {sum2}");
}
