use clap::Parser;
use geo::coords_iter::CoordsIter;
use geo::{Contains, Coord, Line, LineString, MultiPoint, Point, Polygon};
use itertools::Itertools;
use log::{debug, info, trace};
use std::collections::{HashMap, HashSet};

// use log::debug;
// use regex::Regex;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
}

fn parse_char_map(input: String) -> Vec<Vec<char>> {
    let mut out: Vec<Vec<char>> = vec![];
    // let re = Regex::new(r"\d+").unwrap();

    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());
    for line in contents.lines() {
        out.push(line.chars().collect());
    }

    out
}

fn debug_print_char_map(m: &Vec<Vec<char>>) {
    for row in m {
        let tmpv: Vec<String> = row.iter().map(|x| format!("{x}")).collect();
        debug!("{}", tmpv.join(""));
    }
}

fn adjacent(a: &Coord<i64>, b: &Coord<i64>) -> bool {
    let d = a.clone() - b.clone();
    if d.x == 0 && 1 == i64::abs(d.y) {
        debug!("ADJ: b/c x=equal: a = {a:?}, b = {b:?}");
        return true;
    }
    if d.y == 0 && 1 == i64::abs(d.x) {
        debug!("ADJ: b/c y=equal: a = {a:?}, b = {b:?}");
        return true;
    }

    false
}

fn merge(region_vec: &mut Vec<Vec<Coord<i64>>>) {
    'outer: loop {
        let num_regions = region_vec.len();
        debug!("MERGE: outer: attempt on: num_regions = {num_regions}");
        let pair_combos: Vec<Vec<usize>> = (0..num_regions).combinations(2).collect();
        // debug!(">>>>>>>>>>>>>>>>>>> pair_combos = {pair_combos:?}");
        for (ixix, rv_ix_pair) in pair_combos.iter().enumerate() {
            debug!("MERGE: mid: pair_combo {ixix} = {rv_ix_pair:?}");
            let a = rv_ix_pair[0];
            let b = rv_ix_pair[1];
            let mut do_merge: Option<(usize, usize)> = None;
            'test_pair: for ac in &region_vec[a].clone() {
                for bc in &region_vec[b].clone() {
                    trace!("MERGE: inner: attempt on: ac={ac:?}, bc={bc:?}");
                    if adjacent(&ac, &bc) {
                        do_merge = Some((a, b));
                        debug!("MERGE: MARKING MERGE! : {a} <-> {b}");
                        break 'test_pair;
                    }
                }
            }

            // NOTE: END: 'pair: for
            if let Some((a, b)) = do_merge {
                debug!("MERGE: PERFORMING MERGE! : {a} <-> {b}");
                let tmp = region_vec.remove(b);
                region_vec[a].extend(tmp);
                continue 'outer;
            }
        }

        // NOTE: if we get here, then we looked at all pairs with no merges; all done
        break;
    } // NOTE: END: 'outer: loop
}

fn find_regions(m: &Vec<Vec<char>>) -> Vec<(char, Vec<Vec<Coord<i64>>>)> {
    assert!(m.len() > 0);
    let (nr, nc) = (m.len(), m[0].len());
    let mut ungrouped: HashMap<char, Vec<Coord<i64>>> = HashMap::default();
    for r in 0..nr {
        for c in 0..nc {
            let k = m[r][c];
            if !ungrouped.contains_key(&k) {
                ungrouped.insert(k, vec![]);
            }
            ungrouped.get_mut(&k).unwrap().push(Coord {
                x: c as i64,
                y: r as i64,
            });
        }
    }

    let mut rvv: Vec<(char, Vec<Vec<Coord<i64>>>)> = vec![];
    for ch in ungrouped.keys() {
        let mut ch_line_strings: Vec<Vec<Coord<i64>>> = vec![];

        'ug_placement: for ug in &ungrouped[ch] {
            // NOTE: try to place location in existing grouping
            for ls in &mut ch_line_strings {
                for in_chk in &mut *ls {
                    if adjacent(ug, in_chk) {
                        ls.push(ug.clone());
                        continue 'ug_placement;
                    }
                }
            }

            // if we get here, add a new line-string

            // NOTE: if not placed in a previous grouping, make a new grouping
            ch_line_strings.push(vec![ug.clone()]);
        }

        rvv.push((*ch, ch_line_strings.clone()));
    }

    for (ch, ch_rvv) in &mut rvv {
        let tmplen = ch_rvv.len();
        debug_print_char_map(&m);
        merge(ch_rvv);
        info!(
            "Merged {ch} : {} regions into {} regions",
            tmplen,
            ch_rvv.len()
        );
    }

    rvv // region-vec-of-vecs
} // NOTE: end find_regions(m: &Vec<Vec<char>>) -> Vec<(char, Vec<Vec<Coord<i64>>>)>

fn main() {
    env_logger::init();
    let args = Args::parse();
    info!("args.input = {}", args.input);
    let m = parse_char_map(args.input);
    debug_print_char_map(&m);
    let (nr, nc) = (m.len(), m[0].len());

    let dv: Vec<Coord<i64>> = vec![
        Coord { x: 0, y: 1 },
        Coord { x: 0, y: -1 },
        Coord { x: 1, y: 0 },
        Coord { x: -1, y: 0 },
    ];

    let all_regions = find_regions(&m);

    // score regions
    let mut sum1 = 0;
    for (region_char, rv) in all_regions {
        // NOTE: start region loop
        for (_rix, r) in rv.iter().enumerate() {
            // debug!("ch [{rix}]= {ch}");
            // let mut edges: HashSet<Coord<i64>> = HashSet::default();
            let mut edge_count = 0;
            for coord in r {
                debug!("");
                debug!("coord = {coord:?}");
                for d in dv.clone() {
                    let chk = coord.clone() + d.clone();
                    if chk.y < 0 || nr as i64 <= chk.y {
                        trace!("implicit edge = {chk:?}, (nr, nc) = ({nr}, {nc})");
                        edge_count += 1;
                        continue;
                    } else if chk.x < 0 || nc as i64 <= chk.x {
                        trace!("implicit edge = {chk:?}, (nr, nc) = ({nr}, {nc})");
                        edge_count += 1;
                        continue;
                    } else {
                        let check_char = m[chk.y as usize][chk.x as usize];
                        if region_char != check_char {
                            // edges.insert(chk);
                            debug!("!!! edge b/w {coord:?} and {chk:?}");
                            debug_print_char_map(&m);
                            edge_count += 1;
                        }
                    }
                }
            }
            let area = r.len();
            let perim = edge_count;
            let price = area * perim;
            info!("region_char={region_char} : area = {area}, perim = {perim}, price = {price}");
            sum1 += price;
        } // NOTE: end region loop
    } // NOTE: end for (ch, rv) in all_regions

    println!("day12, part1 = {sum1}");
}
