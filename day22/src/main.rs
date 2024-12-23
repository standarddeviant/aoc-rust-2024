use indicatif::ProgressBar;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn parse_numbers(input: String) -> Vec<i64> {
    let mut out: Vec<i64> = vec![];
    let re = Regex::new(r"\d+").unwrap();

    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());
    for line in contents.lines() {
        let tmps: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let tmpi: Vec<i64> = tmps.iter().map(|s| s.parse::<i64>().unwrap()).collect();
        out.extend(tmpi);
    }

    out
}

// NOTE: To mix a value into the secret number...
// 1. calculate the bitwise XOR of the given value and the secret number
// 2. then, the secret number becomes the result of that operation.
//    - If the secret number is 42 and you were to mix 15 into the secret number, the secret number would become 37.
fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

// NOTE: pruning is 'modulo X w/ 16777216', but hex(16777216) == 0x1000000, and...
// modulo X w/ 2**n is equal to X & (2**n - 1)
fn prune(a: i64) -> i64 {
    a & (16777216 - 1)
}

fn mix_prune_basic_check() {
    let a = 42;
    let b = 15;
    println!("mix({}, {}) = {}", a, b, mix(a, b));

    let c = 100000000;
    println!("prune({}) = {}", c, prune(c));
}

// In particular, each buyer's secret number evolves into the next secret number in the sequence via the following process:
//
// 1a. Calculate the result of multiplying the secret number by 64.
// 1b. Then, mix this result into the secret number. Finally, prune the secret number.
//
// 2a. Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer.
// 2b. Then, mix this result into the secret number. Finally, prune the secret number.
//
// 3a. Calculate the result of multiplying the secret number by 2048.
// 3b. Then, mix this result into the secret number. Finally, prune the secret number.

// try the naiive approach w/ bit shifts...
fn evolve(a: i64) -> i64 {
    let a1 = prune(mix(a, a << 6));
    let a2 = prune(mix(a1, a1 >> 5));
    let a3 = prune(mix(a2, a2 << 11));
    a3
}

fn prices_per_changes(avec: Vec<i64>) -> Vec<HashMap<(i32, i32, i32, i32), i32>> {
    let mut maps = vec![];
    // locs.entry(ch).or_insert(HashSet::default()).insert(_coord);

    println!("Part2: generating maps...");
    let pb = ProgressBar::new(avec.len() as u64);
    for a in avec {
        let mut p: i32 = 0;
        let mut prev: i32 = 0;
        let mut map: HashMap<(i32, i32, i32, i32), i32> = HashMap::default();
        let mut tmp = a.clone();
        let mut prices: Vec<i32> = vec![];
        let mut changes: Vec<i32> = vec![];

        for ix in 0..2000 {
            prev = (tmp % 10) as i32;
            tmp = evolve(tmp);
            p = (tmp % 10) as i32;

            prices.push(p as i32);
            changes.push((p - prev) as i32);
            if ix >= 3 {
                let k: (i32, i32, i32, i32) = (
                    changes[ix - 3],
                    changes[ix - 2],
                    changes[ix - 1],
                    changes[ix],
                );
                if !map.contains_key(&k) {
                    map.insert(k, prices[ix]);
                }
            }
        } // NOTE: end evolve loop: for ix in 0..2000
        maps.push(map);
        pb.inc(1);
    } // NOTE: end secrets loop: for a in avec
    pb.finish_with_message("done generating maps");

    maps
}

fn evolve_basic_check() {
    let mut a = 123;
    for _ix in 0..10 {
        let olda = a;
        a = evolve(a);
        println!("{olda} -> {a}");
    }
}

fn main() {
    // mix_prune_basic_check();
    // evolve_basic_check();

    // NOTE: practice input
    // let secrets = vec![1, 10, 100, 2024];

    let mut out_vec = vec![];
    let secrets = parse_numbers("input.txt".into());
    println!("secrets.len() = {}", secrets.len());

    for s in secrets.clone() {
        let mut tmp = s;
        for _ix in 0..2000 {
            tmp = evolve(tmp);
        }
        out_vec.push(tmp);
    }
    let sum1: i64 = out_vec.iter().sum();
    println!("day22, part1 = {sum1}");

    // let (p, c) = prices_changes(123);
    // println!("p.len() = {}, c.len() = {}", p.len(), c.len());
    // for ix in 0..20 {
    //     println!("{:2} , {:2}", p[ix], c[ix]);
    // }
    //

    let maps = prices_per_changes(secrets.clone());

    let mut kset: HashSet<(i32, i32, i32, i32)> = HashSet::default();
    println!("Part2: Aggregating unique keys from {} maps", maps.len());
    let pb = ProgressBar::new(maps.len() as u64);
    for (ix, m) in maps.iter().enumerate() {
        kset.extend(m.keys());
        pb.inc(1);
    }
    pb.finish_with_message("done");

    // println!("kset.len() = {}", kset.len());
    let mut maxv = 0_i32;
    let mut maxk: (i32, i32, i32, i32) = (-10, -10, -10, -10);
    println!(
        "Part2: checking {} keys in {} maps...",
        kset.len(),
        maps.len()
    );
    let pb = ProgressBar::new(kset.len() as u64);
    for k in kset {
        let mut v = 0_i32;
        for m in &maps {
            if m.contains_key(&k) {
                v += m[&k];
            }
        }
        if v > maxv {
            maxv = v;
            maxk = k.clone();
        }
        pb.inc(1);
    }
    pb.finish_with_message("done generating maps");

    println!("maxv = {maxv}, maxk = {maxk:?}");
    println!("day22, part2 = {maxv}");
}
