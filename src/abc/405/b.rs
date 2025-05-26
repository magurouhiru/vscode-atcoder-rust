use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ai: [usize; n],
    }

    let mut cnt = ai.iter().fold(vec![0; m + 1], |mut acc, &a| {
        if a <= m {
            acc[a] += 1;
        }
        acc
    });

    let is_contain_zero = |cnt: &Vec<i32>| -> bool { cnt.iter().skip(1).any(|x| *x == 0) };
    let mut ans = 0;
    for i in (0..n).rev() {
        if is_contain_zero(&cnt) {
            break;
        }
        cnt[ai[i]] -= 1;
        ans += 1;
    }

    println!("{}", ans);
}
