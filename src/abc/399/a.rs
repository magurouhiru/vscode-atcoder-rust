use proconio::input;

fn main() {
    input! {
        n: u32,
        s: String,
        t: String,
    }

    let mut s = s.chars();
    let mut t = t.chars();

    let mut cnt = 0;

    for _ in 0..n {
        if s.next().unwrap() != t.next().unwrap() {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
