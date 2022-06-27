use proconio::marker::*;
use proconio::*;
use std::cmp::min;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    input! {
        n: usize,
        s: Chars,
        w: [usize; n],
    }

    let mut buf_set: BTreeSet<usize> = BTreeSet::new();
    let mut c_map: BTreeMap<usize, usize> = BTreeMap::new();
    let mut a_map: BTreeMap<usize, usize> = BTreeMap::new();
    for i in 0..n {
        buf_set.insert(w[i]);
        if s[i] == '0' {
            let v = c_map.get_mut(&w[i]);
            match v {
                None => {
                    c_map.insert(w[i], 1);
                }
                Some(x) => {
                    *x = *x + 1;
                }
            }
        } else {
            let v = a_map.get_mut(&w[i]);
            match v {
                None => {
                    a_map.insert(w[i], 1);
                }
                Some(x) => {
                    *x = *x + 1;
                }
            }
        }
    }
    if c_map.len() == 0 || a_map.len() == 0 {
        println!("{}", n);
        return;
    }
    let mut buf = 0;
    let mut c_sum_map: BTreeMap<usize, usize> = BTreeMap::new();
    for (k, v) in &c_map {
        buf += v;
        c_sum_map.insert(*k, buf);
    }
    let mut buf = 0;
    let mut a_sum_map: BTreeMap<usize, usize> = BTreeMap::new();
    for (k, v) in a_map.iter().rev() {
        buf += v;
        a_sum_map.insert(*k, buf);
    }

    let mut res = 0;
    for v in buf_set {
        let c_rng = c_sum_map.range(..=v);
        let mut a_rng = a_sum_map.range(v..);
        let c_last = c_rng.last();
        let a_next = a_rng.nth(0);
        match c_last {
            None => {
                if res < *a_next.unwrap().1 {
                    res = *a_next.unwrap().1;
                }
            }
            Some(x) => match a_next {
                None => {
                    if res < *c_last.unwrap().1 {
                        res = *c_last.unwrap().1;
                    }
                }
                Some(y) => {
                    if x.0 == y.0 {
                        let tmp =
                            *x.1 + *y.1 - min(*c_map.get(&v).unwrap(), *a_map.get(&v).unwrap());
                        if res < tmp {
                            res = tmp;
                        }
                    } else {
                        if res < *x.1 + *y.1 {
                            res = *x.1 + *y.1;
                        }
                    }
                }
            },
        }
    }
    println!("{}", res);
}
