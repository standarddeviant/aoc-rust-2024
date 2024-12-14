// use clap::Parser;
use log::debug;
use std::collections::HashMap;

// #[derive(Debug, Parser)]
// #[command(version, about, long_about = None)]
// struct Args {
//     input: String,
// }

// fn parse_ints(input: String) -> Vec<i64> {
//     let out: Vec<i64> = vec![];
//     let re = Regex::new(r"\d+").unwrap();
//
//     let contents = std::fs::read_to_string(input.clone())
//         .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());
//     for line in contents.lines() {
//         let tmps: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
//         let tmpi: Vec<i64> = tmps.iter().map(|s| s.parse::<i64>().unwrap()).collect();
//         return tmpi;
//         // out.push(tmpi);
//     }
//
//     out
// }

fn ndigs(x: &i64) -> usize {
    let tmps = format!("{x}");
    return tmps.len();
}

fn dig_split(x: &i64) -> (i64, i64) {
    let tmps = format!("{x}");
    let nd = tmps.len();
    let ld: usize = nd / 2;
    let lnum: i64 = tmps[0..ld].parse().unwrap();
    let rnum: i64 = tmps[ld..].parse().unwrap();

    (lnum, rnum)
}

fn blink_hm(sh: &mut HashMap<i64, i64>) {
    let copy = sh.clone();
    let keys = copy.keys();
    let mut add_after_blink_op: Vec<(i64, i64)> = vec![];
    for k in keys {
        let n = sh[k];
        if 0 == n {
            continue;
        }
        // NOTE: remove 'these stones'
        debug!("sub : {k} -= {n}");
        *sh.get_mut(k).unwrap() -= n;

        // NOTE: add 'new stones'
        if 0 == *k {
            debug!("add (AFTER): 1 += {n}");
            add_after_blink_op.push((1, n));
        } else if 0 == (ndigs(k) % 2) {
            let (lnum, rnum) = dig_split(k);
            debug!("add (AFTER): {lnum} += {n}");
            debug!("add (AFTER): {rnum} += {n}");
            add_after_blink_op.push((lnum, n));
            add_after_blink_op.push((rnum, n));
        } else {
            let newk = k * 2024;
            debug!("add (AFTER): {newk} += {n}");
            add_after_blink_op.push((newk, n));
        }
    } // NOTE: end for k in keys

    // NOTE: add the new values from the 'split' operations after we've 'blinked'
    // on all the original stones
    for (k, v) in add_after_blink_op {
        *sh.entry(k).or_insert(0) += v;
    }
}

// fn press_enter() {
//     let mut input = String::new();
//     println!("Press Enter...");
//     std::io::stdin()
//         .read_line(&mut input)
//         .expect("error: unable to read user input");
// }

fn main() {
    env_logger::init();
    let stones = vec![0, 4, 4979, 24, 4356119, 914, 85734, 698829]; // input
    let mut sh: HashMap<i64, i64> = HashMap::default();
    for s in &stones {
        sh.insert(s.clone(), 1);
    }

    let mut s_h = sh.clone();
    for itr in 0..75 {
        // s_v.sort();
        // println!("\n *** {_ix} : {s_v:?}");
        blink_hm(&mut s_h);
        if 24 == itr {
            let sum_1_hm: i64 = s_h.values().sum();
            println!("day11, part 1 = {}", sum_1_hm);
        } else if 74 == itr {
            let sum_1_hm: i64 = s_h.values().sum();
            println!("day11, part 1 = {}", sum_1_hm);
        }

        // let mut tmpks: Vec<&i64> = s_h.keys().collect_vec();
        // tmpks.sort();
        // for k in tmpks {
        //     if s_h[k] > 0 {
        //         debug!("    {_ix} : {k} : {}", s_h[k]);
        //     }
        // }
        //
        // blink_vec(&mut s_v);
        // println!("{_ix} : {s_v:?}");
    }
}
