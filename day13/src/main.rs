use nalgebra::{Matrix1x2, Matrix2, MatrixXx2, Vector2};
use regex::Regex;
use std::cmp::min;

fn parse_input(input: String) -> Vec<Vec<(i64, i64)>> {
    let mut out = vec![];
    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());

    let machines = contents.split("\n\n");
    for m in machines {
        let mlines: Vec<&str> = m.trim().split("\n").collect();
        // Button B: X+27, Y+71
        let re_ab = Regex::new(r".*X\+(\d+), Y\+(\d+).*").unwrap();
        // Prize: X=18641, Y=10279
        let re_prz = Regex::new(r".*X\=(\d+), Y\=(\d+).*").unwrap();
        // for ix in 0..mlines.len() {
        //     // println!("mlines[{ix}] = {}", mlines[ix]);
        // }
        let acaps = re_ab.captures(mlines[0]).unwrap();
        let bcaps = re_ab.captures(mlines[1]).unwrap();
        let pcaps = re_prz.captures(mlines[2]).unwrap();
        // println!("acaps = {acaps:?}");
        // println!("bcaps = {bcaps:?}");
        // println!("pcaps = {pcaps:?}");

        // let v = caps.get(2).unwrap().as_str();
        let ax = acaps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let ay = acaps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let bx = bcaps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let by = bcaps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let px = pcaps.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let py = pcaps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        out.push(vec![(ax, ay), (bx, by), (px, py)]);
    }

    out
}

fn min_cost(m: &Vec<(i64, i64)>, part2: bool) -> Option<i64> {
    let a = m[0].clone();
    let b = m[1].clone();
    let mut p = m[2].clone();

    if part2 {
        p.0 += 10000000000000;
        p.1 += 10000000000000;
    }

    let amat = Matrix2::new(a.0 as f64, b.0 as f64, a.1 as f64, b.1 as f64);
    let b = Vector2::new(p.0 as f64, p.1 as f64);
    let decomp = amat.lu();
    let x = decomp.solve(&b);
    if x.is_none() {
        return None;
    }
    let x = x.unwrap();

    let da = x[0].round() - x[0];
    let db = x[1].round() - x[1];
    let eps = 0.001 as f64;
    if da.abs() > eps || db.abs() > eps {
        return None;
    }

    let na = x[0].round() as i64;
    let nb = x[1].round() as i64;
    // println!("na = {na}, nb = {nb}, x = {x:?}");
    return Some(3 * na + nb);
}

fn main() {
    let mut costs: Vec<i64> = vec![];
    let machines = parse_input("input.txt".into());

    for m in &machines {
        if let Some(mc) = min_cost(m, false) {
            // println!("mc = {mc}");
            costs.push(mc);
        }
    }
    let ans1: i64 = costs.iter().sum();
    println!("day13, part1 = {ans1}");

    let mut costs: Vec<i64> = vec![];
    for m in &machines {
        if let Some(mc) = min_cost(m, true) {
            // println!("mc = {mc}");
            costs.push(mc);
        }
    }
    let ans1: i64 = costs.iter().sum();
    println!("day13, part2 = {ans1}");
}
