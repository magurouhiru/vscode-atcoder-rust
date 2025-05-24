use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ai: [usize; n],
    }

    let mut map = HashMap::new();
    for (i, a) in ai.iter().enumerate() {
        match map.get_mut(a) {
            None => {
                map.insert(*a, (i, 0));
            }
            Some(v) => {
                v.1 = i;
            }
        }
    }

    let mut r = vec![0; n];
    let mut tmp = map.len();
    for (i, a) in ai.iter().enumerate() {
        if let Some(v) = map.get(a) {
            if v.1 == 0 {
                r[i] = tmp;
                continue;
            }

            if v.0 == i {
                tmp += 1;
            } else if v.1 == i {
                tmp -= 1;
            }
            r[i] = tmp;
        }
    }

    println!("{}", r.iter().max().unwrap());
}
