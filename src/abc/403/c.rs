use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        _n: usize,
        _m: usize,
        q: usize,
    }

    let mut map = HashMap::new();
    let mut ans = vec![];
    for _ in 0..q {
        input! {
            q1: usize,
            q2: usize,
        }
        match q1 {
            1 => {
                input! {
                    q3: usize,
                }
                match map.get_mut(&q2) {
                    None => {
                        let mut set = HashSet::new();
                        set.insert(q3);
                        map.insert(q2, set);
                    }
                    Some(x) => {
                        x.insert(q3);
                    }
                };
            }
            2 => {
                match map.get_mut(&q2) {
                    None => {
                        let mut set = HashSet::new();
                        set.insert(usize::MAX);
                        map.insert(q2, set);
                    }
                    Some(x) => {
                        x.insert(usize::MAX);
                    }
                };
            }
            3 => {
                input! {
                    q3: usize,
                }
                if let Some(v) = map.get(&q2) {
                    if v.contains(&usize::MAX) || v.contains(&q3) {
                        ans.push("Yes");
                    } else {
                        ans.push("No");
                    }
                } else {
                    ans.push("No");
                }
            }
            _ => unreachable!(),
        }
    }
    let ans = ans.join("\n");
    println!("{}", ans);
}
