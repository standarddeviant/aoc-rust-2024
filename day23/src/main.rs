use regex::Regex;
use std::collections::{HashMap, HashSet};

fn parse_connections(input: String) -> HashMap<String, HashSet<String>> {
    let mut out: HashMap<String, HashSet<String>> = HashMap::default();
    let re = Regex::new(r"(..)-(..)").unwrap();

    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());
    for line in contents.lines() {
        let a: String = line[0..2].into();
        let b: String = line[3..5].into();
        // let caps = re.captures(line).unwrap();
        // let a: String = caps.get(0).unwrap().as_str().into();
        // let b: String = caps.get(1).unwrap().as_str().into();
        // println!("a = {a}, b = {b}");
        out.entry(a.clone())
            .or_insert(HashSet::default())
            .insert(b.clone());
        out.entry(b.clone())
            .or_insert(HashSet::default())
            .insert(a.clone());
    }

    out
}

fn main() {
    // let cnx = parse_connections("practice.txt".into());
    let cnx = parse_connections("input.txt".into());
    let mut trips: HashSet<(String, String, String)> = HashSet::default();
    for k1 in cnx.keys() {
        for k2 in &cnx[k1] {
            for k3 in &cnx[k2] {
                if !k1.starts_with("t") && !k2.starts_with("t") && !k3.starts_with("t") {
                    continue;
                }

                // sort potential trip to simplify insertion into hash set
                let mut tmp: Vec<String> = vec![k1.clone(), k2.clone(), k3.clone()];
                tmp.sort();

                // check sorted triple
                // println!("checking {tmp:?}");
                if cnx[k3].contains(k1) {
                    // println!("SUCCESS!");
                    trips.insert((tmp[0].clone(), tmp[1].clone(), tmp[2].clone()));
                }
            }
        }
    }
    // println!("trips = {trips:?}");
    println!("trips.len() = {}", trips.len());
    println!("day23, part1 = {}", trips.len());
}
