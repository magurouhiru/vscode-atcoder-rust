use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        r: isize,
        c: isize,
        s: Chars,
    }

    let mut takahasi = (r, c);
    let mut hi = (0, 0);
    let mut kemuri: HashSet<(isize, isize)> = HashSet::new();
    let mut result = vec!['0'; n];
    for (i, ss) in s.iter().enumerate() {
        kemuri.insert(hi);
        match ss {
            'N' => {
                takahasi.0 += 1;
                hi.0 += 1;
            }
            'S' => {
                takahasi.0 -= 1;
                hi.0 -= 1;
            }
            'E' => {
                takahasi.1 -= 1;
                hi.1 -= 1;
            }
            'W' => {
                takahasi.1 += 1;
                hi.1 += 1;
            }
            _ => {}
        }

        if kemuri.contains(&takahasi) {
            result[i] = '1';
        }
    }

    let result: String = result.iter().collect();
    println!("{}", result);
}
