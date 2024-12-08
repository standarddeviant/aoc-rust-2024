use clap::Parser;
use itertools::Itertools;
use regex::{Captures, Match, Regex, RegexSet};
use std::cmp::{Ordering, PartialOrd};
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::hash::RandomState;

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

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Cat,
}

fn calculate(x: &Vec<i64>, op_seq: &Vec<&Op>) -> i64 {
    assert!(x.len() >= 3);
    assert!(x.len() - 2 == op_seq.len());
    let mut out: i64 = x[1];
    for ix in 2..x.len() {
        out = match op_seq[ix - 2] {
            Op::Add => out + x[ix],
            Op::Sub => out - x[ix],
            Op::Mul => out * x[ix],
            Op::Div => out / x[ix],
            Op::Cat => {
                let tmps = format!("{}{}", out, x[ix]);
                let tmpi: i64 = tmps.parse().unwrap();
                tmpi
            }
        }
    }

    out
}

// fn make_perms(n_ops: usize, ops: &Vec<Op>) -> Vec<Vec<Op>> {
//     let out: Vec<Vec<Op>> = vec![];
//     let mut w: Vec<Op> = vec![Op::Add; n_ops];
//     for ix in 0..n {
//         for op in ops {
//             w[ix] =
//         }
//     }
//
//
//     out
// }

fn has_solution(x: &Vec<i64>, ops: &Vec<Op>) -> bool {
    let n_ops = x.len() - 2;
    if n_ops <= 0 {
        return false;
    }

    // WARN: this is a messy way to all possible permutations (sequences?)
    // WARN: there must be a bettery way to do this...

    // let perm_ops =
    // let ops: Vec<Op> = ops.clone().repeat(n_ops);
    // let mut perm_ops: Vec<Vec<Op>> = vec![];
    // for tmpp in ops.iter().permutations(n_ops).unique() {
    //     perm_ops.push(tmpp.iter().map(|&o| *o).collect());
    // }

    let perm_ops: Vec<Vec<&Op>> = (1..=n_ops)
        .map(|_| ops.iter())
        .multi_cartesian_product()
        .collect_vec();
    // println!("{i:?}");
    // }
    for op_seq in perm_ops {
        let check = calculate(&x, &op_seq);
        if check == x[0] {
            return true;
        }
    }

    false
}

fn main() {
    let args = Args::parse();
    let vv: Vec<Vec<i64>> = parse_numbers(args.input);

    println!("vv = ");
    let mut sum1 = 0;
    for tmpv in &vv {
        println!("  {tmpv:?}");
        if has_solution(tmpv, &vec![Op::Add, Op::Mul]) {
            sum1 += tmpv[0];
        }
    }
    println!("day07, part 1 = {sum1}");

    println!("vv = ");
    let mut sum2 = 0;
    for tmpv in &vv {
        println!("  {tmpv:?}");
        if has_solution(tmpv, &vec![Op::Add, Op::Mul, Op::Cat]) {
            sum2 += tmpv[0];
        }
    }
    println!("day07, part 2 = {sum2}");
}
