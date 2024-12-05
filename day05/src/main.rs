use clap::Parser;
use regex::{Captures, Match, Regex, RegexSet};
use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
}

fn parse_input(input: String) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut rules: Vec<Vec<i32>> = vec![];
    let mut updates: Vec<Vec<i32>> = vec![];
    let re_uint = Regex::new(r"\d+").unwrap();
    let re_rule = Regex::new(r"\d+\|\d+").unwrap();
    let re_update = Regex::new(r"\d+,\d+").unwrap();
    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());

    for line in contents.lines() {
        let tmps: Vec<&str> = re_uint.find_iter(line).map(|m| m.as_str()).collect();
        let tmpi: Vec<i32> = tmps.iter().map(|s| s.parse::<i32>().unwrap()).collect();
        if re_rule.is_match(line) {
            rules.push(tmpi);
        } else if re_update.is_match(line) {
            updates.push(tmpi);
        }
    }

    (rules, updates)
}

fn apply_rule(mut update: Vec<i32>, rule: Vec<i32>) -> Vec<i32> {
    if update.contains(&rule[0]) && update.contains(&rule[1]) {
        loop {
            let ixa = update.iter().position(|&x| x == rule[0]).unwrap();
            let ixb = update.iter().position(|&x| x == rule[1]).unwrap();
            if ixa < ixb {
                break;
            }
            assert!(ixb < update.len() - 1);
            update.swap(ixb, ixb + 1);
        }
        // hmmm
    }
    update
}

fn main() {
    let args = Args::parse();
    let (rules, updates) = parse_input(args.input);
    // println!("rules = {:?}", rules.clone());
    // println!("updates = {updates:?}");

    let mut sum1 = 0;
    for u in &updates {
        let mut w = u.clone();
        for rule in rules.clone() {
            w = apply_rule(w, rule);
        }
        if w.eq(u) {
            // correct order, yay
            // println!("correct order: {w:?}");
            sum1 += w[w.len() / 2];
        }
    }
    println!("day05, part1 = {sum1}");
}

// EOF
