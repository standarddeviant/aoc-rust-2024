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

fn printv(v: &Vec<i64>) {
    let mut tmps: String = "".into();
    let tmps = v
        .iter()
        .map(|i| {
            if *i >= 0 {
                format!("{i}")
            } else {
                format!(".")
            }
        })
        .join("");
    println!("{tmps}");
}

fn custom_sort_1(v: &mut Vec<i64>) {
    let mut f = v.len() - 1;
    let mut s = 0;
    // let mut itr = 0;
    loop {
        // move s forward to next space
        while v[s] >= 0 && s < v.len() - 1 {
            s += 1;
        }
        // move f backward to next fill
        while v[f] < 0 && f > 0 {
            f -= 1;
        }
        // check if we should bail, we bail when...
        // 1. space isnt space
        // 2. fill isnt fill
        // 3. fill < space
        if v[s] >= 0 || v[f] < 0 || f < s {
            break;
        }
        // simple swap!
        v.swap(s, f);

        // remove + re-insert
        // let fval = v.remove(f);
        // v.insert(s, fval);

        // printv(&v);
        // itr += 1;
    }
}

fn parse_fill_space(input: String) -> Vec<i64> {
    let mut fs: Vec<i64> = vec![];
    // let re = Regex::new(r"\d+").unwrap();

    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());
    for line in contents.lines() {
        // there should only be one line...
        let tmps: Vec<String> = line.chars().map(|c| format!("{c}")).collect();
        // println!("fs = {}", tmpv.len());
        fs = tmps.iter().map(|s| s.parse::<i64>().unwrap()).collect();
        break;
    }

    fs
}

fn expand_blocks(fs: Vec<i64>) -> Vec<i64> {
    let mut blocks: Vec<i64> = vec![];
    // expand 'parsed-repr' into 'unsorted-block-repr'
    let mut file_ix = 0;
    for ix in 0..fs.len() {
        let is_file = 0 == ix % 2;
        if is_file {
            let tmp: Vec<i64> = vec![file_ix; fs[ix] as usize];
            blocks.extend(tmp);
            file_ix += 1;
        } else {
            let tmp: Vec<i64> = vec![-1_i64; fs[ix] as usize];
            blocks.extend(tmp);
        }
    }

    blocks
}

fn main() {
    let args = Args::parse();

    let fs = parse_fill_space(args.input);
    let mut b1 = expand_blocks(fs.clone());

    // sort our list...
    custom_sort_1(&mut b1);

    let mut sum1 = 0;
    for (ix, fnum) in b1.iter().enumerate() {
        if *fnum >= 0_i64 {
            sum1 += fnum * ix as i64;
        }
    }
    println!("day09, part1 = {sum1}");
}
