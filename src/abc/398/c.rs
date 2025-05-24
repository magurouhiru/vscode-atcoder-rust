use std::collections::{BTreeMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: u32,
        si: [usize; n],
    }

    let mut hs = HashSet::new();
    let mut bts = BTreeMap::new();
    for (i, s) in si.iter().enumerate() {
        if hs.insert(s) {
            bts.insert(*s, i);
        } else {
            bts.remove(s);
        }
    }

    match bts.pop_last() {
        None => println!("-1"),
        Some((_, i)) => println!("{}", i + 1),
    }
}
