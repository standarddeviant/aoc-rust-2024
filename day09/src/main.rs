use clap::Parser;
use geo::geometry::{Coord, Point, Polygon};
use geo::polygon;
use geo::Contains;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
}

pub const SPACE: i64 = -1_i64;

fn last_fill(v: &Vec<i64>) -> usize {
    v.iter().positions(|x| *x >= 0).max().unwrap()
}

fn first_space(v: &Vec<i64>) -> usize {
    v.iter().positions(|x| *x < 0).min().unwrap()
}

fn custom_sort_1(v: &mut Vec<i64>) {
    let mut itr = 0;
    loop {
        let _lastf = last_fill(v);
        let _firsts = first_space(v);
        if _lastf < _firsts {
            break;
        }
        v.swap(_lastf, _firsts);
        // println!("cs1: {itr} : {v:?}");
        itr += 1;
    }
}

fn parse_blocks(input: String) -> Vec<i64> {
    let mut w: Vec<i64> = vec![];
    // let re = Regex::new(r"\d+").unwrap();

    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());
    for line in contents.lines() {
        // there should only be one line...
        let tmpv: Vec<String> = line.chars().map(|c| format!("{c}")).collect();
        // println!("tmpv = {tmpv:?}");
        // println!("tmpv.len() = {}", tmpv.len());
        w = tmpv.iter().map(|s| s.parse::<i64>().unwrap()).collect();
    }

    // expand 'parsed-repr' into 'unsorted-block-repr'
    let mut w2: Vec<i64> = vec![];
    let mut file_ix = 0;
    for ix in 0..w.len() {
        let is_file = 0 == ix % 2;
        if is_file {
            let tmp: Vec<i64> = vec![file_ix; w[ix] as usize];
            w2.extend(tmp);
            file_ix += 1;
        } else {
            let tmp: Vec<i64> = vec![-1_i64; w[ix] as usize];
            w2.extend(tmp);
        }
    }

    w2
}

fn main() {
    let args = Args::parse();
    let mut b = parse_blocks(args.input);

    // sort our list...
    custom_sort_1(&mut b);

    let mut sum1 = 0;
    for (ix, fnum) in b.iter().enumerate() {
        if *fnum >= 0_i64 {
            sum1 += fnum * ix as i64;
        }
    }
    println!("day09, part1 = {sum1}");
}
