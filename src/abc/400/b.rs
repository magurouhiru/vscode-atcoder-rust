use num::range;
use proconio::input;

fn main() {
    input! {
        n: u32,
        m: u32,
    }

    let max = 1000000000;
    let mut ans = 0;
    for i in range(0, m + 1) {
        match n.checked_pow(i) {
            Some(v) => ans += v,
            None => {
                println!("inf");
                return;
            }
        }
        if ans > max {
            println!("inf");
            return;
        }
    }
    println!("{}", ans);
}
