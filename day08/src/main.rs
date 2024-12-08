use clap::Parser;
use geo::geometry::{Coord, Point, Polygon};
use geo::polygon;
use geo::Contains;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

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

#[derive(Clone, Debug)]
struct Map {
    map: Vec<Vec<char>>,
    locs: HashMap<char, HashSet<Coord<i64>>>,
}

fn parse_map(input: String) -> Map {
    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());

    let mut locs: HashMap<char, HashSet<Coord<i64>>> = HashMap::default();
    let mut map: Vec<Vec<char>> = vec![];
    for (row, line) in contents.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != '.' {
                let _coord = Coord::<i64> {
                    x: col as i64,
                    y: row as i64,
                };
                // TODO: use a set instead?
                locs.entry(ch).or_insert(HashSet::default()).insert(_coord);
            }
        }
        map.push(line.chars().collect());
    }

    Map { map, locs }
}

fn anode_locs(boundn: i64, p: Coord<i64>, q: Coord<i64>) -> Vec<Coord<i64>> {
    // NOTE: use i64 to handle overflow here
    // NOTE: usize is very convenient everywhere else for indexing
    let z1: Coord<i64> = p + p - q;
    let z2: Coord<i64> = q + q - p;

    let mut out: Vec<Coord<i64>> = vec![];
    for _z in [z1, z2] {
        if _z.x < 0 || boundn <= _z.x || _z.y < 0 || boundn <= _z.y {
            continue;
        }
        out.push(_z);
    }

    out
}

fn main() {
    let args = Args::parse();
    let m: Map = parse_map(args.input);
    let mut anodes: HashSet<Coord<i64>> = HashSet::default();
    // NOTE: for the bounds to 'contain' our points, our bounds polygon
    //       must span from the span including -1 to including LEN
    let nrows = m.map.len() as i64;
    let ncols = m.map[0].len() as i64;
    println!("nrows = {nrows}");
    println!("ncols = {ncols}");
    assert!(nrows == ncols);

    let bounds: Polygon<i64> = polygon![
        (x: -1_i64, y: -1_i64), // top-left
        (x: ncols , y: -1_i64), // top-right
        (x: ncols , y: nrows), // bottom-right
        (x: -1_i64, y: nrows), // bottom-left
    ];
    // let msz = (m.map.len(), m.map[0].len());
    for k in m.locs.keys() {
        let vset = m.locs.get(&k).unwrap().clone();
        for pair in vset.iter().combinations(2) {
            println!("{k} : pair = {pair:?}");
            for anode in anode_locs(nrows, pair[0].clone(), pair[1].clone()) {
                anodes.insert(anode);
            }
        }
    }

    let mut check: Vec<Coord<i64>> = anodes.clone().iter().map(|a| *a).collect();
    // check.sort_by(|);
    check.sort_by(|a, b| {
        if Ordering::Equal == a.x.cmp(&b.x) {
            return a.y.cmp(&b.y);
        } else {
            return a.x.cmp(&b.x);
        }
    });
    for tmp in check {
        println!("({}, {})", tmp.x, tmp.y);
    }

    // println!("check = {check:#?}");
    // for an in &anodes {
    //     println!("an = {an:?}");
    // }

    println!("day08, part1 = {}", anodes.len());
}
