use regex::Regex;
// use std::ops::BitXor
// NOTE: To mix a value into the secret number...
// 1. calculate the bitwise XOR of the given value and the secret number
// 2. then, the secret number becomes the result of that operation.
//    - If the secret number is 42 and you were to mix 15 into the secret number, the secret number would become 37.
fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

// NOTE: pruning is 'modulo X w/ 16777216', but hex(16777216) == 0x1000000, and...
// modulo X w/ 2**n is equal to X & (2**n - 1)
fn prune(a: u64) -> u64 {
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
fn evolve(a: u64) -> u64 {
    let a1 = prune(mix(a, a << 6));
    let a2 = prune(mix(a1, a1 >> 5));
    let a3 = prune(mix(a2, a2 << 11));
    a3
}

fn evolve_basic_check() {
    let mut a = 123;
    for _ix in 0..10 {
        let olda = a;
        a = evolve(a);
        println!("{olda} -> {a}");
    }
}

fn parse_numbers(input: String) -> Vec<u64> {
    let mut out: Vec<u64> = vec![];
    let re = Regex::new(r"\d+").unwrap();

    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());
    for line in contents.lines() {
        let tmps: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let tmpi: Vec<u64> = tmps.iter().map(|s| s.parse::<u64>().unwrap()).collect();
        out.extend(tmpi);
    }

    out
}

fn main() {
    // mix_prune_basic_check();
    // evolve_basic_check();

    // NOTE: practice input
    // let secrets = vec![1, 10, 100, 2024];

    let mut out_vec = vec![];
    let secrets = parse_numbers("input.txt".into());
    println!("secrets.len() = {}", secrets.len());

    for s in secrets {
        let mut tmp = s;
        for _ix in 0..2000 {
            tmp = evolve(tmp);
        }
        out_vec.push(tmp);
    }
    let sum1: u64 = out_vec.iter().sum();
    println!("day22, part1 = {sum1}");
    // let practice_input: Vec<u64> = vec![
    //     15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
    // ];
}
