use clap::Parser;
use itertools::Itertools;
use log::{debug, info, trace};
// use petgraph::algo::all_simple_paths;
// use petgraph::prelude::DiGraphMap;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
}

fn parse_ints(input: String) -> Vec<i64> {
    let out: Vec<i64> = vec![];
    let re = Regex::new(r"\d+").unwrap();

    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());
    for line in contents.lines() {
        let tmps: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let tmpi: Vec<i64> = tmps.iter().map(|s| s.parse::<i64>().unwrap()).collect();
        return tmpi;
        // out.push(tmpi);
    }

    out
}

fn ndigs(x: i64) -> usize {
    let tmps = format!("{x}");
    return tmps.len();
}

fn dig_split(x: i64) -> (i64, i64) {
    let tmps = format!("{x}");
    let nd = tmps.len();
    let ld: usize = nd / 2;
    let lnum: i64 = tmps[0..ld].parse().unwrap();
    let rnum: i64 = tmps[ld..].parse().unwrap();

    (lnum, rnum)
}

fn blink1(stones: &mut Vec<i64>) {
    // let mut w = stones.clone();
    let mut ix = 0;
    loop {
        if ix >= stones.len() {
            break;
        }

        if 0 == stones[ix] {
            stones[ix] = 1;
            ix += 1;
        } else if 0 == (ndigs(stones[ix]) % 2) {
            let (lnum, rnum) = dig_split(stones[ix]);
            stones.insert(ix, lnum);
            ix += 1;
            stones[ix] = rnum;
            ix += 1;
        } else {
            stones[ix] = 2024 * stones[ix];
            ix += 1;
        }
    }
}

fn press_enter() {
    let mut input = String::new();
    println!("Press Enter...");
    std::io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
}

fn main() {
    // let args = Args::parse();
    // let mut stones = parse_ints(args.input);
    // println!("stones = {stones:?}");

    // let mut stones = vec![125, 17]; // practice1
    let mut stones = vec![0, 4, 4979, 24, 4356119, 914, 85734, 698829]; // input

    for _ix in 0..25 {
        blink1(&mut stones);
        // println!("{} : {}", _ix + 1, stones.len());
    }
    println!("day11, part 1 = {}", stones.len());
}
