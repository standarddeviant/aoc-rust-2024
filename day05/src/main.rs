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

fn apply_rule(update: &mut Vec<i32>, rule: Vec<i32>) {
    if update.contains(&rule[0]) && update.contains(&rule[1]) {
        loop {
            let i0 = update.iter().position(|&x| x == rule[0]).unwrap();
            let i1 = update.iter().position(|&x| x == rule[1]).unwrap();
            if i0 < i1 {
                // println!("stopping {rule:?} to {update:?}");
                // println!(
                //     "stopping to apply {rule:?} : ({} @ {i0}) -> ({} @ {i1})",
                //     rule[0], rule[1]
                // );
                break;
            }
            // println!("applying {rule:?} to {update:?}");
            assert!(i1 < update.len() - 1);
            update.swap(i1, i1 + 1);
        }
        // hmmm
    }
}

fn rules_correctly_applied(update: &mut Vec<i32>, rules: Vec<Vec<i32>>) -> bool {
    for rule in rules {
        if update.contains(&rule[0]) && update.contains(&rule[1]) {
            let i0 = update.iter().position(|&x| x == rule[0]).unwrap();
            let i1 = update.iter().position(|&x| x == rule[1]).unwrap();
            if i1 < i0 {
                // println!("rule {rule:?}, does not apply to {update:?}");
                return false;
            }
        }
    }
    return true;
}

fn main() {
    let args = Args::parse();
    let (rules, updates) = parse_input(args.input);
    // println!("rules = {:#?}", rules.clone());
    // println!("updates = {updates:?}");

    let mut sum1 = 0;
    let mut sum2 = 0;
    for u in &updates {
        let mut w = u.clone();
        // at first it seemed only one pass of each rule was sufficient, but...
        // the same number is handled by multiple rules
        // so, what can happen is...
        // a rule containing X,Y is applied
        // then a rule about Z,X is applied
        // with the algo I chose, the second operation can move X 'past' Y when applying Z
        // So that's why we have a while loop with this simple algo
        while !rules_correctly_applied(&mut w, rules.clone()) {
            for rule in rules.clone() {
                apply_rule(&mut w, rule);
            }
        }
        if w.eq(u) {
            sum1 += w[w.len() / 2];
        } else {
            // println!("w = {w:?}");
            // println!("u = {u:?}");
            sum2 += w[w.len() / 2];
        }
    }
    println!("day05, part1 = {sum1}");
    println!("day05, part2 = {sum2}");
}

// EOF
