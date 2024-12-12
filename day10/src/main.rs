use clap::Parser;
use itertools::Itertools;
use ndarray::{concatenate, Array, Array2, Order};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use log::{debug, info, trace};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
}

fn parse_single_digits(input: String) -> Vec<Vec<i64>> {
    let mut out: Vec<Vec<i64>> = vec![];
    let re = Regex::new(r"\d").unwrap();

    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());
    for line in contents.lines() {
        let tmps: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let tmpi: Vec<i64> = tmps.iter().map(|s| s.parse::<i64>().unwrap()).collect();
        out.push(tmpi);
    }

    out
}

fn printm(m: &Vec<Vec<i64>>) {
    for row in m {
        let tmps = row.iter().map(|x| format!("{x}")).join("");
        info!("{tmps}");
    }
}

fn printm_locs(m: &Vec<Vec<i64>>, locs: &Vec<(usize, usize)>) {
    assert!(m.len() > 0);
    let mut w = m.clone();
    let (nr, nc) = (w.len(), w[0].len());
    for r in 0..nr {
        for c in 0..nc {
            w[r][c] = -1;
        }
    }
    for loc in locs {
        w[loc.0][loc.0] = m[loc.0][loc.1];
    }

    for row in w {
        let tmps: String = row
            .iter()
            .map(|&x| if x >= 0 { format!("{x}") } else { format!(".") })
            .join("");
        info!("{tmps}");
    }
}

fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

// fn vec2_to_arr2<T>(x: &Vec<Vec<T>>) -> Array2<T>
// where
//     T: std::clone::Clone,
//     T: num_traits::identities::Zero,
// {
//     let nrows = x.len();
//     let ncols = x[0].len();
//     let sh = ((nrows, ncols), Order::RowMajor);
//     let mut a2 = Array2::<T>::zeros((nrows, ncols));
//     for r in 0..nrows {
//         for c in 0..ncols {
//             a2[r;c] = x[r][c];
//         }
//     }
//
//     a2
// }

// fn num_trails((r: usize, c: usize), m: &Vec<Vec<<i64>>) -> usize {
//     let mut out = 0;
//     out
// }

fn trace1(diff: i64, src: &(usize, usize), m: &Vec<Vec<i64>>) -> Vec<(usize, usize)> {
    assert!(m.len() > 0);
    let (nr, nc) = (m.len() as i64, m[0].len() as i64);
    let mut out: Vec<(usize, usize)> = vec![];
    // look up/down/left/right
    let x = m[src.0][src.1];
    let looks: Vec<(i64, i64)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
    for look in looks {
        let r = src.0 as i64 + look.0;
        let c = src.1 as i64 + look.1;
        if r < 0 || nr <= r || c < 0 || nc <= c {
            trace!("continue on {:?}, sz={:?}", (r, c), (nr, nc));
            continue;
        }
        // safe to look at indices r,c and cast as usize's

        let y = m[r as usize][c as usize];
        // debug!(
        //     "check: {diff_check} = {x}-{y} @ r,c = {:?} ; sz={:?}",
        //     (r, c),
        //     (nr, nc)
        // );
        if diff == y - x {
            out.push((r as usize, c as usize));
        }
    }

    out
}

fn trace_from_starts(starts: &Vec<(usize, usize)>, m: &Vec<Vec<i64>>) -> usize {
    let mut out = 0;
    for s in starts {
        // let mut tracks: Vec<Vec<(usize, usize)>> = vec![vec![s.clone()]];
        // NOTE: insert vec of 0s at index 0
        let mut locs: Vec<Vec<(usize, usize)>> = vec![vec![s.clone()]];

        // NOTE: insert vec of Xs at index X; X=1..9
        for next in 1..=9 {
            // debug!("next = {next}");
            let mut thisv: Vec<(usize, usize)> = vec![];
            // debug!("locs.len() = {}", locs.len());
            let lastv: &Vec<(usize, usize)> = &locs[next - 1];
            for loc in 0..lastv.len() {
                thisv.extend(trace1(1, &lastv[loc], &m));
            }
            let vals: Vec<i64> = thisv.iter().map(|x| m[x.0][x.1]).collect();
            debug!("thisv (vals) = {:?}", vals);
            locs.push(thisv);
        }

        // how many 9's do we have?

        let nines = locs[9].clone();
        let nines: HashSet<&(usize, usize)> = HashSet::from_iter(nines.iter());
        let nines: Vec<(usize, usize)> = nines.iter().map(|&x| x.clone()).collect();
        debug!("nines.len() = {}", nines.len());
        debug!("nines = {:?}", locs[9]);
        debug!("s = {s:?}");
        // printm_locs(&m, &flatten(locs));
        // printm(&m);
        out += nines.len();
    }

    out
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let m = parse_single_digits(args.input);
    printm(&m);

    // find starts
    let mut starts: Vec<(usize, usize)> = vec![];
    // let mut ends: Vec<(usize, usize)> = vec![];
    let nrows = m.len();
    let ncols = m[0].len();
    for r in 0..nrows {
        for c in 0..ncols {
            match m[r][c] {
                0 => {
                    debug!("zero @ ({r}, {c})");
                    starts.push((r, c));
                }
                9 => {
                    debug!("nine @ ({r}, {c})");
                    // ends.push((r, c));
                }
                _ => {}
            }
        }
    }

    // trace back from ends
    let sum1 = trace_from_starts(&starts, &m);
    println!("day10, part1 = {sum1}");
}
