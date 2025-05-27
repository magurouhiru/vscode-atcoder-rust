use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u32,
        ai: [usize; n],
    }

    let max = (10_i64.pow(k) - 1) as usize;
    let r = ai
        .iter()
        .fold(1, |acc, a| if max / *a < acc { 1 } else { acc * a });

    println!("{}", r);
}
