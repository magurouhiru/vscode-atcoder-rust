use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let indexs: Vec<usize> = s
        .iter()
        .enumerate()
        .filter(|(_, &c)| c.is_ascii_uppercase())
        .map(|(i, _)| i)
        .collect();

    if indexs.len() < 2 {
        println!("Yes");
        return;
    }

    let target = indexs
        .iter()
        .skip(1)
        .map(|&i| s[i - 1])
        .collect::<Vec<char>>();
    let mut target: HashSet<char> = HashSet::from_iter(target);
    for c in t {
        target.remove(&c);
    }
    if target.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
