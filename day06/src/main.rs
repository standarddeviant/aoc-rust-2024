use clap::Parser;
use regex::{Captures, Match, Regex, RegexSet};
use std::cmp::{Ordering, PartialOrd};
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::hash::RandomState;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
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

    Day6 { map, loc, dir }
}

fn print_d6(d6: &Day6) {
    println!("loc = {:?}, dir = {:?}", d6.loc, d6.dir);
    for line in &d6.map {
        let s: String = String::from_iter(line.clone().iter());
        println!("{}", s);
    }
}

enum Move {
    Cardinal(Vec<usize>),
    Rotate,
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

    if '>' == loc_ch {
        if d6.loc[1] >= ncols - 1 {
            return Move::Done; // all done
        }
        next = d6.map[d6.loc[0]][d6.loc[1] + 1];
    } else if '<' == loc_ch {
        if d6.loc[1] <= 0 {
            return Move::Done; // all done
        }
        next = d6.map[d6.loc[0]][d6.loc[1] - 1];
    }
    if 'v' == loc_ch {
        if d6.loc[0] >= nrows - 1 {
            return Move::Done; // all done
        }
        next = d6.map[d6.loc[0] + 1][d6.loc[1]];
    } else if '^' == loc_ch {
        if d6.loc[0] <= 0 {
            return Move::Done; // all done
        }
        next = d6.map[d6.loc[0] - 1][d6.loc[1]];
    }

    // check if we should rotate and count that as a separate, distinct move
    if '#' == next {
        // apply rotation here
        let new_loc_ch = turn_right.get(&loc_ch).unwrap();
        d6.map[d6.loc[0]][d6.loc[1]] = new_loc_ch.clone();
        return Move::Rotate;
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
    // print_map(&d6.map);
    let mut set1 = HashSet::from([(d6.loc[0], d6.loc[1])]);
    loop {
        // print_d6(&d6);
        match apply_move(&mut d6) {
            Move::Cardinal(new_loc) => {
                assert!(new_loc.len() == 2);
                set1.insert((new_loc[0], new_loc[1]));
            }
            Move::Rotate => {
                // nothing to do
            }
            Move::Done => {
                break;
            }
        }
    }
    println!("day06, part1 = {}", set1.len());
    // println!("day06, part1 = {???}");
}
