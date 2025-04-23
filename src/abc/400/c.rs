use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut ans = 0;
    let mut i: u32 = 1;
    let mut j: u64 = 1;
    while let Some(x) = 2u64.checked_pow(i) {
        if x > n {
            break;
        }
        while let Some(y) = j.checked_pow(2) {
            if y <= n / x {
                ans += 1;
                j += 2;
            } else {
                break;
            }
        }
        i += 1;
        j = 1;
    }

    println!("{}", ans);
}
