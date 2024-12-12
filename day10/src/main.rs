use clap::Parser;
use itertools::Itertools;
use log::{debug, info, trace};
use petgraph::algo::all_simple_paths;
use petgraph::prelude::DiGraphMap;
use regex::Regex;
use std::collections::HashSet;

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

fn trace_from_starts(starts: &Vec<(usize, usize)>, m: &Vec<Vec<i64>>) -> (usize, usize) {
    // assert(m)
    let mut out1 = 0;
    let mut out2 = 0;
    for s in starts {
        let mut g: DiGraphMap<(usize, usize), i32> = DiGraphMap::default();
        // let mut tracks: Vec<Vec<(usize, usize)>> = vec![vec![s.clone()]];
        // NOTE: here we insert vec of 0s at index 0
        let mut locs: Vec<Vec<(usize, usize)>> = vec![vec![s.clone()]];
        let mut loc0: (usize, usize) = (0, 0);
        // let mut ni_9s: Vec<NodeIndex> = vec![];
        // let mut ni_2_loc = HashMap::<NodeIndex, (usize, usize)>::default();

        // NOTE: here we insert vec of Xs at index X; X=1..9
        for next in 1..=9 {
            // debug!("next = {next}");
            let mut thisv: Vec<(usize, usize)> = vec![];
            // debug!("locs.len() = {}", locs.len());
            let lastv: &Vec<(usize, usize)> = &locs[next - 1];
            for ix in 0..lastv.len() {
                let from = lastv[ix];
                let tos = trace1(1, &lastv[ix], &m);
                thisv.extend(tos.clone()); // part1
                g.add_node(lastv[ix]); // part2

                // if next == 1, then there's only (1) 0-node in lastv[]
                if next == 1 {
                    loc0 = lastv[ix];
                }
                for to in tos {
                    g.add_node(to);
                    g.add_edge(lastv[ix], to, 1);
                    debug!(
                        "edge from {}::{:?} --> {}::{:?}",
                        m[from.0][from.1], from, m[to.0][to.1], to,
                    );
                }
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
        out1 += nines.len();

        for loc9 in nines {
            let ways = all_simple_paths::<Vec<_>, _>(&g, loc0, loc9, 0, None).collect::<Vec<_>>();
            debug!(
                "found {} paths from {}::{:?} --> {}::{:?}",
                ways.len(),
                m[loc0.0][loc0.1],
                loc0,
                m[loc9.0][loc9.1],
                loc9,
            );

            out2 += ways.len();
        }
        // println!("for zero @ {:?}:", locs[0][0]);
        // println!("g = {g:?}");
    } // NOTE: end for s in starts

    (out1, out2)
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
    let (sum1, sum2) = trace_from_starts(&starts, &m);
    println!("day10, part1 = {sum1}");
    println!("day10, part2 = {sum2}");
}
