use clap::Parser;
use regex::{Captures, Match, Regex, RegexSet};
use std::cmp::{Ordering, PartialOrd};
use std::collections::VecDeque;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::hash::RandomState;

use geo::geometry::{Coord, LineString};
use itertools::Itertools;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    input: String,
}

// #[derive(Copy, Clone, Debug)]
// struct Coord(usize, usize);

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
    loc: Coord<usize>,
    dir: Direction,
    turns: Vec<Coord<usize>>,
    blocks: Vec<Coord<usize>>,
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
    let mut loc: Coord<usize> = Coord::<usize>::zero();
    let mut dir: Direction = Direction::Up;
    // let re = Regex::new(r"\d+").unwrap();

    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());

    for (row, line) in contents.lines().enumerate() {
        let cvec: Vec<char> = line.chars().collect();
        if let Some(loc_col) = line.chars().position(|x| dirlu.contains_key(&x)) {
            let tmp: &Direction = dirlu.get(&cvec[loc_col]).unwrap();
            dir = tmp.clone();
            loc.x = row;
            loc.y = loc_col;
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

fn mid_tri(tri: Vec<&Coord<usize>>) -> Option<Vec<&Coord<usize>>> {
    let mut w = tri.clone(); // work var

    for _rot_ix in 0..3 {
        let chk1_a = w[1].x == w[0].x && w[1].y == w[2].y;
        let chk1_b = w[1].x == w[2].x && w[1].y == w[0].y;
        if chk1_a || chk1_b {
            return Some(w);
        }
        w.rotate_right(1);
    }

    // no common joints
    None
}

fn mid_tri_quad(mt: &Vec<Coord<usize>>) -> Option<Quadrant> {
    if mt[0].x == mt[1].x && mt[1].y == mt[2].y {
        // 0,1 same rows
        // 1,2 same cols
    } else if mt[0].y == mt[1].y && mt[1].x == mt[2].x {
        // 0,1 same cols
        // 1,2 same rows
    }
    None
}

// fn mid_tri_is_loopable(tri: Vec<Coord>, blocks: Vec<Coord>) {
//
// }

enum Move {
    Cardinal(Coord<usize>),
    Rotate(Coord<usize>, Coord<usize>, Direction),
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
    let loc_ch = d6.map[d6.loc.x][d6.loc.y];

    let mut next_loc: Coord<usize> = Coord::<usize> { x: 0, y: 0 };
    if '>' == loc_ch {
        if d6.loc.y as usize >= ncols - 1 {
            return Move::Done; // all done
        }
        next_loc.x = d6.loc.x;
        next_loc.y = d6.loc.y + 1;
    } else if '<' == loc_ch {
        if d6.loc.y <= 0 {
            return Move::Done; // all done
        }
        next_loc.x = d6.loc.x;
        next_loc.y = d6.loc.y - 1;
    }
    if 'v' == loc_ch {
        if d6.loc.x as usize >= nrows - 1 {
            return Move::Done; // all done
        }
        next_loc.x = d6.loc.x + 1;
        next_loc.y = d6.loc.y;
    } else if '^' == loc_ch {
        if d6.loc.x <= 0 {
            return Move::Done; // all done
        }
        next_loc.x = d6.loc.x - 1;
        next_loc.y = d6.loc.y;
    }

    let next = d6.map[next_loc.x][next_loc.y];
    // check if we should rotate and count that as a separate, distinct move
    if '#' == next {
        // apply rotation here
        let new_loc_ch = turn_right.get(&loc_ch).unwrap();
        d6.map[d6.loc.x as usize][d6.loc.y as usize] = new_loc_ch.clone();
        return Move::Rotate(
            d6.loc.clone(),
            next_loc.clone(),
            dirlu.get(&loc_ch).unwrap().clone(),
        );
    }

    // proceed knowing that...
    // 1. we are not at the edge
    // 1. the next char is not '#'
    assert!(next == '.');

    // swap next and loc_ch to 'move'
    match loc_ch {
        '>' => {
            d6.map[d6.loc.x as usize][d6.loc.y as usize] = next;
            d6.map[d6.loc.x as usize][d6.loc.y as usize + 1] = loc_ch;
            d6.loc.y += 1;
        }
        '<' => {
            d6.map[d6.loc.x as usize][d6.loc.y as usize] = next;
            d6.map[d6.loc.x as usize][d6.loc.y as usize - 1] = loc_ch;
            d6.loc.y -= 1;
        }
        'v' => {
            d6.map[d6.loc.x as usize][d6.loc.y as usize] = next;
            d6.map[d6.loc.x as usize + 1][d6.loc.y as usize] = loc_ch;
            d6.loc.x += 1;
        }
        '^' => {
            d6.map[d6.loc.x as usize][d6.loc.y as usize] = next;
            d6.map[d6.loc.x as usize - 1][d6.loc.y as usize] = loc_ch;
            d6.loc.x -= 1;
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
    let mut lines: Vec<(Coord<usize>, Coord<usize>, Direction)> = vec![];

    let mut set1 = HashSet::from([(d6.loc.x as usize, d6.loc.y as usize)]);
    // let mut blocks: Vec<(usize, usize)> = vec![];
    // let mut rots: Vec<(usize, usize)> = vec![];
    let mut p1: Vec<Coord<usize>> = vec![d6.loc.clone()];

    let mut prev_turn: Coord<usize> = d6.loc;
    loop {
        // print_d6(&d6);
        match apply_move(&mut d6) {
            Move::Cardinal(new_loc) => {
                set1.insert((new_loc.x as usize, new_loc.y as usize));
                p1.push(new_loc);
            }
            Move::Rotate(rot, _block, dir) => {
                // blocks.insert((block[0], block[1]));
                d6.turns.push(rot);
                lines.push((prev_turn, rot, dir));
                prev_turn = rot;
                // d6.blocks.push(Coord(block[0], block[1]));
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
    let blocks: Vec<Coord<usize>> = d6.blocks.clone();
    // into_iter().collect();
    let turns = d6.turns.clone();
    let mut tri_common: Vec<Vec<Coord<usize>>> = vec![];
    for tri in turns.clone().iter().combinations(3) {
        if let Some(mt) = mid_tri(tri.clone()) {
            println!("common joint found: {mt:?}");
            tri_common.push(mt.iter().map(|x| *x.clone()).collect());
        } else {
            println!("NO common joint of {tri:?}");
        }
    }

    for mt in tri_common {
        let block_checks: Vec<Coord<usize>> = match mid_tri_quad(&mt) {
            Some(Quadrant::TopLeft) => {
                //  *---*#
                //  #
                //  *---*#
                //  ?   |
                //  ?   |
                // O?---*
                //      #
                vec![
                    Coord::<usize> {
                        x: mt[0].x - 1,
                        y: mt[0].y,
                    },
                    Coord::<usize> {
                        x: mt[1].x,
                        y: mt[1].y + 1,
                    },
                    Coord::<usize> {
                        x: mt[2].x + 1,
                        y: mt[2].y,
                    },
                ]
            }
            Some(Quadrant::TopRight) => {
                //
                //  *---*#
                //  #
                //  *---*#
                //  ?   |
                //  ?   |
                // O?---*
                //      #
                vec![
                    Coord::<usize> {
                        x: mt[0].x - 1,
                        y: mt[0].y,
                    },
                    Coord::<usize> {
                        x: mt[1].x,
                        y: mt[1].y + 1,
                    },
                    Coord::<usize> {
                        x: mt[2].x + 1,
                        y: mt[2].y,
                    },
                ]
            }
            Some(Quadrant::BottomLeft) => {
                //  #
                //  *---*#
                //  ?   |
                //  ?   |
                // O?---*
                //      #
                vec![
                    Coord::<usize> {
                        x: mt[0].x - 1,
                        y: mt[0].y,
                    },
                    Coord::<usize> {
                        x: mt[1].x,
                        y: mt[1].y + 1,
                    },
                    Coord::<usize> {
                        x: mt[2].x + 1,
                        y: mt[2].y,
                    },
                ]
            }
            Some(Quadrant::BottomRight) => {
                //  *---*#
                //  O
                //  *???*#
                //  ?   |
                //  |   |
                // #*---M
                //      #
                vec![
                    Coord::<usize> {
                        x: mt[0].x - 1,
                        y: mt[0].y,
                    },
                    Coord::<usize> {
                        x: mt[1].x,
                        y: mt[1].y + 1,
                    },
                    Coord::<usize> {
                        x: mt[2].x + 1,
                        y: mt[2].y,
                    },
                ]
            }
            _ => vec![],
        };

        // block_checks
        println!("block_checks = {block_checks:?}");

        // if tri_is_loopable(j, tri, blocks) {}
        println!("looking at blocks around this triple...");
        println!("mt = {mt:?}");
    }
}

// EOF
