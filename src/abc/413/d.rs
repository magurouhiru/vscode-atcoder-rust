use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    let mut ans = Vec::with_capacity(t);
    for _ in 0..t {
        input! {
            n: usize,
            a: [isize; n],
        }

        // すべて同じ場合
        if a.iter().skip(1).all(|&x| x == a[0]) {
            ans.push("Yes");
            continue;
        }

        // -1倍の場合
        if a.iter().all(|ai| ai.abs() == a[0].abs()) {
            let p_cnt = a.iter().filter(|&&ai| ai > 0).count();
            let n_cnt = a.iter().filter(|&&ai| ai < 0).count();
            if p_cnt + n_cnt == n && p_cnt.min(n_cnt) == n / 2 {
                ans.push("Yes");
            } else {
                ans.push("No");
            }
            continue;
        }

        let sorted: Vec<&isize> = a
            .iter()
            .sorted_by(|&&a, &&b| a.abs().cmp(&b.abs()))
            .collect();
        let mut possible = true;
        for i in 0..n - 2 {
            if sorted[i] * sorted[i + 2] == sorted[i + 1] * sorted[i + 1] {
                continue;
            } else {
                possible = false;
                break;
            }
        }
        if possible {
            ans.push("Yes");
        } else {
            ans.push("No");
        }
    }
    println!("{}", ans.join("\n"));
}
