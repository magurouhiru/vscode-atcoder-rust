use proconio::marker::*;
use proconio::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        q: usize,
        mut a: [usize; k],
        l: [Usize1; q],
    }
    for lv in l {
        if a[lv] == n {
            continue;
        } else if lv + 1 == a.len() {
            a[lv] += 1;
        } else if a[lv] + 1 == a[lv + 1] {
            continue;
        } else {
            a[lv] += 1;
        }
    }
    let mut res: String = a[0].to_string();
    for i in 1..a.len() {
        res.push_str(" ");
        res.push_str(&(a[i].to_string()));
    }
    println!("{}", res);
}
