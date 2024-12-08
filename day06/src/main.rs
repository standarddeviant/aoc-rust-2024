use clap::Parser;
use regex::{Captures, Match, Regex, RegexSet};
use std::cmp::{Ordering, PartialOrd};
use std::collections::VecDeque;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::RandomState;

use itertools::Itertools;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
}

#[derive(Copy, Clone, Debug)]
struct Location(usize, usize);

#[derive(Copy, Clone, Debug)]
enum Quadrant {
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight,
}
#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}
struct Day6 {
    map: Vec<Vec<char>>,
    loc: Vec<usize>,
    dir: Direction,
    turns: Vec<Location>,
    blocks: Vec<Location>,
}

fn parse_input(input: String) -> Day6 {
    let dirlu: HashMap<char, Direction, RandomState> =
        HashMap::<char, Direction, RandomState>::from([
            ('>', Direction::Right), // right
            ('v', Direction::Down),  // down
            ('<', Direction::Left),  // left
            ('^', Direction::Up),    // up
        ]);

    // let loc_set = HashSet::from(['>', 'v', '<', '^']);
    let mut map: Vec<Vec<char>> = vec![];
    let mut loc: Vec<usize> = vec![0, 0];
    let mut dir: Direction = Direction::Up;
    // let re = Regex::new(r"\d+").unwrap();

    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());

    for (row, line) in contents.lines().enumerate() {
        let cvec: Vec<char> = line.chars().collect();
        if let Some(loc_col) = line.chars().position(|x| dirlu.contains_key(&x)) {
            let tmp: &Direction = dirlu.get(&cvec[loc_col]).unwrap();
            dir = tmp.clone();
            loc[0] = row;
            loc[1] = loc_col;
        }
        map.push(line.chars().collect());
    }

    Day6 {
        map,
        loc,
        dir,
        turns: vec![],
        blocks: vec![],
    }
}

fn print_d6(d6: &Day6) {
    println!("loc = {:?}, dir = {:?}", d6.loc, d6.dir);
    for line in &d6.map {
        let s: String = String::from_iter(line.clone().iter());
        println!("{}", s);
    }
}

fn mid_tri(tri: Vec<&Location>) -> Option<Vec<&Location>> {
    let mut w = tri.clone(); // work var

    for _rot_ix in 0..3 {
        let chk1_a = w[1].0 == w[0].0 && w[1].1 == w[2].1;
        let chk1_b = w[1].0 == w[2].0 && w[1].1 == w[0].1;
        if chk1_a || chk1_b {
            return Some(w);
        }
        w.rotate_right(1);
    }

    // no common joints
    None
}

fn mid_tri_quad(mt: &Vec<Location>) -> Option<Quadrant> {
    if mt[0].0 == mt[1].0 && mt[1].1 == mt[2].1 {
        // 0,1 same rows
        // 1,2 same cols
    } else if mt[0].1 == mt[1].1 && mt[1].0 == mt[2].0 {
        // 0,1 same cols
        // 1,2 same rows
    }
    None
}

// fn mid_tri_is_loopable(tri: Vec<Location>, blocks: Vec<Location>) {
//
// }

enum Move {
    Cardinal(Vec<usize>),
    Rotate(Vec<usize>, Vec<usize>),
    Done,
}

