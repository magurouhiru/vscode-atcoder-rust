use std::collections::HashMap;

use im_rc::HashSet;
use proconio::input;

fn main() {
    input! {
        t: u32,
    }
    for _i in 0..t {
        input! {
            n: u32,
            ai: [usize; 2*n],
        }

        let mut map: HashMap<usize, (Item, Item)> = HashMap::new();
        for (i, a) in ai.iter().enumerate() {
            let pre = if i == 0 { None } else { ai.get(i - 1).copied() };
            let post = ai.get(i + 1).copied();
            let item = Item {
                index: i as isize,
                tonari: (pre, post),
            };
            match map.get_mut(a) {
                None => {
                    let d = Item {
                        index: -100,
                        tonari: (None, None),
                    };
                    map.insert(*a, (item, d));
                }
                Some(v) => {
                    v.1 = item;
                }
            };
        }

        let mut set: HashSet<(usize, usize)> = HashSet::new();
        let is_jouken1 = |k1: &usize| -> bool {
            let op1 = map.get(k1);
            if let Some(v1) = op1 {
                if v1.0.index.abs_diff(v1.1.index) != 1 {
                    return true;
                }
            }
            false
        };
        let is_jouken2 = |k1: &usize, k2: &usize| -> bool {
            let op1 = map.get(k1);
            let op2 = map.get(k2);
            if let (Some(v1), Some(v2)) = (op1, op2) {
                if v1.0.index.abs_diff(v1.1.index) != 1
                    && v2.0.index.abs_diff(v2.1.index) != 1
                    && v1.0.index.abs_diff(v2.0.index) == 1
                    && v1.1.index.abs_diff(v2.1.index) == 1
                {
                    return true;
                }
            }
            false
        };
        let sort = |v1: usize, v2: usize| -> (usize, usize) {
            if v1 < v2 {
                (v1, v2)
            } else {
                (v2, v1)
            }
        };
        for (k, (v1, v2)) in map.iter() {
            if is_jouken1(k) {
                if let Some(v) = v1.tonari.0 {
                    if is_jouken2(k, &v) {
                        set.insert(sort(*k, v));
                    }
                }
                if let Some(v) = v1.tonari.1 {
                    if is_jouken2(k, &v) {
                        set.insert(sort(*k, v));
                    }
                }
                if let Some(v) = v2.tonari.0 {
                    if is_jouken2(k, &v) {
                        set.insert(sort(*k, v));
                    }
                }
                if let Some(v) = v2.tonari.1 {
                    if is_jouken2(k, &v) {
                        set.insert(sort(*k, v));
                    }
                }
            }
        }

        println!("{}", set.len());
    }
}

#[derive(Debug)]
struct Item {
    index: isize,
    tonari: (Option<usize>, Option<usize>),
}
