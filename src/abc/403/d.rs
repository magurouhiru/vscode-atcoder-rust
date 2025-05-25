use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        ai: [usize; n],
    }

    let max = 1_000_001;
    let mut cnt = vec![0; max];
    let mut kind = HashSet::new();
    for &a in &ai {
        cnt[a] += 1;
        kind.insert(a);
    }

    if d == 0 {
        println!("{}", n - kind.len());
        return;
    }

    let mut ans = 0;
    let index = |i, j| -> usize { i + j * d };
    for i in 0..d {
        let mut j = 0;
        while index(i, j) < max {
            // 連続区間の開始を探す
            if cnt[index(i, j)] == 0 {
                j += 1;
                continue;
            }
            // 連続区間ごとに2色塗り
            let mut odd = 0;
            let mut even = 0;
            let mut is_odd = true;
            while index(i, j) < max && cnt[index(i, j)] > 0 {
                if is_odd {
                    odd += cnt[index(i, j)];
                } else {
                    even += cnt[index(i, j)];
                }
                j += 1;
                is_odd = !is_odd;
            }
            ans += odd.min(even);
        }
    }

    println!("{}", ans);
}