fn apply_move(mut d6: &mut Day6) -> Move {
    let turn_right: HashMap<char, char, RandomState> = HashMap::<char, char, RandomState>::from([
        ('>', 'v'), // right-to-down
        ('v', '<'), // down-to-left
        ('<', '^'), // left-to-up
        ('^', '>'), // up-to-right
    ]);
    let dirlu: HashMap<char, Direction, RandomState> =
        HashMap::<char, Direction, RandomState>::from([
            ('>', Direction::Right), // right
            ('v', Direction::Down),  // down
            ('<', Direction::Left),  // left
            ('^', Direction::Up),    // up
        ]);
    let nrows = d6.map.len();
    assert!(d6.map.len() > 0);
    let ncols = d6.map[0].len();

    // check if we're at the edge, and get next char if not
    let mut next: char = '~';
    let loc_ch = d6.map[d6.loc[0]][d6.loc[1]];

    let mut next_loc: Vec<usize> = vec![0, 0];
    if '>' == loc_ch {
        if d6.loc[1] >= ncols - 1 {
            return Move::Done; // all done
        }
        next_loc[0] = d6.loc[0];
        next_loc[1] = d6.loc[1] + 1;
    } else if '<' == loc_ch {
        if d6.loc[1] <= 0 {
            return Move::Done; // all done
        }
        next_loc[0] = d6.loc[0];
        next_loc[1] = d6.loc[1] - 1;
    }
    if 'v' == loc_ch {
        if d6.loc[0] >= nrows - 1 {
            return Move::Done; // all done
        }
        next_loc[0] = d6.loc[0] + 1;
        next_loc[1] = d6.loc[1];
    } else if '^' == loc_ch {
        if d6.loc[0] <= 0 {
            return Move::Done; // all done
        }
        next_loc[0] = d6.loc[0] - 1;
        next_loc[1] = d6.loc[1];
    }

    let next = d6.map[next_loc[0]][next_loc[1]];
    // check if we should rotate and count that as a separate, distinct move
    if '#' == next {
        // apply rotation here
        let new_loc_ch = turn_right.get(&loc_ch).unwrap();
        d6.map[d6.loc[0]][d6.loc[1]] = new_loc_ch.clone();
        return Move::Rotate(d6.loc.clone(), next_loc.clone());
    }

    // proceed knowing that...
    // 1. we are not at the edge
    // 1. the next char is not '#'
    assert!(next == '.');

    // swap next and loc_ch to 'move'
    match loc_ch {
        '>' => {
            d6.map[d6.loc[0]][d6.loc[1]] = next;
            d6.map[d6.loc[0]][d6.loc[1] + 1] = loc_ch;
            d6.loc[1] += 1;
        }
        '<' => {
            d6.map[d6.loc[0]][d6.loc[1]] = next;
            d6.map[d6.loc[0]][d6.loc[1] - 1] = loc_ch;
            d6.loc[1] -= 1;
        }
        'v' => {
            d6.map[d6.loc[0]][d6.loc[1]] = next;
            d6.map[d6.loc[0] + 1][d6.loc[1]] = loc_ch;
            d6.loc[0] += 1;
        }
        '^' => {
            d6.map[d6.loc[0]][d6.loc[1]] = next;
            d6.map[d6.loc[0] - 1][d6.loc[1]] = loc_ch;
            d6.loc[0] -= 1;
        }
        _ => {
            eprintln!("THIS SHOULD NOT HAPPEN!");
            return Move::Done;
        } // should be an error...
    }

    return Move::Cardinal(d6.loc.clone());
}

// enum MoveResult {
//
// }

// fn can_move(&mut m: &Vec<Vec<char>>)

fn main() {
    let args = Args::parse();
    println!("args = {args:#?}");

    let mut d6 = parse_input(args.input);

    let mut set1 = HashSet::from([(d6.loc[0], d6.loc[1])]);
    // let mut blocks: Vec<(usize, usize)> = vec![];
    // let mut rots: Vec<(usize, usize)> = vec![];
    let mut p1: Vec<(usize, usize)> = vec![(d6.loc[0], d6.loc[1])];
    loop {
        // print_d6(&d6);
        match apply_move(&mut d6) {
            Move::Cardinal(new_loc) => {
                assert!(new_loc.len() == 2);
                set1.insert((new_loc[0], new_loc[1]));
                p1.push((new_loc[0], new_loc[1]));
            }
            Move::Rotate(rot, block) => {
                // blocks.insert((block[0], block[1]));
                d6.turns.push(Location(rot[0], rot[1]));
                d6.blocks.push(Location(block[0], block[1]));
            }
            Move::Done => {
                break;
            }
        }
    }
    println!("day06, part1 = {}", set1.len());
    // println!("day06, part1 = {???}");

    // day2, examine the 'turns' and 'blocks' ...
    let mut sum2 = 0;
    let blocks: Vec<Location> = d6.blocks.clone();
    // into_iter().collect();
    let turns = d6.turns.clone();
    let mut tri_common: Vec<Vec<Location>> = vec![];
    for tri in turns.clone().iter().combinations(3) {
        if let Some(mt) = mid_tri(tri.clone()) {
            println!("common joint found: {mt:?}");
            tri_common.push(mt.iter().map(|x| *x.clone()).collect());
        } else {
            println!("NO common joint of {tri:?}");
        }
    }

    for mt in tri_common {
        match mid_tri_quad(mt) {
            Quadrant::TopLeft => {}
            Quadrant::TopRight => {}
            Quadrant::BottomLeft => {}
            Quadrant::BottomRight => {}
        }
        // if tri_is_loopable(j, tri, blocks) {}
        println!("looking at blocks around this triple...");
        println!("mt = {mt:?}");
    }
}

// EOF
