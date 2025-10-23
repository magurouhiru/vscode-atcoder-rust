use proconio::input;

fn main() {
    input! {
        n: usize,
        ai: [usize; n],
        k: usize,
    }

    println!("{}", ai.iter().filter(|&&x| x >= k).count());
}
