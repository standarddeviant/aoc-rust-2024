use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

fn parse_init_rules(
    input: String,
) -> (
    HashMap<String, u8>,
    HashMap<String, Vec<usize>>,
    Vec<(String, String, String, String)>,
    Vec<String>,
) {
    // let mut out: HashMap<String, HashSet<String>> = HashMap::default();
    let mut vals: HashMap<String, u8> = HashMap::default();
    let mut fwdlu: HashMap<String, Vec<usize>> = HashMap::default(); // lookup into rules
    let mut rules: Vec<(String, String, String, String)> = vec![];
    // let mut zkeys: HashSet<String> = HashSet::default();
    let mut zkeys: Vec<String> = Vec::default();

    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());
    for line in contents.lines() {
        // println!("{line}");
        if line.contains(":") {
            // x00: 1
            let re = Regex::new(r"(...): (\d)").unwrap();
            let caps = re.captures(line).unwrap();
            let k = caps.get(1).unwrap().as_str();
            let v = caps.get(2).unwrap().as_str();
            // println!("k = {k}, v = {v}");
            let v = v.parse::<u8>().unwrap();
            vals.insert(k.into(), v);
        } else if line.contains("->") {
            // x00 AND y00 -> z00
            let re = Regex::new(r"(\w+) (\w+) (\w+) -> (\w+)").unwrap();
            let caps = re.captures(line).unwrap();
            let a: String = caps.get(1).unwrap().as_str().into();
            let op: String = caps.get(2).unwrap().as_str().into();
            let b: String = caps.get(3).unwrap().as_str().into();
            let c: String = caps.get(4).unwrap().as_str().into();
            if c.starts_with("z") {
                zkeys.push(c.clone());
            }
            rules.push((a.clone(), op, b.clone(), c));
            let ix = rules.len() - 1;
            // if fwdlu.contains_key(&a) || fwdlu.contains_key(&b) {
            //     println!("danger!");
            // }
            //
            // NOTE: this syntax is kind of wild, but very convenient...
            // NOTE: basically...
            // 1. get entry() or insert empty vec if needed
            // 2. push ix to vec
            fwdlu.entry(a).or_insert(vec![]).push(ix);
            fwdlu.entry(b).or_insert(vec![]).push(ix);
        }
    }

    // NOTE: sort zkeys here for convenience
    zkeys.sort();
    (vals, fwdlu, rules, zkeys)
}

fn main() {
    // let (mut vals, fwdlu, rules, zkeys) = parse_init_rules("toy.txt".into());
    // let (mut vals, fwdlu, rules, zkeys) = parse_init_rules("practice.txt".into());
    let (mut vals, fwdlu, rules, zkeys) = parse_init_rules("input.txt".into());
    println!("vals =\n{vals:?}");
    println!("fwdlu =\n{fwdlu:?}");
    println!("rules =\n{rules:?}");

    // NOTE: part1
    'outer: loop {
        print!(".");
        io::stdout().flush().unwrap();

        let iter_vals = vals.clone();
        for k in iter_vals.keys() {
            if let Some(rules_ix_vec) = fwdlu.get(k) {
                for rules_ix in rules_ix_vec {
                    let (a, op, b, c) = rules[*rules_ix].clone();
                    if !vals.contains_key(&a) || !vals.contains_key(&b) {
                        continue;
                    }
                    let aval = vals.get(&a).unwrap();
                    let bval = vals.get(&b).unwrap();
                    let cval = match op.as_str() {
                        "AND" => aval & bval,
                        "XOR" => aval ^ bval,
                        "OR" => aval | bval,
                        _ => panic!(),
                    };
                    vals.insert(c, cval);
                }
            }
        }

        for zkey in &zkeys {
            if !vals.contains_key(zkey) {
                continue 'outer;
            }
        }

        // if the above loop is completely satisfied, then we're done, so break;
        break;
    }

    println!("");
    let mut zdec: u64 = 0;
    for (zix, zkey) in zkeys.iter().enumerate() {
        println!("vals[{zkey}] = {}", vals.get(zkey).unwrap());
        let zbit: u64 = vals.get(zkey).unwrap().clone() as u64;
        zdec += zbit << zix;
    }

    println!("day24, part1 = {zdec}");
}
