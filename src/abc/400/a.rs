use proconio::input;

fn main() {
    input! {
        s: u32,
    }

    let n = 400;

    if n % s == 0 {
        println!("{}", n / s);
    } else {
        println!("-1");
    }
}
