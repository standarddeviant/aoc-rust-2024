#[derive(Debug)]
enum Parsing {
    Lock,
    Key,
}

fn parse_lock_or_key(lk: String) -> (Parsing, Vec<usize>) {
    let lk = lk.trim();
    let which = if lk.starts_with("#") {
        Parsing::Lock
    } else {
        Parsing::Key
    };

    let mut out: Vec<usize> = vec![];
    let tmblrs: Vec<String> = lk.split("\n").map(|x| x.to_string()).collect();

    match which {
        Parsing::Lock => {
            for col in 0..tmblrs[0].len() {
                let mut tval: usize = tmblrs.len() - 1;
                for row in 0..tmblrs.len() {
                    if '.' == tmblrs[row].chars().nth(col).unwrap() {
                        tval = row - 1;
                        break;
                    }
                }
                out.push(tval);
            }
        }
        Parsing::Key => {
            for col in 0..tmblrs[0].len() {
                let mut tval: usize = tmblrs.len() - 1;
                for row in (0..tmblrs.len()).rev() {
                    if '.' == tmblrs[row].chars().nth(col).unwrap() {
                        tval = tmblrs.len() - row - 2;
                        break;
                    }
                }
                out.push(tval);
            }
        }
    }

    (which, out)
}

fn parse_input(input: String) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut locks: Vec<Vec<usize>> = vec![];
    let mut keys: Vec<Vec<usize>> = vec![];

    let mut w: Vec<String> = vec![];
    let contents = std::fs::read_to_string(input.clone())
        .expect(format!("Unable to read file ->{}<-", input.clone()).as_str());

    let locks_and_keys = contents.split("\n\n");
    for (ix, lk) in locks_and_keys.enumerate() {
        // println!("\n{ix}\n{lk}");
        let (which, tmp) = parse_lock_or_key(lk.into());
        // println!("which = {which:?}, tmp = {tmp:?}");
        match which {
            Parsing::Lock => locks.push(tmp),
            Parsing::Key => keys.push(tmp),
        }
    }

    (locks, keys)
}

fn lock_key_match(lock: &Vec<usize>, key: &Vec<usize>) -> bool {
    // println!("lock = {lock:?}");
    // println!("key = {key:?}");
    for ix in 0..lock.len() {
        // println!("  ix = {}, sum = {}", ix, lock[ix] + key[ix]);
        if lock[ix] + key[ix] > 5 {
            return false;
        }
    }
    return true;
}

fn main() {
    let (locks, keys) = parse_input("input.txt".into());
    // println!("locks = {locks:?}");
    // println!("keys = {keys:?}");

    let mut ans1: usize = 0;
    for lock in &locks {
        for key in &keys {
            if lock_key_match(lock, key) {
                ans1 += 1;
            }
        }
    }
    println!("day25, part1 = {ans1}");
}
