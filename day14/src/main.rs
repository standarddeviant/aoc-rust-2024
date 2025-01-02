use num::complex::Complex;
use regex::Regex;

fn parse_input(input: String) -> Vec<(Complex<i64>, Complex<i64>)> {
    let mut out: Vec<(Complex<i64>, Complex<i64>)> = vec![];
    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());

    // p=0,4 v=3,-3

    let re = Regex::new(r".*p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
    for line in contents.lines() {
        // println!("line = {}", line);
        if let Some(caps) = re.captures(line) {
            let px = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let py = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let vx = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let vy = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
            out.push((Complex { re: px, im: py }, Complex { re: vx, im: vy }));
        }
    }

    out
}

const XSIZE: i64 = 101;
const YSIZE: i64 = 103;

const XPRAC: i64 = 11;
const YPRAC: i64 = 7;

fn main() {
    // let pv_vec = parse_input("practice.txt".into());
    // let (X, Y) = (XPRAC, YPRAC);

    let pv_vec = parse_input("input.txt".into());
    let (X, Y) = (XSIZE, YSIZE);

    // NOTE:
    // 1. we should subtract one from divide-by-2, but...
    // 2. we also need to add 1 because we zero-index, so...
    // 3. they cancel out
    let (XMID, YMID) = (X / 2, Y / 2);

    let nsec: i64 = 100;
    let mut quad_counts = [0, 0, 0, 0];

    for pv in pv_vec {
        let mut pnew = pv.0 + (nsec * pv.1);
        pnew.re = pnew.re.rem_euclid(X);
        pnew.im = pnew.im.rem_euclid(Y);

        if pnew.re == XMID || pnew.im == YMID {
            // println!("SKIPPING!, pv = {:?}, pnew = {}", pv, pnew);
            continue;
        }
        // QUAD 0
        else if pnew.re < XMID && pnew.im < YMID {
            // println!("quad 0");
            quad_counts[0] += 1;
        }
        // QUAD 1
        else if pnew.re > XMID && pnew.im < YMID {
            // println!("quad 1");
            quad_counts[1] += 1;
        }
        // QUAD 2
        else if pnew.re > XMID && pnew.im > YMID {
            // println!("quad 2");
            quad_counts[2] += 1;
        }
        // QUAD 3
        else if pnew.re < XMID && pnew.im > YMID {
            // println!("quad 3");
            quad_counts[3] += 1;
        }
    }

    println!("quad_counts = {quad_counts:?}");
    let ans1: i64 = quad_counts.iter().product();
    println!("day14, part1 = {ans1}");
}
