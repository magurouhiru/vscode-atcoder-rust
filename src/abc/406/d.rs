use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        xyi: [(usize, usize); n],
        q: usize,
        qi: [(usize, usize); q],
    }

    let mut x_map = HashMap::new();
    let mut y_map = HashMap::new();

    for (x, y) in xyi {
        x_map.entry(x).or_insert(HashSet::new()).insert(y);
        y_map.entry(y).or_insert(HashSet::new()).insert(x);
    }

    let mut x_rm = HashSet::new();
    let mut y_rm = HashSet::new();
    let mut cnt = vec![0; q];
    for (i, (qq, z)) in qi.iter().enumerate() {
        match qq {
            1 => {
                if !x_rm.contains(z) {
                    x_rm.insert(*z);
                    if let Some(s) = x_map.get_mut(z) {
                        cnt[i] = s.len();
                        for y in s.iter() {
                            if let Some(set) = y_map.get_mut(y) {
                                set.remove(z);
                            }
                        }
                        s.clear();
                    }
                }
            }
            2 => {
                if !y_rm.contains(z) {
                    y_rm.insert(*z);
                    if let Some(s) = y_map.get_mut(z) {
                        cnt[i] = s.len();
                        for x in s.iter() {
                            if let Some(set) = x_map.get_mut(x) {
                                set.remove(z);
                            }
                        }
                        s.clear();
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", cnt.iter().join("\n"));
}
