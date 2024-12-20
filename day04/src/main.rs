use clap::Parser;
use regex::{Captures, Match, Regex, RegexSet};
use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
}

fn parse_2d_chars(input: String) -> Vec<Vec<char>> {
    let mut out = vec![];
    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());
    for line in contents.lines() {
        let cvec: Vec<char> = line.chars().collect();
        out.push(cvec);
    }

    out
}

// algo:
// 1. treat each char as a starting point
// 2. search 'radially' from that starting point
//    NOTE: our 'radial' options are like a video game controller:
//    up, down, left, right = (4)
//    between-diagonals = (4)
fn count_2d_str(needle: Vec<char>, haystack: Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let ndl = needle;
    let hay = haystack;
    let nrows = hay.len();
    assert!(nrows > 0);
    let ncols = hay[0].len();

    // dr = delta-of-row-ix
    // dc = delta-of-col-ix
    let dr_dc_vec: [(i32, i32); 8] = [
        (0, 1),   // right
        (1, 1),   // up-right
        (1, 0),   // up
        (1, -1),  // up-left
        (0, -1),  // left
        (-1, -1), // dn-left
        (-1, 0),  // dn
        (-1, 1),  // dn-right
    ];

    for rix in 0..nrows {
        for cix in 0..ncols {
            for (dr, dc) in dr_dc_vec {
                let mut x: Vec<char> = vec!['\0'; ndl.len()];
                let (mut _r, mut _c) = (rix as i32, cix as i32);
                for _ix in 0..ndl.len() {
                    if (0 <= _r && _r < nrows as i32) && (0 <= _c && _c < ncols as i32) {
                        x[_ix] = hay[_r as usize][_c as usize]
                    } else {
                        break;
                    }
                    _r += dr;
                    _c += dc;
                }
                if ndl.eq(&x) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_2d_x_mas(haystack: Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let ndl = vec!['M', 'A', 'S'];
    let hay = haystack;
    let nrows = hay.len();
    assert!(nrows > 0);
    let ncols = hay[0].len();

    for rix in 0..nrows {
        for cix in 0..ncols {
            if 'A' != hay[rix][cix] {
                continue;
            }
            let check_rc = (0 < rix && rix < nrows - 1) && (0 < cix && cix < ncols - 1);
            if !check_rc {
                continue;
            }

            // the string-reversing is redundant since we know A is in the middle, but it's
            // functional. A more optimal approach would be to hide messy if/else logic in a
            // helper function
            let check_dr_1 = vec![hay[rix - 1][cix - 1], hay[rix][cix], hay[rix + 1][cix + 1]];
            let mut check_dr_2 = check_dr_1.clone();
            check_dr_2.reverse();

            let check_ur_1 = vec![hay[rix + 1][cix - 1], hay[rix][cix], hay[rix - 1][cix + 1]];
            let mut check_ur_2 = check_ur_1.clone();
            check_ur_2.reverse();

            let check_ur = ndl.eq(&check_ur_1) || ndl.eq(&check_ur_2);
            let check_dr = ndl.eq(&check_dr_1) || ndl.eq(&check_dr_2);

            if check_dr && check_ur {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let args = Args::parse();
    let haystack = parse_2d_chars(args.input.clone());

    // println!("haystack = {haystack:?}");

    let count1 = count_2d_str(vec!['X', 'M', 'A', 'S'], haystack.clone());
    println!("day04, part1 = {count1:?}");

    let count2 = count_2d_x_mas(haystack.clone());
    println!("day04, part2 = {count2:?}");
}

// EOF
